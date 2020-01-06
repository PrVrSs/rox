extern crate rox;

use rox::scanner::Scanner;
use rox::parser::Parser;

fn main() {
    let expr = "var a = 1;";
    let tokens = Scanner::new(expr).scan_tokens().unwrap();
    let parser = Parser::new(tokens).parse();
    println!("{:?}", parser);
}
