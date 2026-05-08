use kaban_lexer::token::TokenKind;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ParseError {
    // #[error("{0} expected!")]
    // Expected(String), 
    #[error("{0:?} expected!")]
    ExpectedToken(TokenKind),
    #[error("Missing type declaration")]
    MissingTypeDeclaration,
    #[error("All statements must end with a semicolon!")]
    MissingSemicolon,
    #[error("Missing closing bracket")]
    MissingRightBracket,
    #[error("Missing closing parenthesis")]
    MissingRightParen,
    #[error("Missing parenthesis")]
    MissingLeftParen,
}
