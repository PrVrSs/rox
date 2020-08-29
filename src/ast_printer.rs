use crate::expr::{Expr, Visitor, Acceptor};
use crate::token::{Token};


#[derive(Debug, Copy, Clone)]
pub struct AstPrinter {}


impl AstPrinter {
    pub fn pretty_print(&mut self, expr: Expr) -> String {
        expr.accept(self)
    }
}


impl Visitor<String> for AstPrinter {
    fn visit_assign(&mut self, name: &Token, value: &Expr) -> String {
        unimplemented!()
    }

    fn visit_variable(&mut self, name: &Token) -> String {
        unimplemented!()
    }

    fn visit_literal(&mut self, name: &Token) -> String {
        unimplemented!()
    }
}
