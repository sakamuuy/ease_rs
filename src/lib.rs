use std::f64::consts::PI;

pub fn ease_in_sine(v: f64) -> f64 {
    1.0 - (v * PI).cos() / 2.0
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
    let printed_index: Vec<i32> = vec![];

    for x in 1..=100 {
        let i = mapping(msg_len as f64, 0.0, 100.0, 0.0, x as f64) as i32;
        if let Some(_) = printed_index.iter().find(|v| **v == i) {
            println!("found");
            continue;
        }

        let c = msg.chars().nth(i as usize);
        if let Some(c) = c {
            print!("{}", c);
        }
    }

    println!("/n");
}

fn main() {
    let message: &str = "This library will print line with easing";
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
}
