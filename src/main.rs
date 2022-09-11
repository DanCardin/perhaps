use ansi_term::{Colour, Colour::RGB};
use clap::Parser;
use crossterm::{
    cursor::{Hide, MoveToNextLine, RestorePosition, SavePosition, Show},
    execute,
    style::Print,
};
use glob;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Command, Stdio};
use toml_edit::Document;

pub const GREEN: Colour = RGB(118, 148, 106);
pub const BLUE: Colour = RGB(70, 130, 180);
pub const YELLOW: Colour = RGB(250, 189, 47);
pub const RED: Colour = RGB(251, 73, 52);

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    commands: Vec<TargetName>,
}

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
struct TargetName(String);

impl std::str::FromStr for TargetName {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl TargetName {
    fn git_pattern(&self) -> Option<glob::Pattern> {
        if self.0.starts_with("git:") {
            Some(glob::Pattern::new(self.0.strip_prefix("git:").unwrap()).unwrap())
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Target {
    name: TargetName,
    requires: Vec<TargetName>,
    body: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TargetEvaluation {
    target: Target,
    needs_to_execute: bool,
}

fn main() -> std::io::Result<()> {
    let opts: Args = Args::parse();

    let mut f = File::open("perhaps.toml")?;

    let mut result = parse_file(&mut f);

    let task_evaluations = evaluate_dependencies(&mut result, opts.commands);

    execute_evaluations(&task_evaluations).unwrap();
    Ok(())
}

fn parse_file<'a>(file: &'a mut File) -> HashMap<TargetName, Target> {
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let document = content.parse::<Document>().unwrap();

    document
        .iter()
        .map(|(k, v)| {
            let table = v.as_table().unwrap();
            let deps: Vec<_> = table
                .get("requires")
                .unwrap()
                .as_array()
                .unwrap()
                .iter()
                .map(|v| {
                    let value = v.as_str().unwrap().to_string();
                    TargetName(value)
                })
                .collect();
            let body = table.get("body").unwrap().as_str().unwrap();
            let target_name = TargetName(k.to_string());
            (
                target_name.clone(),
                Target {
                    name: target_name,
                    requires: deps,
                    body: Some(body.to_string()),
                },
            )
        })
        .collect()
}

fn evaluate_dependencies(
    targets: &mut HashMap<TargetName, Target>,
    tasks: Vec<TargetName>,
) -> Vec<TargetEvaluation> {
    let mut result = Vec::new();
    let changed_files = get_changed_files();
    for task in &tasks {
        result.extend(evaluate_task_dependencies(targets, task, &changed_files));
    }
    result
}

fn evaluate_task_dependencies(
    targets: &mut HashMap<TargetName, Target>,
    task: &TargetName,
    changed_files: &[String],
) -> Vec<TargetEvaluation> {
    let mut result = Vec::new();

    if let Some(target) = targets.remove(&task) {
        let mut needs_to_execute = false;
        for dep in &target.requires {
            // dbg!(&dep);

            let dep_target_evals = evaluate_task_dependencies(targets, dep, changed_files);
            if dep_target_evals.iter().any(|te| te.needs_to_execute) {
                needs_to_execute = true;
            }

            result.extend(dep_target_evals);
        }
        let target_eval = TargetEvaluation {
            target,
            needs_to_execute,
        };
        result.push(target_eval);
    } else {
        if let Some(pattern) = task.git_pattern() {
            let target_eval = TargetEvaluation {
                target: Target {
                    name: task.clone(),
                    requires: Vec::new(),
                    body: None,
                },
                needs_to_execute: glob_matches(pattern, &changed_files),
            };
            result.push(target_eval);
        } else {
            unimplemented!("missing {task:?}");
        }
    }
    result
}

fn get_changed_files() -> Vec<String> {
    let output = Command::new("git")
        .args(["diff", "--name-only", "main"])
        .output()
        .expect("sh command failed to start");

    let data: &[u8] = output.stdout.as_ref();
    let reader = BufReader::new(data);
    let lines: Vec<Result<String, _>> = reader.lines().collect();
    let lines: Result<Vec<String>, _> = lines.into_iter().collect();
    lines.unwrap()
}

fn glob_matches<S: AsRef<str>>(pattern: glob::Pattern, values: &[S]) -> bool {
    for value in values {
        if pattern.matches(value.as_ref()) {
            return true;
        }
    }
    return false;
}

fn execute_evaluations(task_evaluations: &[TargetEvaluation]) -> Result<(), std::io::Error> {
    for task_evaluation in task_evaluations {
        let name = &task_evaluation.target.name.0;
        if !task_evaluation.needs_to_execute {
            eprintln!("{}", GREEN.paint(format!("{name}: Fulfilled, skipping")));
            continue;
        }

        let target = &task_evaluation.target;

        let prefix = BLUE.paint(format!("Executing {name}"));
        if let Some(body) = &target.body {
            let mut child = Command::new("zsh")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?;

            let mut stdin = child.stdin.take().unwrap();
            stdin.write_all(body.as_bytes())?;
            std::mem::drop(stdin);

            let stderr = BufReader::new(child.stderr.take().unwrap());

            let mut out = io::stderr();
            execute!(out, Hide, SavePosition).unwrap();
            for line in stderr.lines() {
                let line = line.unwrap();
                execute!(out, Print(format!("{prefix}: {line}")), RestorePosition).unwrap();
            }
            execute!(out, Show, MoveToNextLine(2)).unwrap();

            let child = child.wait_with_output()?;
            if !child.status.success() {
                let status_code = child.status.code().unwrap_or(0);
                println!("Failed {name}: {status_code}");
                break;
                // let err = String::from_utf8_lossy(&child.stderr).to_string();
                // return Err(CorgError::BlockExecutionError(err));
            }
        }
    }
    Ok(())
}
