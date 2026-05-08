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
fn addition_is_left_associative() {
    let input = "1 + 2 + 3;";
    let source = input.to_source();

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize(); 
    let ast = Parser::new(&tokens, source).parse_program();
    insta::assert_snapshot!(format!("input: {}\n\n{:#?}", input, ast.to_debugger()));
}

#[test]
fn array_literals_with_trailing_commas() {
    test_snapshot!("[10 + 1,] + [10, 2,];");
}

#[test]
fn mutable_and_non_mutable_method_calls() {
    test_snapshot!("x:foo(x, y,); x.bazz();");
}

#[test]
fn chained_expressions_call_index_deref() {
    test_snapshot!("v[!10] - w + x() + y[1] + z^;");
}

#[test]
fn parentheses_override_precedence() {
    test_snapshot!("(x + y) * w;");
}

#[test]
fn type_casting_with_complex_pointer_type() {
    test_snapshot!("x as i32*?[CONSTANT + 1]*;");
}

#[test]
fn type_casting_with_nested_union_types() {
    test_snapshot!("x as union(i32*, Person[], f64, union(i32&mut, f64&, c8 &mut));");
}

#[test]
fn bool_equality() {
    test_snapshot!("x == false;");
}

