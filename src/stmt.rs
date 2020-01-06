use crate::token::{Token};
use crate::expr::{Expr};


#[derive(Clone, Debug)]
pub enum Stmt {
    Empty,
    Variable(Token,Option<Expr>),
}
