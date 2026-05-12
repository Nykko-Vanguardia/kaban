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

#[test]
fn if_expression_with_braces() {
    test_snapshot!("if (x == 10) {
            foo();
    }");
}

#[test]
fn if_expression_without_braces() {
    test_snapshot!("if (x == 10) foo(); x + 10;");
}

#[test]
fn if_expression_else_condition_and_braces() {
    test_snapshot!("if (x == 10) { foo(); } else { bazz(); }");
}

#[test]
fn if_expression_else_condition_and_braces_and_multiple_expressions() {
    test_snapshot!("if (x == 10) { foo(); buzz(); } else { bazz(); }");
}

#[test]
fn if_expression_else_condition_without_braces() {
    test_snapshot!("if (x == 10) foo(); else bazz();");
}

#[test]
fn if_expression_with_else_if_condition() {
    test_snapshot!("if (x == 10) foo(); else if (x == y) buzz(); else bazz();");
}

#[test]
fn match_statement_with_brace_and_no_brace() {
    test_snapshot!("
        match (foo()) {
            10 => 20,
            20 => buzz(),
            _ => {
                bazz();
                fizz();
            },
        }
    ");
}

#[test]
fn do_while_condition_with_bool_condition() {
    test_snapshot!("do {foo(); let x = 10;} while (x == 10)");
}
