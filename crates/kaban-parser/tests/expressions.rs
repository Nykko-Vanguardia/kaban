use kaban_core::source::IsSource;
use kaban_lexer::Lexer;
use kaban_parser::Parser;

#[test]
fn addition_is_left_associative() {
    let input = "1 + 2 + 3;".to_source();

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize(); 
    let ast = Parser::new(&tokens, input).parse_program();
    insta::assert_debug_snapshot!(ast.to_debugger());
}

#[test]
fn array_literals_with_trailing_commas() {
    // let input = "[10 + 1,] + [10, 2,];".to_source();
    //
    // let mut lexer = Lexer::new(input);
    // let tokens = lexer.tokenize(); 
    // let ast = Parser::new(&tokens, input).parse_program();
}

#[test]
fn mutable_method_call_with_arguments() {
    // let input = "x:foo(x, y,);".to_source();
    // let tokens = Lexer::new(input).tokenize(); 
    // let ast = Parser::new(&tokens, input).parse_program();
}

#[test]
fn chained_expressions_call_index_deref() {
    // let input = "w + x() + y[1] + z^;".to_source();
    //
    // let tokens = Lexer::new(input).tokenize(); 
    // let mut ast = Parser::new(&tokens, input);
    // let expression = ast.parse_expression();
}

#[test]
fn parentheses_override_precedence() {
    // let input = "(x + y) * w;".to_source();
    //
    // let tokens = Lexer::new(input).tokenize(); 
    // let mut ast = Parser::new(&tokens, input);
}

#[test]
fn type_casting_with_complex_pointer_type() {
    // let input = "x as i32*?[CONSTANT + 1]*;".to_source();
    //
    // let tokens = Lexer::new(input).tokenize(); 
    // let mut ast = Parser::new(&tokens, input);
}

#[test]
fn type_casting_with_nested_union_types() {
    // let input = "x as union(i32*, Person[], f64, union(i32&mut, f64&, c8 &mut));".to_source();
    //
    // let tokens = Lexer::new(input).tokenize(); 
    // let mut ast = Parser::new(&tokens, input);
}

#[test]
fn bool_equality() {
    // let input = "x == false;".to_source();
    //
    // let tokens = Lexer::new(input).tokenize(); 
    // let mut ast = Parser::new(&tokens, input);
    // ast.parse_expression();
}
