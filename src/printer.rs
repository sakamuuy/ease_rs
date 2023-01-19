use crate::easing::{self, match_easing, EasingKind};
use std::{io::Write, thread, time};

/// # Printer
///
pub struct Printer {
    msg: String,
    printed_index: Vec<i32>,
    duration: time::Duration,
    easing: EasingKind,
}

impl Printer {
    pub fn new(msg: &str, duration_time: u64, easing: easing::EasingKind) -> Printer {
        Printer {
            msg: String::from(msg),
            printed_index: vec![],
            duration: time::Duration::from_millis(duration_time),
            easing,
        }
    }

    fn print_character(&mut self, index: i32) {
        let c = &self.msg.chars().nth(index as usize);
        if let Some(c) = c {
            print!("{}", c);

            // Following line allows to print character immediately.
            std::io::stdout().flush().unwrap();
            let _ = &self.printed_index.push(index);
        }
    }

    pub fn print(&mut self) {
        let msg_len = &self.msg.len();
        let easing = match_easing(*&self.easing);

        for x in 1..=100 {
            let x = easing(x as f64 / 100.0);
            let eased_index = easing::mapping(*msg_len as f64, 0.0, 1.0, 0.0, x) as i32;

            if self
                .printed_index
                .iter()
                .find(|v| **v == eased_index)
                .is_some()
            {
                continue;
            }

            let last_index = &self.printed_index.last();

            match last_index {
                Some(index) => {
                    for i in **index + 1..=eased_index {
                        let _ = &self.print_character(i);
                    }
                }
                // First time only
                None => {
                    let _ = &self.print_character(0);
                }
            }
            thread::sleep(*&self.duration);
        }
        println!();
    }
}
