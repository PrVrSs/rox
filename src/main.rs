extern crate rox;

use rox::scanner::Scanner;

fn main() {
    let expr = "1 + 1";

    println!("{:?}", Scanner::new(expr).scan_tokens());
}
