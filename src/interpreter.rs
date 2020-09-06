use crate::expr;
use crate::expr::{Expr};
use crate::token::Token;
use crate::error::{Result};


#[derive(Debug, Clone)]
pub enum Object {}


#[derive(Debug, Clone)]
pub struct Interpreter {}


impl Interpreter {
    fn evaluate(&mut self, expr: &Expr) -> Result<Object> {
        expr.accept(self)
    }
}


impl expr::Visitor<Result<Object>> for Interpreter {
    fn visit_assign(&mut self, name: &Token, value: &Expr) -> Result<Object> {
        unimplemented!()
    }

    fn visit_literal(&mut self, name: &Token) -> Result<Object> {
        unimplemented!()
    }
}
