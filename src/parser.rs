use crate::token::{TokenType, Token};
use crate::error::{Result, Error};
use crate::stmt::Stmt;
use crate::expr::Expr;


#[derive(Debug)]
pub struct Parser {
    current: usize,
    tokens: Vec<Token>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser{current: 0, tokens}
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>> {
        let mut statements:Vec<Stmt> = Vec::new();

        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(error) => return Err(error),
            }
        }

        Ok(statements)
    }

    fn expression(&mut self) -> Result<Expr> {
        self.assignment()
    }

    fn declaration(&mut self) -> Result<Stmt> {
        if self.match_token(vec![TokenType::VAR]) {
            return self.var_declaration();
        }

        Err(Error::Parser(0, "".to_string()))
//        self.var_statement()
    }

    fn var_declaration(&mut self) -> Result<Stmt> {
        let name = self.consume(
            TokenType::IDENTIFIER,
            "Expect variable name."
        )?;

        let initializer = if self.match_token(vec![TokenType::EQUAL]) {
            Some(self.expression()?)
        } else { None };

        self.consume(TokenType::SEMICOLON, "Expect ';' after variable declaration.")?;

        Ok(Stmt::Variable(name, initializer))
    }

    fn assignment(&mut self) -> Result<Expr> {
        let expr= self.or();

        if self.match_token(vec![TokenType::EQUAL]) {
//            let equals = self.previous();
            let value = self.assignment()?;

            if let Expr::Variable(token) = expr {
                return Ok(Expr::Assign(token, Box::new(value)));
            }

            return Err(Error::Parser(0, "".to_string()));
        }

        Ok(expr)
    }

    fn or(&mut self) -> Expr {
        let mut expr = self.and();

        while self.match_token(vec![TokenType::OR]) {
            let op = self.previous();
            let right = self.and();

            expr = Expr::Logical(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn and(&mut self) -> Expr {
        let mut expr =  self.equality();

        while self.match_token(vec![TokenType::AND]) {
            let op = self.previous();
            let right = self.equality();

            expr = Expr::Logical(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn equality(&mut self) -> Expr {
        let mut expr =  self.comparison();

        while self.match_token(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let op = self.previous();
            let right = self.comparison();

            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.addition();

        while self.match_token(vec![TokenType::GREATER, TokenType::GREATER_EQUAL, TokenType::LESS, TokenType::LESS_EQUAL]) {
            let op = self.previous();
            let right = self.addition();

            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn addition(&mut self) -> Expr {
        let mut expr = self.multiplication();

        while self.match_token(vec![TokenType::PLUS, TokenType::MINUS]) {
            let op = self.previous();
            let right = self.multiplication();

            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn multiplication(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_token(vec![TokenType::SLASH, TokenType::STAR]) {
            let op = self.previous();
            let right = self.unary();

            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_token(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.unary();

            return Expr::Unary(operator, Box::new(right));
        }

        self.call()
    }

    fn call(&mut self) -> Expr {
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.match_token(vec![TokenType::NUMBER, TokenType::STRING]) {
            return Expr::Literal(self.previous());
        }

        if self.match_token(vec![TokenType::IDENTIFIER]){
            return Expr::Variable(self.previous());
        }

        Expr::Literal(self.previous())
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        Err(Error::Parser(0, message.to_string()))
    }

    fn match_token(&mut self, token_types: Vec<TokenType>) -> bool {
        if token_types.iter().any(|&token_type| self.check(token_type)) {
            self.advance();

            return true;
        }

        false
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() { self.current += 1; }
        self.previous()
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {return false;}

        self.peek().token_type == token_type
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }
}
