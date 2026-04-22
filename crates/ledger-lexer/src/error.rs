#[derive(thiserror::Error, Debug, PartialEq)]
pub enum LexError {
    #[error("Float literal must have a digit after the decimal point")]
    InvalidFloat,
    #[error("Unexpected character")]
    UnexpectedCharacter,
    #[error("Incomplete String")]
    IncompleteString,
    #[error("Invalid Unicode")]
    InvalidUnicode,
    #[error("Char Literals must contain one character")]
    InvalidCharLiteral,
}
