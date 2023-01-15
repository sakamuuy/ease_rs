mod easing;
mod printer;

fn main() {
    let message: &str = "abcdefghijklmnopqrstuvwxyzABCDEJGHIJKLMNOPQRSTUVWXYZ";
    let mut p = printer::Printer::new(message, 100);
    p.print();
}
