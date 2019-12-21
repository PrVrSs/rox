use rox::scanner::Scanner;
use rox::token::{Token, TokenType, Literal};

#[test]
fn test_simple_token() {
    let test_data = "1";

    let result = Scanner::new(test_data).scan_tokens();
    let token = Token::new(
        TokenType::NUMBER,
        1,
        Some("1".to_string()),
        Some(Literal::Float(1.0))
    );

    assert_eq!(token, result.unwrap()[0]);
}
