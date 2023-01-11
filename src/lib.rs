use std::{f64::consts::PI, io::Write, thread, time};

pub fn ease_in_sine(v: f64) -> f64 {
    1.0 - ((v * PI) / 2.0).cos()
}

fn norm(value: f64, upper: f64, bottom: f64) -> f64 {
    (value - bottom) / (upper - bottom)
}

fn lerp(upper: f64, bottom: f64, ratio: f64) -> f64 {
    bottom + (upper - bottom) * ratio
}

fn mapping(to_upper: f64, to_bottom: f64, from_upper: f64, from_bottom: f64, value: f64) -> f64 {
    lerp(to_upper, to_bottom, norm(value, from_upper, from_bottom))
}

fn print_with_easing(msg: &str) {
    let msg_len = msg.len();
    let mut printed_index: Vec<i32> = vec![];
    let duration = time::Duration::from_millis(30);

    for x in 1..=100 {
        let x = ease_in_sine(x as f64 / 100.0);
        let i = mapping(msg_len as f64, 0.0, 1.0, 0.0, x as f64) as i32;
        if let Some(_) = printed_index.iter().find(|v| **v == i) {
            continue;
        }

        let c = msg.chars().nth(i as usize);
        if let Some(c) = c {
            print!("{}", c);
            // Following line allows to print character immediately.
            std::io::stdout().flush().unwrap();
            printed_index.push(i);
        }

        thread::sleep(duration);
    }

    println!("");
}

fn main() {
    let message: &str =
        "aaaaaaaaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeefffffffffgggggggggghhhhhhhhhh";
    print_with_easing(message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ease_in_sine_works() {}

    #[test]
    fn mapping_works() {
        let result = mapping(500.0, 300.0, 200.0, 100.0, 140.0);
        assert_eq!(result, 380.0);
    }

    #[test]
    fn norm_works() {
        let result = norm(0.01, 1.0, 0.0);
        assert_eq!(result, 0.01);
    }
}
