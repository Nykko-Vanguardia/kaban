use kaban_lexer::Lexer;
use crate::Parser;

#[test]
fn parse_list() {
    let input = "[10 + 1,] + [10, 2,];";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize(); 
    let mut ast = Parser::new(&tokens);
    let expression = ast.parse_expression();
    println!("{:#?}", expression);
    println!("{:#?}", ast.errors);
    println!("{:#?}", lexer.errors);
}

#[test]
fn parse_method() {
    let input = "x:foo(x, y,)";

    let tokens = Lexer::new(input).tokenize(); 
    let mut ast = Parser::new(&tokens);
    let expression = ast.parse_expression();
    println!("{:#?}", expression);
}

#[test]
fn parse_expressions() {
    let input = "w + x() + y[1] + z^";

    let tokens = Lexer::new(input).tokenize(); 
    let mut ast = Parser::new(&tokens);
    let expression = ast.parse_expression();
    println!("{:#?}", expression);
}

#[test]
fn parse_expression_paren() {
    let input = "(x + y) * w";

    let tokens = Lexer::new(input).tokenize(); 
    let mut ast = Parser::new(&tokens);
    let expression = ast.parse_expression();
    println!("{:#?}", expression);
}

#[test]
fn parse_type_casting() {
    let input = "x as i32*?[CONSTANT + 1]*";

    let tokens = Lexer::new(input).tokenize(); 
    let mut ast = Parser::new(&tokens);
    let expression = ast.parse_expression();
    println!("{:#?}", expression);
}

#[test]
fn parse_type_casting_union() {
    let input = "x as union(i32*, Person[], f64, union(i32&mut, f64&, c8 &mut))";

    let tokens = Lexer::new(input).tokenize(); 
    let mut ast = Parser::new(&tokens);
    let expression = ast.parse_expression();
    println!("{:#?}", expression);
    println!("{:#?}", ast.errors);
}
