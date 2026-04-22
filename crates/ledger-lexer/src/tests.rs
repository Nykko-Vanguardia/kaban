use crate::Token;
use crate::Lexer;

#[test]
fn test_basic_lexing() {
    let input = "let x = 5 + 10;";

    let tokens = Lexer::new(input).tokenize(); 

    let expected = vec![
        Token::Let,
        Token::Identifier(&input[4..5]),
        Token::Equals,
        Token::IntLit(&input[8..9]),
        Token::Plus,
        Token::IntLit(&input[12..14]),
        Token::Semicolon,
        Token::EOF,
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_complex_expression() {
    let input = "let answer = (50 * 123) / 2;";
    let tokens = Lexer::new(input).tokenize();

    let expected = vec![
        Token::Let,
        Token::Identifier(&input[4..10]),
        Token::Equals,
        Token::LeftParen,
        Token::IntLit(&input[14..16]),
        Token::Star,
        Token::IntLit(&input[19..22]),
        Token::RightParen,
        Token::Slash,
        Token::IntLit(&input[26..27]),
        Token::Semicolon,
        Token::EOF,
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_whitespace_insensitivity() {
    let input = "let    x
    = 
    100   ;";
    let tokens = Lexer::new(input).tokenize();

    let expected = vec![
        Token::Let,
        Token::Identifier(&input[7..8]),
        Token::Equals,
        Token::IntLit(&input[20..23]),
        Token::Semicolon,
        Token::EOF,
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_comments() {
    let input = "//The path of the righteous man is
            /*
              beset on all sides by the inequities of the selfish and the tyranny of evil men
             */

            let: i32;
            ";
            let tokens = Lexer::new(input).tokenize();

            let expected = vec![
                Token::Let,
                Token::Colon,
                Token::I32,
                Token::Semicolon,
                Token::EOF,
            ];

            assert_eq!(tokens, expected);
}

#[test]
fn test_strings() {
    let input = r#"let x = "let";"#; 
    let tokens = Lexer::new(input).tokenize();

    let expected = vec![
        Token::Let,
        Token::Identifier(&input[4..5]),
        Token::Equals,
        Token::StringLit(&input[9..12]),
        Token::Semicolon,
        Token::EOF,
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_raw_doc_comment() {
    let input = "/** * hello */";
    let tokens = Lexer::new(input).tokenize();

    let expected_slice = &input[3..12];

    assert_eq!(tokens[0], Token::DocComment(expected_slice));
}
