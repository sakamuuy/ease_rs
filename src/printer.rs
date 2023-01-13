use std::{io::Write, thread, time};
use crate::easing;

pub struct Printer {
    msg: String,
    printed_index: Vec<i32>,
    duration_time: u64 
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
            &self.printed_index.push(index);
        }
    }

    pub fn print(&mut self) {
        let msg_len = &self.msg.len();
        let duration = time::Duration::from_millis(*&self.duration_time);

        for x in 1..=100 {
            // let x = ease_in_out_back(x as f64 / 100.0);

            let i = easing::mapping(*msg_len as f64, 0.0, 1.0, 0.0, x as f64) as i32;
            if let Some(_) = &self.printed_index.iter().find(|v| **v == i) {
                continue;
            }

            if let Some(l) = &self.printed_index.last() {
                for n in **l..i {
                    &self.print_character(n);
                }
                thread::sleep(duration);
                continue;
            }

            let c = &self.msg.chars().nth(i as usize);
            if let Some(c) = c {
                print!("{}", c);
                // Following line allows to print character immediately.
                std::io::stdout().flush().unwrap();
                &self.printed_index.push(i);
            }
            thread::sleep(duration);
        }

        println!("");
    }
}
