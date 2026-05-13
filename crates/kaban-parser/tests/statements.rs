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

#[test]
fn let_with_tuple_destructure() {
    test_snapshot!("let (x, y) = (10, 10.5);");
}

#[test]
fn let_with_nested_tuple_destructure() {
    test_snapshot!("let ((ax, ay), b) = ((10, foo()), 10.5);");
}

#[test]
fn let_with_nested_tuple_destructure_and_mutable_elements() {
    test_snapshot!("let ((mut ax, ay,), mut b) = ((10, foo()), 10.5);");
}

#[test]
fn let_with_struct_destructure() {
    test_snapshot!("let {x, y} = foo();");
}

#[test]
fn let_with_struct_destructure_with_mutable_and_bindings() {
    test_snapshot!("let {x: mut foo, y: buzz,} = foo();");
}

#[test]
fn let_with_nested_struct_destructure_with_mutable_and_bindings() {
    test_snapshot!("let {a: {ax: mut foo, mut ay}, b: buzz,} = foo();");
}
