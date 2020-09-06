use crate::expr::{Expr, Visitor as ExprVisitor};
use crate::stmt::{Stmt, Visitor as StmtVisitor};
use crate::token::{Token, Literal};


#[derive(Debug, Copy, Clone)]
pub struct AstPrinter {}


impl AstPrinter {

    pub fn print_expr(&mut self, expr: Expr) -> String {
        expr.accept(self)
    }

    pub fn print_stmt(&mut self, stmt: Stmt) -> String {
        stmt.accept(self)
    }
}


impl StmtVisitor<String> for AstPrinter {
    fn visit_variable(&mut self, name: &Token, expr: &Option<Expr>) -> String {
        let var_name = name.lexeme.clone().unwrap();

        let var_value = match expr {
            Some(value) => self.print_expr(value.clone()),
            _ => "".to_string()
        };

        format!("{} = {}", var_name, var_value)
    }

}


impl ExprVisitor<String> for AstPrinter {
    fn visit_assign(&mut self, name: &Token, value: &Expr) -> String {
        unimplemented!()
    }

    fn visit_literal(&mut self, token: &Token) -> String {
        if let Some(i) = token.clone().lexeme {
            return i;
        }

        "".to_string()
    }
}
