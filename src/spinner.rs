use lazy_static::lazy_static;

lazy_static! {
    pub static ref FRAMES: Vec<&str> = vec!["⠋", "⠙", "⠚", "⠞", "⠖", "⠦", "⠴", "⠲", "⠳", "⠓"];
}

use std::{
    io::{stdout, Write},
    sync::mpsc::{channel, Sender, TryRecvError},
    thread,
    time::Duration,
};

mod utils;
pub use crate::utils::spinner_names::SpinnerNames as Spinners;
use crate::utils::spinners_data::SPINNERS as SpinnersMap;

pub struct Spinner {
    sender: Sender<()>,
}

impl Spinner {
    pub fn new(message: String) -> Self {
        let (sender, recv) = channel::<()>();

        thread::spawn(move || 'outer: loop {
            let mut stdout = stdout();
            for frame in FRAMES.iter() {
                match recv.try_recv() {
                    Ok(_) | Err(TryRecvError::Disconnected) => {
                        break 'outer;
                    }
                    Err(TryRecvError::Empty) => {}
                }

                print!("\r{} {}", frame, message);
                stdout.flush().unwrap();
                thread::sleep(Duration::from_millis(spinner_data.interval as u64));
            }
        });

        Spinner { sender }
    }

    pub fn stop(self) {
        self.sender
            .send(())
            .expect("Could not stop spinner thread.");
    }

    pub fn stop_with_message(self, msg: String) {
        self.stop();
        print!("\r{}", msg);
    }
}
