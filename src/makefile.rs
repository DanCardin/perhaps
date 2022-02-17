fn eval_tasks(tasks: Vec<Target>) {
    // let mut git_deps = HashSet::new();

    // let mut output = String::new();
    // for target in targets {
    //     let name = target.name;
    //     output.push_str(&format!(".perhaps/{name}: "));

    //     for dep in target.requires {
    //         match dep {
    //             Dependency::GitCheck(v) => {
    //                 let re = fnmatch_regex::glob_to_regex(&v)
    //                     .unwrap()
    //                     .as_str()
    //                     .strip_prefix("^")
    //                     .unwrap()
    //                     .strip_suffix("$")
    //                     .unwrap()
    //                     .to_string();
    //                 let pattern = regex::escape(&re).replace('/', "__");
    //                 let git_dep = format!(".perhaps/{pattern}");

    //                 git_deps.insert(re);
    //                 output.push_str(&git_dep.replace(':', "\\:"));
    //             }
    //             Dependency::TargetName(v) => {
    //                 output.push_str(&format!(".perhaps/{v}"));
    //             }
    //         }

    //         output.push(' ')
    //     }
    //     let body = target.body;
    //     output.push('\n');
    //     output.push_str(&format!("\t{body}\n"));

    //     let name = name.replace('/', "__");
    //     output.push_str(&format!("\ttouch .perhaps/{name}\n"));
    //     output.push_str("\n");
    // }

    // for re in git_deps.iter() {
    //     let pattern = regex::escape(&re).replace(':', "\\:").replace('/', "__");
    //     let dep = format!(".perhaps/{pattern}");
    //     output.push_str(&format!("{dep}:\n"));
    //     output.push_str(&format!(
    //         "\t@if [ $(git diff --name-only | grep '{re}') ]; then \\\n"
    //     ));
    //     output.push_str(&format!("\t    touch '{dep}'; \\\n"));
    //     output.push_str("\tfi\n");
    //     output.push('\n');
    // }
}

// let mut deps = VecDeque::new();
// deps.extend(tasks);

// while !deps.is_empty() {
//     let dep = deps.pop_front().unwrap();

//     if let Some(target) = targets.remove(&dep) {
//         deps.extend(
//             target
//                 .requires
//                 .clone()
//                 .into_iter()
//                 .filter(|p| {
//                     if let Dependency::TargetName(_) = p {
//                         true
//                     } else {
//                         false
//                     }
//                 })
//                 .clone(),
//         );

//         let git_deps = target
//             .requires
//             .clone()
//             .into_iter()
//             .filter(|p| {
//                 if let Dependency::TargetName(_) = p {
//                     false
//                 } else {
//                     true
//                 }
//             })
//             .clone();

//         result.push(target);

//         for git_dep in git_deps {
//             result.push(Target {
//                 name: git_dep,
//                 requires: Vec::new(),
//                 body: None,
//             });
//         }
//     } else {
//         dbg!(&targets, &dep);
//     }
// }

// let result: Vec<_> = result.into_iter().rev().collect();
