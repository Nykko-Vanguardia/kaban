use kaban_core::source::IsSource;
use kaban_lexer::{Lexer, lexer::LexResult};
use kaban_parser::Parser;
mod test_macro;

#[test]
fn missing_semicolon() {
    test_snapshot!("x = 10");
}

#[test]
fn semicolon_on_block() {
    test_snapshot!("{};");
}

#[test]
fn missing_parenthesis_on_if() {
    test_snapshot!("if x == 10 {}");
}

#[test]
fn missing_single_parenthesis_on_if() {
    test_snapshot!("if x == 10) {}");
}

#[test]
fn hanging_infix_operator() {
    test_snapshot!("x.");
}

#[test]
fn hanging_braces() {
    test_snapshot!("{ let x = 10");
}

#[test]
fn semicolon_on_nested_block_expression() {
    test_snapshot!("{ let x = { let y = 10; pass y + 1; }; pass x; };");
}

// #[test]
// fn if_assignment() {
//     test_snapshot!("x = if (x == 10) pass 20;;");
// }
