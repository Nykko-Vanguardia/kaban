use kaban_core::source::IsSource;

use crate::Token;
use crate::Lexer;
use crate::token::TokenKind;

#[test]
fn test_basic_lexing() {
    let input = "let x = 5 + 10;".to_source();
    let tokens = Lexer::new(input).tokenize();
    let expected = vec![
        Token::new(TokenKind::Let, 0, 3),
        Token::new(TokenKind::Identifier, 4, 5),
        Token::new(TokenKind::Equals, 6, 7),
        Token::new(TokenKind::IntLit, 8, 9),
        Token::new(TokenKind::Plus, 10, 11),
        Token::new(TokenKind::IntLit, 12, 14),
        Token::new(TokenKind::Semicolon, 14, 15),
        Token::new(TokenKind::EOF, 15, 15),
    ];
    assert_eq!(tokens, expected);
    println!("{:#?}", tokens);
}

#[test]
fn test_complex_expression() {
    let input = "let answer = (50 * 123) / 2;".to_source();
    let tokens = Lexer::new(input).tokenize();
    let expected = vec![
        Token::new(TokenKind::Let, 0, 3),
        Token::new(TokenKind::Identifier, 4, 10),
        Token::new(TokenKind::Equals, 11, 12),
        Token::new(TokenKind::LeftParen, 13, 14),
        Token::new(TokenKind::IntLit, 14, 16),
        Token::new(TokenKind::Star, 17, 18),
        Token::new(TokenKind::IntLit, 19, 22),
        Token::new(TokenKind::RightParen, 22, 23),
        Token::new(TokenKind::Slash, 24, 25),
        Token::new(TokenKind::IntLit, 26, 27),
        Token::new(TokenKind::Semicolon, 27, 28),
        Token::new(TokenKind::EOF, 28, 28),
    ];
    assert_eq!(tokens, expected);
}

#[test]
fn test_whitespace_insensitivity() {
    let input = "let    x\n    = \n   100   ;".to_source();
    let tokens = Lexer::new(input).tokenize();
    let expected = vec![
        Token::new(TokenKind::Let, 0, 3),
        Token::new(TokenKind::Identifier, 7, 8),
        Token::new(TokenKind::Equals, 13, 14),
        Token::new(TokenKind::IntLit, 19, 22),
        Token::new(TokenKind::Semicolon, 25, 26),
        Token::new(TokenKind::EOF, 26, 26),
    ];
    assert_eq!(tokens, expected);
}

#[test]
fn test_comments() {
    let input = "//The path of the righteous man is\n            /*\n              beset on all sides by the inequities of the selfish and the tyranny of evil men\n             */\n            let: i32;\n            ";
    let tokens = Lexer::new(input.to_source()).tokenize();
    // find positions of let, colon, i32, semicolon, eof
    let let_pos = input.find("let").unwrap() as u32;
    let colon_pos = input.find(':').unwrap() as u32;
    let i32_pos = input.find("i32").unwrap() as u32;
    let semi_pos = input.find(';').unwrap() as u32;
    let expected = vec![
        Token::new(TokenKind::Let, let_pos, let_pos + 3),
        Token::new(TokenKind::Colon, colon_pos, colon_pos + 1),
        Token::new(TokenKind::I32, i32_pos, i32_pos + 3),
        Token::new(TokenKind::Semicolon, semi_pos, semi_pos + 1),
        Token::new(TokenKind::EOF, input.len() as u32, input.len() as u32),
    ];
    assert_eq!(tokens, expected);
}

#[test]
fn test_strings() {
    let input = r#"let x = "let";"#.to_source();
    let tokens = Lexer::new(input).tokenize();
    let expected = vec![
        Token::new(TokenKind::Let, 0, 3),
        Token::new(TokenKind::Identifier, 4, 5),
        Token::new(TokenKind::Equals, 6, 7),
        Token::new(TokenKind::StringLit, 8, 13),  // includes quotes
        Token::new(TokenKind::Semicolon, 13, 14),
        Token::new(TokenKind::EOF, 14, 14),
    ];
    assert_eq!(tokens, expected);
}

#[test]
fn test_doc_comment() {
    let input = "/** * hello */".to_source();
    let tokens = Lexer::new(input).tokenize();
    let expected = vec![
        Token::new(TokenKind::DocComment, 0, 14),
        Token::new(TokenKind::EOF, 14, 14),
    ];
    assert_eq!(tokens, expected);
}
