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
    #[error("Missing closing brace")]
    MissingRightBrace,
    #[error("Expected Block")]
    MissingBlock,
    #[error("Expected an Identifier or Destructure Pattern")]
    MissingIdentifier,
    //TODO: Make this error actually coherent
    #[error("Mut expected to be after the : when binding not before")]
    StructMutBinding,
    #[error("Expected a > symbol to close generics")]
    MissingGreater,
    #[error("Let statements can not be public, try using const at the top level")]
    PubInLet,
    #[error("Expected a method after : access")]
    ExpectedMethod,
    #[error("Invalid type modifier after self")]
    InvalidModifierAfterSelf,
}
