use kaban_core::source::{IsSource};
use kaban_lexer::Lexer;
use kaban_parser::Parser;

macro_rules! test_snapshot {
    ($input:expr) => {
        let input = $input;
        let source = input.to_source();
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize(); 
        let mut parser = Parser::new(&tokens, source);
        let ast = parser.parse_program();
        let print = if parser.errors.len() > 0 {
            format!("input: {}\n\n{:#?}\n\nerrors!: {:#?}", input, ast.to_debugger(), parser.errors)
        } else {
            format!("input: {}\n\n{:#?}", input, ast.to_debugger())
        };
        
        insta::assert_snapshot!(print);
    };
}

#[test]
fn addition_is_left_associative() {
    let input = "1 + 2 + 3;";
    let source = input.to_source();

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize(); 
    let mut parser = Parser::new(&tokens, source);
    let ast = parser.parse_program();
    let print = if parser.errors.len() > 0 {
        format!("input: {}\n\n{:#?}\n\nerrors!: {:#?}", input, ast.to_debugger(), parser.errors)
    } else {
        format!("input: {}\n\n{:#?}", input, ast.to_debugger())
    };

    insta::assert_snapshot!(print);
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
fn bubble_up_on_undefined() {
    test_snapshot!("x?;");   
}

#[test]
fn panic_up_on_undefined() {
    test_snapshot!("x!;");   
}

#[test]
fn undefined_coalecing_with_member_access() {
    test_snapshot!("x ?? foo.bazz;");   
}

#[test]
fn  bubble_up_member_access() {
    test_snapshot!("foo?.bazz;");
}

#[test]
fn  panic_member_access() {
    test_snapshot!("foo!.bazz;");
}

#[test]
fn  undefined_chaining_member_access() {
    test_snapshot!("foo??.bazz;");
}

#[test]
fn chaining_method_and_member_access_calls() {
    test_snapshot!("items.iter().enumerate();");
}

#[test]
fn bool_equality() {
    test_snapshot!("x == false;");
}

#[test]
fn continue_and_break() {
    test_snapshot!("continue; break;");
}

#[test]
fn pass_and_break_without_returning_anything() {
    test_snapshot!("pass; return;");
}

#[test]
fn pass_value_and_break_and_returning_expression() {
    test_snapshot!("pass x + 10; return self.foo();");
}


#[test]
fn block_with_multiple_assignments_then_pass() {
    test_snapshot!("x = {
            let y = 10;
            let z = 20;
            pass y + z;
        };");
}

#[test]
fn if_expression_with_braces() {
    test_snapshot!("if (x == 10) {
            foo();
    }");
}

#[test]
fn if_expression_without_braces() {
    test_snapshot!("if (x == 10) x += 10; foo();");
}

#[test]
fn if_expression_return() {
    test_snapshot!("if (x == 10) return;");
}

#[test]
fn if_expression_return_with_value() {
    test_snapshot!("if (x == 10) return x(x);");
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
fn if_expression_with_else_if_condition_braces() {
    test_snapshot!("if (x == 10) { foo(); x+=10; } else if (x == y) { buzz(); } else { bazz(); }");
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
fn while_condition_with_bool_condition_and_braces() {
    test_snapshot!("while (x == 10) { x += 1; }");
}

#[test]
fn do_while_condition_with_bool_condition() {
    test_snapshot!("do {foo(); let x = 10;} while (x == 10);");
}

#[test]
fn for_condition_with_identifier_binding() {
    test_snapshot!("for (i in 1..10) {x += i;}");
}

#[test]
fn for_condition_with_tuple_destructure_binding_without_braces() {
    test_snapshot!("for ((i, v,) in items.iter().enumerate()) x += i;");
}

#[test]
fn identifier_assignment_with_binary_operation() {
    test_snapshot!("x = 10 + 5;");
}

#[test]
fn identifier_modulo_assignment_with_binary_operation() {
    test_snapshot!("x %= 20 * 10 % 30;");
}

#[test]
fn named_struct_instantiation_with_explicit_field_declarations() {
    test_snapshot!("Foo {x: x, y: y};");
}

#[test]
fn named_module_struct_instantiation_with_explicit_field_declarations() {
    test_snapshot!("Foo.Buzz {x: x, y: y};");
}

#[test]
fn named_module_struct_instantiation_with_implicit_field_declarations() {
    test_snapshot!("Foo.Buzz {x, y,};");
}

#[test]
fn named_module_struct_instantiation_with_explicit_and_implicit_field_declarations() {
    test_snapshot!("Foo.Buzz {w: foo() + 10, x, y, z: 10,};");
}

#[test]
fn anonymous_struct_instantiation_with_explicit_field_declarations() {
    test_snapshot!("{x: x, y: y};");
}

#[test]
fn anonymous_struct_instantiation_with_implicit_and_explicit_field_declarations() {
    test_snapshot!("{x, y: y,};");
}

#[test]
fn anonymous_struct_instantiation_with_implicit_field_declarations() {
    test_snapshot!("{x, y,};");
}

#[test]
fn anonymous_struct_instantiation_with_single_implicit_field_declarations() {
    test_snapshot!("{x,};");
}

#[test]
fn anonymous_struct_instantiation_with_nested_implicit_and_explicit_field_declarations() {
    test_snapshot!("{x, y: {a: b, b, c: 20,}};");
}


#[test]
fn anonymous_func_decl_with_explicit_types_and_braces() {
    test_snapshot!("x = func(x: i32, y: f64) -> i32 { pass x; };");
}

#[test]
fn anonymous_func_decl_with_explicit_types_without_braces() {
    test_snapshot!("x = func(x: i32, y: f64) -> i32 pass x;");
}

#[test]
fn anonymous_func_decl_with_implicit_types_and_braces() {
    test_snapshot!("x = func(x, y) { pass x; };");
}

#[test]
fn anonymous_func_decl_with_implicit_types_without_braces() {
    test_snapshot!("x = func(x, y,) pass x;");
}

#[test]
fn anonymous_func_decl_with_mut_and_implicit_and_explicit_types() {
    test_snapshot!("x = func(mut  x, y: i32,) { let x = 20; pass x; };");
}

#[test]
fn passing_a_callback_with_implicit_types() {
    test_snapshot!("foo(func(a, b) pass a + b, 20);");
}
//NOTE: REMOVED
// #[test]
// fn anonymous_func_decl_with_generics() {
//     test_snapshot!("x = func<T>(x: T, y) -> T { pass x; };");
// }
//
// #[test]
// fn anonymous_func_decl_with_multiple_generics() {
//     test_snapshot!("x = func<T, U,>(x: T, y: U) -> T { pass x; };");
// }
