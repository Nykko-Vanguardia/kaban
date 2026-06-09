use kaban_core::source::IsSource;
use kaban_lexer::{Lexer, lexer::LexResult};
use kaban_parser::Parser;
mod test_macro;

#[test]
fn addition_is_left_associative() {
    let input = "1 + 2 + 3;";
    let source = input.to_source();

    let mut lexer = Lexer::new(source);
    let LexResult { result, .. } = lexer.tokenize();
    let mut parser = Parser::new(&result, source);
    let ast = parser.parse_program();

    let print = format!("input: {}\n\n{:#?}", input, ast);
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
fn type_casting_with_i32() {
    test_snapshot!("x as i32;");
}

#[test]
fn type_casting_with_dynamic_array() {
    test_snapshot!("x as i32[];");
}

#[test]
fn type_casting_with_complex_pointer_type() {
    test_snapshot!("x as i32*?[CONSTANT + 1]*;");
}

#[test]
fn type_casting_with_simple_union_type() {
    test_snapshot!("x as union(i32*, f64, Person);");
}

#[test]
fn type_casting_with_nested_union_types() {
    test_snapshot!("x as union(i32*, Person[], f64, union(i32&mut, f64&, c8 &mut));");
}

#[test]
fn type_casting_with_types_with_generics() {
    test_snapshot!("x as HashMap<String, i32>;");
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
fn bubble_up_member_access() {
    test_snapshot!("foo?.bazz;");
}

#[test]
fn panic_member_access() {
    test_snapshot!("foo!.bazz;");
}

#[test]
fn undefined_chaining_member_access() {
    test_snapshot!("foo??.bazz;");
}

#[test]
fn implementation_access() {
    test_snapshot!("Person::Core;");
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
    test_snapshot!(
        "x = {
            let y = 10;
            let z = 20;
            pass y + z;
        };"
    );
}

#[test]
fn if_expression_with_braces() {
    test_snapshot!(
        "if (x == 10) {
            foo();
    }"
    );
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
    test_snapshot!("if (x == 10) foo() else bazz();");
}

#[test]
fn if_expression_with_else_if_condition() {
    test_snapshot!("if (x == 10) foo() else if (x == y) buzz() else bazz();");
}

#[test]
fn if_expression_with_else_if_condition_braces() {
    test_snapshot!("if (x == 10) { foo(); x+=10; } else if (x == y) { buzz(); } else { bazz(); }");
}

#[test]
fn if_expression_with_is_condition() {
    test_snapshot!("if (x is Day.Monday) { foo(); } else if (x is type i32) { buzz(); }");
}

#[test]
fn if_expression_with_is_and_to_binding_condition() {
    test_snapshot!("if (x to time is Day.Monday) { foo(); }");
}

#[test]
fn if_expression_with_is_and_complex_to_binding_condition() {
    test_snapshot!(
        "if (x to mut time is Day.Monday) { foo(); } else if (x to {mut y, z,} is Day.Tuesday) { buzz(); }"
    );
}

#[test]
fn match_statement_with_brace_and_no_brace() {
    test_snapshot!(
        "
        match (foo()) {
            10 => 20,
            20 => buzz(),
            _ => {
                bazz();
                fizz();
            },
        }
    "
    );
}

#[test]
fn match_statement_with_brace_and_no_brace_and_pipe() {
    test_snapshot!(
        "
        match (foo()) {
            10 | 30 | 40 => 20,
            20 => buzz(),
            _ => {
                bazz();
                fizz();
            },
        }
    "
    );
}

#[test]
fn match_statement_with_is_statement() {
    test_snapshot!(
        "
        match (day_enum) {
            is Day.Monday => 20,
            to time is Day.Tuesday => buzz(),
            to {time, mut wednesday_event} is Day.Wednesday => buzz(),
        }
    "
    );
}

#[test]
fn match_statement_with_is_statement_and_pipes() {
    test_snapshot!(
        "
        match (day_enum) {
            is Day.Monday
            | is Day.Tuesday
            | 32 => 20,
            to time is Day.Wednesday => buzz(),
        }
    "
    );
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
fn anonymous_func_decl_with_concrete_func_type() {
    test_snapshot!("let x: func(x: i32, y: i32) -> i32 = func(x, y) pass x + y;");
}

///Decided it was a good idea to add mut to function types, you can explicitly say the function i
///expect needs a mutable param x
#[test]
fn anonymous_func_decl_with_concrete_func_type_and_mut_params() {
    test_snapshot!("let x: func(mut x: i32, y: i32) -> i32 = func(mut x, y) pass x + y;");
}

#[test]
fn passing_a_callback_with_implicit_types() {
    test_snapshot!("foo(func(a, b) pass a + b, 20);");
}

#[test]
fn generic_instantiation_function() {
    test_snapshot!("foo@<i32>(10);");
}

#[test]
fn generic_instantiation_function_with_multiple_types() {
    test_snapshot!("foo@<i32, f64,>(10);");
}

#[test]
fn methods_with_generic_instantiation() {
    test_snapshot!("foo.buzz@<i32,>(10);");
}

#[test]
fn methods_with_generic_instantiation_with_multiple_types() {
    test_snapshot!("foo.buzz.bazz@<i32, f64,>(10);");
}

#[test]
fn mutable_self_methods_with_generic_instantiation_with_multiple_types() {
    test_snapshot!("foo:bazz@<i32, f64,>(10);");
}

#[test]
fn mutable_self_methods_with_generic_instantiation_with_multiple_types_from_array_indexed_obj() {
    test_snapshot!("foos[0]:bazz@<i32, f64,>(10);");
}

#[test]
fn member_access_generic_instantiation() {
    test_snapshot!("Person.Obj@<i32>;");
}

#[test]
fn comptime_func_call() {
    test_snapshot!("@foo();");
}

#[test]
fn comptime_block() {
    test_snapshot!("@ {foo(); pass 5 + 10;}");
}

#[test]
fn reference_mut_ref_and_ownership_of() {
    test_snapshot!("let x = &y; let z = &mut w; let u = *v;");
}

#[test]
fn anonymous_enum_type() {
    test_snapshot!("x as enum { Default, Fast, Clean: i32, };");
}

#[test]
fn anonymous_enum_access() {
    test_snapshot!("return enum.Day;");
}

#[test]
fn true_literal() {
    test_snapshot!("x == true;");
}

#[test]
fn string_literal_expression() {
    test_snapshot!(r#""hello";"#);
}

#[test]
fn unary_minus() {
    test_snapshot!("-10;");
}

#[test]
fn empty_block_expression() {
    test_snapshot!("{}");
}

#[test]
fn nested_block_expression() {
    test_snapshot!("{ let x = { let y = 10; pass y + 1; }; pass x; }");
}

#[test]
fn member_access_assignment() {
    test_snapshot!("a.b = 10;");
}

#[test]
fn index_assignment() {
    test_snapshot!("a[i] = 10;");
}

#[test]
fn nested_struct_instantiation() {
    test_snapshot!("Foo { x: Bar { y: 10 }, z: 20 };");
}

#[test]
fn long_member_access_chain() {
    test_snapshot!("a.b.c.d;");
}

#[test]
fn mixed_member_and_method_chain() {
    test_snapshot!("a.b().c.d();");
}

#[test]
fn impl_access_method_call() {
    test_snapshot!("Person::Core.new();");
}

#[test]
fn do_while_with_compound_condition() {
    test_snapshot!("do { x += 1; } while (x < 10 && y > 0);");
}

#[test]
fn for_loop_with_mut_binding() {
    test_snapshot!("for (mut i in 1..10) { x += i; }");
}

#[test]
fn nested_match_expression() {
    test_snapshot!(
        "
        match (x) {
            10 => match (y) {
                20 => foo(),
                _ => bazz(),
            },
            _ => bar(),
        }
    "
    );
}

#[test]
fn nested_function_calls() {
    test_snapshot!("foo(bar(baz(10)));");
}

#[test]
fn mutable_impl_access_method_call() {
    test_snapshot!("Person::Core.new():set_name(\"foo\");");
}

#[test]
fn assignment_to_nested_member() {
    test_snapshot!("a.b.c = 10;");
}

#[test]
fn chained_index_and_method() {
    test_snapshot!("arr[0].method();");
}

#[test]
fn if_expression_as_argument() {
    test_snapshot!("foo(if (x > 0) pass x else pass y);");
}

#[test]
fn block_expression_as_argument() {
    test_snapshot!("foo({ let x = 10; pass x + 1; });");
}

#[test]
fn passing_type_to_function() {
    test_snapshot!("foo(type i32);");
}

#[test]
fn primative_impl_access() {
    test_snapshot!("type i32::Core.method();");
}

#[test]
fn primative_impl_access_with_parenthesis() {
    test_snapshot!("(type i32)::Core.method();");
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
//
// #[test]
// fn new_method_call() {
//     test_snapshot!("new Person@<i32>::Core@<i32, f64>.from_id(10);");
// }
//
// #[test]
// fn destruct_method_call() {
//     test_snapshot!("destruct Person@<i32>::Core@<i32, f64>.from_id(10);");
// }
//
