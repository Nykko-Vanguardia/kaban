use kaban_lexer::Lexer;
use kaban_lexer::Token;
use crate::Parser;

#[test]
fn parse_list() {
    let input = "x, y, 10,);";

    let tokens = Lexer::new(input).tokenize(); 
    let mut ast = Parser::new(&tokens);
    let expression = ast.parse_comma_seperated_expressions(Token::RightParen);
    println!("{:#?}", expression);
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
    let input = "x as union(i32*, Person[], f64, union(i32&mut, f64&, char8 &mut))";

    let tokens = Lexer::new(input).tokenize(); 
    let mut ast = Parser::new(&tokens);
    let expression = ast.parse_expression();
    println!("{:#?}", expression);
    println!("{:#?}", ast.errors);
}
