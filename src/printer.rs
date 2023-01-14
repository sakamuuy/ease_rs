use crate::easing;
use std::{io::Write, thread, time};

pub struct Printer {
    msg: String,
    printed_index: Vec<i32>,
    duration_time: u64,
}

impl Printer {
    pub fn new(msg: &str, duration_time: u64) -> Printer {
        Printer {
            msg: String::from(msg),
            printed_index: vec![],
            duration_time,
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
        let duration = time::Duration::from_millis(*&self.duration_time);

        for x in 1..=100 {
            // let x = ease_in_out_back(x as f64 / 100.0);
            let x = x as f64 / 100.0;
            let eased_index = easing::mapping(*msg_len as f64, 0.0, 1.0, 0.0, x) as i32;

            if let Some(_) = &self.printed_index.iter().find(|v| **v == eased_index) {
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
            thread::sleep(duration);
        }
        println!("");
    }
}