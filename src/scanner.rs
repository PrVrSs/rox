use std::collections::HashMap;
use std::iter;
use std::fmt;
use std::result;
use std::str;

use crate::token::{TokenType, Token, Literal};

#[derive(Debug)]
pub enum Error {
    Scanner(u64, char),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Scanner(ref line, ref msg) =>
                write!(f, "{}: {}", line, msg),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Scanner<'a> {
    src: iter::Peekable<str::Chars<'a>>,
    line: u64,
    offset: u64,
    eof: bool,
    keywords: HashMap<&'static str, TokenType>,
}

impl <'a>Scanner<'a> {
    pub fn new(c: &'a str) -> Self {
        Scanner {
            src: c.chars().peekable(),
            line: 1,
            offset: 0,
            eof: false,
            keywords: Scanner::init_keywords(),
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>> {
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            self.skip_whitespace();

            match self.src.next() {
                Some(symbol) => {
                    if !self.skip_comments(symbol) {
                        match self.scan_token(symbol) {
                            Ok(token) => tokens.push(token),
                            Err(error) => return Err(error),
                        }
                    }
                }
                None => {
                    tokens.push(self.token(TokenType::EOF));
                    break;
                }
            }
        }
        Ok(tokens)
    }

    fn init_keywords() -> HashMap<&'static str, TokenType> {
        [
            ("and",    TokenType::AND),
            ("class",  TokenType::CLASS),
            ("else",   TokenType::ELSE),
            ("false",  TokenType::FALSE),
            ("for",    TokenType::FOR),
            ("fun",    TokenType::FUN),
            ("if",     TokenType::IF),
            ("nil",    TokenType::NIL),
            ("or",     TokenType::OR),
            ("print",  TokenType::PRINT),
            ("return", TokenType::RETURN),
            ("super",  TokenType::SUPER),
            ("this",   TokenType::THIS),
            ("true",   TokenType::TRUE),
            ("var",    TokenType::VAR),
            ("while",  TokenType::WHILE),
        ].iter().cloned().collect()
    }

    fn skip_whitespace(&mut self) {
        while let Some(&symbol) = self.src.peek() {
            if !symbol.is_whitespace() {
                        break;
                    } else if symbol == '\n' {
                        self.line += 1;
                    }

                    self.src.next();
        }
    }

    fn skip_comments(&mut self, symbol: char) -> bool {
        if symbol == '/' && self.src.peek() == Some(&'/') {
            while let Some(&symbol) = self.src.peek() {
                self.src.next();

                if symbol == '\n' {
                    self.line += 1;

                    return true;
                }
            }
        }

        false
    }

    fn number(&mut self, symbol: char) -> Token {
        let mut string = symbol.to_string();

        while let Some(&symbol) = self.src.peek() {
            match symbol {
                c if c == '.' || c.is_digit(10) => string.push(c),
                _ => break,
            };

            self.src.next();
        }

        let float: f64 = string.parse().unwrap();

        Token::new(
            TokenType::NUMBER,
            self.line,
            Some(string),
            Some(Literal::Float(float))
        )
    }

    fn identifier(&mut self, symbol: char) -> Token {
        let mut string = symbol.to_string();

        while let Some(&symbol) = self.src.peek() {
            if !symbol.is_alphabetic() && symbol != '_' && !symbol.is_digit(10) {
                break;
            }

            string.push(symbol);
            self.src.next();
        }

        match self.keywords.get(&*string) {
            Some(keyword_type) => self.token(keyword_type.clone()),
            None => Token::new(
                TokenType::IDENTIFIER,
                self.line,
                Some(string),
                None
            ),
        }
    }

    fn token(& self, token_type: TokenType) -> Token {
        Token::new(token_type, self.line,None,None)
    }

    fn scan_token(&mut self, symbol: char) -> Result<Token> {
        match symbol {
            '(' => Ok(self.token(TokenType::LEFT_PAREN)),
            ')' => Ok(self.token(TokenType::RIGHT_PAREN)),
            '{' => Ok(self.token(TokenType::LEFT_BRACE)),
            '}' => Ok(self.token(TokenType::RIGHT_BRACE)),
            ',' => Ok(self.token(TokenType::COMMA)),
            '.' => Ok(self.token(TokenType::DOT)),
            '-' => Ok(self.token(TokenType::MINUS)),
            '+' => Ok(self.token(TokenType::PLUS)),
            ';' => Ok(self.token(TokenType::SEMICOLON)),
            '*' => Ok(self.token(TokenType::STAR)),
            _ => self.complex_token(symbol),
        }
    }

    fn complex_token(&mut self, symbol: char) -> Result<Token> {
        if symbol.is_digit(10) {
            Ok(self.number(symbol))
        } else if symbol.is_alphabetic() {
            Ok(self.identifier(symbol))
        } else {
            Err(Error::Scanner(self.line, symbol))
        }
    }
}
