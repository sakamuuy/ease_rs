mod easing;
mod printer;

fn main() {
    let message: &str = "abcdefghijklmnopqrstuvwxyzABCDEJGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzABCDEJGHIJKLMNOPQRSTUVWXYZtext";
    let mut p = printer::Printer::new(message, 50, easing::EasingKind::EaseInQuad);
    p.print();
}
