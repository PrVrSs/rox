#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    //    Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF
}


#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Str(String),
    Float(f64),
}


impl Eq for Literal {}


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: Option<String>,
    pub literal: Option<Literal>,
    pub line: u64,
}


impl Default for Token {
    fn default() -> Self {
        Token {
            token_type: TokenType::EOF,
            lexeme: None,
            literal: None,
            line: 0,
        }
    }
}


impl Token {
    pub fn new(
        token_type: TokenType,
        line: u64,
        lexeme: Option<String>,
        literal: Option<Literal>,
    ) -> Self {
        Token { token_type, line, lexeme, literal, }
    }
}
