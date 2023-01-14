mod easing;
mod printer;

fn main() {
    let message: &str = "abcdejghijklmnopqrstuvwxyzABCDEJGHIJKLMNOPQRSTUVWXYZ";
    let mut p = printer::Printer::new(message, 10);
    p.print();
}
