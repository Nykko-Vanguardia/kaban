use kaban_core::source::IsSource;
use kaban_lexer::Lexer;
use crate::Parser;

#[test]
fn parse_list() {
    let input = "[10 + 1,] + [10, 2,];".to_source();

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize(); 
    let mut ast = Parser::new(&tokens, input);
    ast.parse_expression();
    println!("{}", ast.node_tags.len());
    println!("Node Tags: {:#?}", ast.node_tags);
    println!("{:#?}", ast.node_data);
    println!("{:#?}", ast.extra);
    // println!("{:#?}", ast.errors);
}

#[test]
fn parse_method() {
    let input = "x:foo(x, y,)".to_source();
    let tokens = Lexer::new(input).tokenize(); 
    let mut ast = Parser::new(&tokens, input);
    ast.parse_expression();
    // println!("{:#?}", ast.node_tags);
    // println!("{:#?}", ast.node_data);
    // println!("{:#?}", ast.extra);
}

#[test]
fn parse_expressions() {
    let input = "w + x() + y[1] + z^".to_source();

    let tokens = Lexer::new(input).tokenize(); 
    let mut ast = Parser::new(&tokens, input);
    let expression = ast.parse_expression();
    println!("{:#?}", expression);
}

#[test]
fn parse_expression_paren() {
    let input = "(x + y) * w".to_source();

    let tokens = Lexer::new(input).tokenize(); 
    let mut ast = Parser::new(&tokens, input);
    let expression = ast.parse_expression();
    println!("{:#?}", expression);
}

#[test]
fn parse_type_casting() {
    let input = "x as i32*?[CONSTANT + 1]*".to_source();

    let tokens = Lexer::new(input).tokenize(); 
    let mut ast = Parser::new(&tokens, input);
    let expression = ast.parse_expression();
    println!("{:#?}", expression);
}

#[test]
fn parse_type_casting_union() {
    let input = "x as union(i32*, Person[], f64, union(i32&mut, f64&, c8 &mut))".to_source();

    let tokens = Lexer::new(input).tokenize(); 
    let mut ast = Parser::new(&tokens, input);
    let expression = ast.parse_expression();
    println!("{:#?}", expression);
    println!("{:#?}", ast.errors);
}

#[test]
fn parse_bool() {
    let input = "x == false".to_source();

    let tokens = Lexer::new(input).tokenize(); 
    let mut ast = Parser::new(&tokens, input);
    ast.parse_expression();
    println!("{:#?}", ast.node_tags);
    println!("{:#?}", ast.node_data);
    println!("{:#?}", ast.extra);
}
