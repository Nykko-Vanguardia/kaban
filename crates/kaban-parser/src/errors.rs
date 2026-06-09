use std::fmt::{Debug, Display};

use kaban_core::{UIndex, compiler_error::CompilerError};
use kaban_lexer::token::TokenKind;

use crate::node::TokenIndex;

#[derive(Debug, PartialEq)]
pub enum ParseErrorKind {
    ExpectedExpression,
    ExpectedToken(TokenKind),
    MissingTypeDeclaration,
    MissingBlock,
    MissingIdentifier,
    //TODO: Make this error actually coherent
    StructMutBinding,
    PubInLet,
    ExpectedMethod,
    InvalidModifierAfterSelf,
    ExpectedForOrColonColon,
    InvalidImplItem,
    MissingBlockOrSemicolon,
    MissingSelfReferenceModifier,
    RequiresExplicitBidningForIs,
}

pub struct ParseError {
    pub kind: ParseErrorKind,
    pub found: TokenKind,
    pub position: UIndex,
    pub token_index: TokenIndex,
}

impl CompilerError for ParseError {
    fn message(&self) -> String {
        let kind_message = match self.kind {
            ParseErrorKind::ExpectedExpression =>
                format!("Expressions expected! found {:?} instead", self.found),
            ParseErrorKind::ExpectedToken(token) =>
                format!("{:?} expected! found {:?} instead", token, self.found),
            ParseErrorKind::MissingTypeDeclaration => "Missing type declaration".to_string(),
            ParseErrorKind::MissingBlock => "Expected Block".to_string(),
            ParseErrorKind::MissingIdentifier => "Expected an Identifier or Destructure Pattern".to_string(),
            //TODO: Make this error actually coherent
            ParseErrorKind::StructMutBinding => "Mut expected to be after the : when binding not before".to_string(),
            ParseErrorKind::PubInLet => "Let statements can not be public, try using const at the top level".to_string(),
            ParseErrorKind::ExpectedMethod => "Expected a method after : access".to_string(),
            ParseErrorKind::InvalidModifierAfterSelf => "Invalid type modifier after self".to_string(),
            ParseErrorKind::ExpectedForOrColonColon => "Expected a for or colon colon here".to_string(),
            ParseErrorKind::InvalidImplItem => "Invalid statement inside this impl or trait block".to_string(),
            ParseErrorKind::MissingBlockOrSemicolon => "Expected either a block implementation or semicolon here".to_string(),
            ParseErrorKind::MissingSelfReferenceModifier => "Self must be a pointer, to send a copy you must explicitly declare the type. Did you mean to write self& or self &mut, or self*?".to_string(),
            ParseErrorKind::RequiresExplicitBidningForIs => "Left side of is statement is not an identifier, you need to cast the value to an identifier binding
                eg. (x[0] to time is Day.Monday) or (x[0] to {{x, y}} is WebEvent.Click)
            ".to_string(),
        };

        format!("ParseError at {}: {kind_message}", self.position)
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}
