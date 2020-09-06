extern crate rox;

use rox::scanner::Scanner;
use rox::parser::Parser;
use rox::ast_printer::AstPrinter;


fn main() {
    let expr = "var a = 1;";
    let tokens = Scanner::new(expr).scan_tokens().unwrap();
    let parser = Parser::new(tokens).parse().unwrap();
    let mut printer = AstPrinter{};

    for i in &parser {
        println!("{:?}", printer.print_stmt(i.clone()));
    }

    println!("{:?}", parser);
}
