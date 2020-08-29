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


impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Expr::Assign(name, variable) => visitor.visit_assign(name, variable),
            Expr::Variable(name) => visitor.visit_variable(name),
            Expr::Literal(name) => visitor.visit_literal(name),
            _ => panic!()
        }
    }
}


pub trait Visitor<T> {
    fn visit_assign(&mut self, name: &Token, value: &Expr) -> T;
    fn visit_variable(&mut self, name: &Token) -> T;
    fn visit_literal(&mut self, name: &Token) -> T;
}


// pub trait Acceptor<T> {
//     fn accept(&self, visitor: &mut dyn Visitor<T>) -> T;
// }
//
//
// impl<T> Acceptor<T> for Expr {
//     fn accept(&self, visitor: &mut dyn Visitor<T>) -> T {
//         match self {
//             Expr::Assign(name, variable) => visitor.visit_assign(name, variable),
//             _ => panic!()
//         }
//     }
// }


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
