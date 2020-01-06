use crate::token::{Token};

#[derive(Clone, Debug)]
pub enum Expr {
    Literal(Token),
    Variable(Token),
    Assign(Token, Box<Expr>),
    Logical(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
}

//// The abstract visitor
//pub trait Visitor<T> {
//    fn visit_name(&mut self, n: &Name) -> T;
//    fn visit_stmt(&mut self, s: &Stmt) -> T;
//    fn visit_expr(&mut self, e: &Expr) -> T;
//}
//
//
//struct Ast;
//
//impl Visitor<i64> for Ast {
//    fn visit_name(&mut self, n: &Name) -> i64 { panic!() }
//    fn visit_stmt(&mut self, s: &Stmt) -> i64 {
//        match *s {
//            Stmt::Expr(ref e) => self.visit_expr(e),
//            Stmt::Let(..) => unimplemented!(),
//        }
//    }
//
//    fn visit_expr(&mut self, e: &Expr) -> i64 {
//        match *e {
//            Expr::IntLit(n) => n,
//            Expr::Add(ref lhs, ref rhs) => self.visit_expr(lhs) + self.visit_expr(rhs),
//            Expr::Sub(ref lhs, ref rhs) => self.visit_expr(lhs) - self.visit_expr(rhs),
//        }
//    }
//}
//
//impl Expr {
//    pub fn accept<T>(&self, v: &mut Visitor<T>) -> T {}
//}
