use kaban_core::source::{IsSource};
use kaban_lexer::Lexer;
use kaban_parser::Parser;

macro_rules! test_snapshot {
    ($input:expr) => {
        let input = $input;
        let source = input.to_source();
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize(); 
        let ast = Parser::new(&tokens, source).parse_program();
        
        insta::assert_snapshot!(format!("input: {}\n\n{:#?}", input, ast.to_debugger()));
    };
}

#[test]
fn let_statement_with_no_type() {
    test_snapshot!("let x = 10 + 5;");
}

#[test]
fn let_statement_with_i32_type() {
    test_snapshot!("let y: i32 = 10 + 5;");
}

#[test]
fn let_mut_statement_with_i32_type() {
    test_snapshot!("let mut z: i32 = 10 + 5;");
}
