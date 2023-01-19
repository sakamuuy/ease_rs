mod easing;
mod printer;

fn main() {
    let message: &str = "example text message.";
    let mut p = printer::Printer::new(message, 50, easing::EasingKind::EaseInQuad);
    p.print();
}
