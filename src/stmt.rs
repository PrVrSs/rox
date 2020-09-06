use crate::token::{Token};
use crate::expr::{Expr};


#[derive(Clone, Debug)]
pub enum Stmt {
    Empty,
    Variable(Token, Option<Expr>),
}


impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Stmt::Variable(name, expr) => visitor.visit_variable(name, expr),
            _ => panic!()
        }
    }
}

pub trait Visitor<T> {
    fn visit_variable(&mut self, name: &Token, expr: &Option<Expr>) -> T;
}
