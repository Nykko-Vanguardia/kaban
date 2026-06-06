use kaban_core::source::IsSource;
use kaban_lexer::Lexer;
use kaban_parser::Parser;
// use kaban_parser::AST;
mod test_macro;

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
fn let_with_array_destructure() {
    test_snapshot!("let [x, y] = [[20, 10], [30, 10]];");
}

#[test]
fn let_with_nested_array_destructure_and_mutable_elements() {
    test_snapshot!("let [[mut ax, ay,], [bx, by]] = [[20, 10], [30, 10]];");
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

#[test]
fn let_statement_with_if_expression() {
    test_snapshot!("let x = if (x == 10) pass 20;");
}

#[test]
fn private_const_statement() {
    test_snapshot!("const MY_NUMBER: i32 = 10;");
}

#[test]
fn public_const_statement() {
    test_snapshot!("pub const MY_NUMBER: i32 = 10;");
}

#[test]
fn private_struct_decl_with_no_generics() {
    test_snapshot!("struct Point {x: i32, y: i32}");
}

#[test]
fn private_struct_decl_with_no_generics_and_public_fields() {
    test_snapshot!("struct Point {pub x: i32, pub y: i32,}");
}

#[test]
fn public_struct_decl_with_no_generics() {
    test_snapshot!("pub struct Point {x: i32, y: i32}");
}

#[test]
fn public_struct_decl_with_no_generics_and_public_fields() {
    test_snapshot!("struct Point {x: i32, pub y: i32,}");
}

#[test]
fn private_struct_decl_with_generics_and_no_constraints() {
    test_snapshot!("struct Point<T> {pub x: T, pub y: T,}");
}

#[test]
fn private_struct_decl_with_generics_and_one_i32_constraint() {
    test_snapshot!("struct Point<T: i32> {pub x: T, pub y: T,}");
}

#[test]
fn private_struct_decl_with_generics_and_one_i32_or_f64_constraint() {
    test_snapshot!("struct Point<T: i32 | f64> {pub x: T, pub y: T,}");
}

#[test]
fn private_struct_decl_with_generics_and_impl_or_constraint() {
    test_snapshot!("struct Point<T: impl Serializable | impl Debug> {pub x: T, pub y: T,}");
}

#[test]
fn private_struct_decl_with_generics_and_impl_and_constraint() {
    test_snapshot!("struct Point<T: impl Serializable & impl Debug> {pub x: T, pub y: T,}");
}

#[test]
fn private_struct_decl_with_generics_and_impl_and_and_or_constraint() {
    test_snapshot!(
        "struct Point<T: impl Serializable & impl Debug | impl DebugSerializable> {pub x: T, pub y: T,}"
    );
}

//NOTE: FOR NOW ITS ALWAYS LEFT PRECEDENCE, I do not know if i want to add precedence of and over
//or
#[test]
fn private_struct_decl_with_generics_and_parenthesis_constraint() {
    test_snapshot!(
        "struct Point<T: impl Serializable & (impl Debug | impl DebugSerializable)> {pub x: T, pub y: T,}"
    );
}

//NOTE: FOR NOW ITS ALWAYS LEFT PRECEDENCE, I do not know if i want to add precedence of and over
//or
#[test]
fn private_struct_decl_with_multiple_generics() {
    test_snapshot!("struct Point<T, U,> {pub x: T, pub y: U,}");
}

//NOTE: FOR NOW ITS ALWAYS LEFT PRECEDENCE, I do not know if i want to add precedence of and over
//or
#[test]
fn private_struct_decl_with_multiple_generics_and_interface_constraints() {
    test_snapshot!("struct Point<T: impl Serializable, U: impl Debug,> {pub x: T, pub y: U,}");
}

#[test]
fn private_enum_decl_with_tags_only() {
    test_snapshot!("enum Day {Sunday, Monday, Tuesday}");
}

#[test]
fn public_enum_decl_with_tags_only() {
    test_snapshot!("pub enum Day {Sunday, Monday, Tuesday}");
}

#[test]
fn private_enum_decl_with_type_assignments() {
    test_snapshot!("enum Day {Sunday: i32, Monday: f64, Tuesday,}");
}

#[test]
fn private_enum_decl_with_type_assignments_and_generics() {
    test_snapshot!("enum Day<T> {Sunday: i32, Monday: f64, Tuesday: T,}");
}

#[test]
fn private_enum_decl_with_type_assignments_and_struct_and_tuple_decl() {
    test_snapshot!(
        "enum Day {Sunday: i32, Monday: struct {hour: u8, money: f64,}, Tuesday: (i32, f64),}"
    );
}

#[test]
fn private_func_decl_with_no_generics_and_no_return_type() {
    test_snapshot!("func foo(x: i32, y: f64) { let z = x + y; return z; }");
}

#[test]
fn private_func_decl_with_no_generics_and_with_return_type() {
    test_snapshot!("func foo(x: i32, y: f64) -> f64 { let z = x + y; return z; }");
}

#[test]
fn private_func_decl_with_no_generics_and_with_return_type_and_mut_values() {
    test_snapshot!("func foo(mut x: i32, y: f64,) -> f64 { let z = x + y; return z; }");
}

#[test]
fn private_func_decl_with_generics_and_with_return_type_and_mut_values() {
    test_snapshot!("func foo<T,>(mut x: T, y: f64,) -> T { let z = x + y; return z; }");
}

#[test]
fn private_func_decl_with_multiple_generics_and_with_return_type_and_mut_values() {
    test_snapshot!("func foo<T, U>(mut x: T, y: U,) -> T { let z = x + y; return z; }");
}

#[test]
fn public_func_decl_with_generic_constaint_interface_sugar() {
    test_snapshot!(
        "pub func foo(mut x: impl Serializable, y: impl Debug & impl Clone) -> T { let z = x + y; return z; }"
    );
}

// REMOVED
// #[test]
// fn func_decl_with_self_param() {
//     test_snapshot!("func foo(self, x: i32) -> i32 { let z = x; return z; }");
// }
//
// #[test]
// fn func_decl_with_self_param_only() {
//     test_snapshot!("func foo(self) -> i32 { let z = self.y; return z; }");
// }

#[test]
fn func_decl_with_self_read_reference() {
    test_snapshot!("func foo(self&) -> i32 { let z = self.y; return z; }");
}

#[test]
fn func_decl_with_self_mut_reference() {
    test_snapshot!("func foo(self&mut,) -> i32 { let z = self.y; return z; }");
}

#[test]
fn func_decl_with_self_pointer() {
    test_snapshot!("func foo(self*, x: i32) -> i32 { let z = self.y; return z; }");
}

// REMOVED
// #[test]
// fn func_decl_with_mut_self() {
//     test_snapshot!("func foo(mut self, x: i32) -> i32 { let z = self.y; return z; }");
// }

// #[test]
// fn func_decl_with_self_param_only_and_generics() {
//     test_snapshot!("func foo<T>(self, y: T) -> i32 { let z = self.y; return z; }");
// }

#[test]
fn impl_decl_with_no_generics() {
    test_snapshot!(
        "
    impl Person::Core {
        pub const NUMBER: u8 = 10;

        pub func walk(self&, steps: i32) {
            self.step(steps);
        }

        func step(step: i32) -> i32 {
            return step;
        }
    }
    "
    );
}

#[test]
fn impl_decl_with_generics() {
    test_snapshot!(
        "
    impl Person<T>::Core<T> {
        pub const NUMBER: u8 = 10;

        pub func walk(self&, steps: i32) {
            self.step(steps);
        }

        func step(step: i32) -> i32 {
            return step;
        }
    }
    "
    );
}

#[test]
fn impl_decl_with_generics_and_constaint() {
    test_snapshot!(
        "
    impl Talks for Person<T>::Core<T> {
        pub const NUMBER: u8 = 10;

        pub func walk(self&, steps: i32) {
            self.step(steps);
        }

        func step(step: i32) -> i32 {
            return step;
        }

        func default_talk(self&, message: c8);
    }
    "
    );
}

#[test]
fn impl_decl_with_no_generics_and_constaint() {
    test_snapshot!(
        "
    impl Talks for Person::Core {
        pub const NUMBER: u8 = 10;

        pub func walk(self&, steps: i32) {
            self.step(steps);
        }

        func step(step: i32) -> i32 {
            return step;
        }

        func default_talk(self&, message: c8);
    }
    "
    );
}

#[test]
fn interface_decl_with_no_shape_no_generics() {
    test_snapshot!(
        "
    pub interface Talks {
        func step(step: i32) -> i32 {
            return step;
        }

        func default_talk(self&, message: c8);
    }
    "
    );
}

#[test]
fn interface_decl_with_shape_no_generics() {
    test_snapshot!(
        "
    pub interface Talks {
        shape: struct { x: i32, y: i32 }
        func step(step: i32) -> i32 {
            return self.x;
        }

        func default_talk(self&, message: c8);
    }
    "
    );
}

#[test]
fn interface_decl_with_no_shape_generics() {
    test_snapshot!(
        "
    pub interface Talks<T> {
        func step(step: T) -> T {
            return step;
        }

        func default_talk(self&, message: c8);
    }
    "
    );
}

#[test]
fn interface_decl_with_shape_generics() {
    test_snapshot!(
        "
    pub interface Talks<T> {
        shape: struct { x: i32, y: i32 }

        func step(step: T) -> T {
            return self.x;
        }

        func default_talk(self&, message: c8);
    }
    "
    );
}

#[test]
fn func_decl_with_no_params() {
    test_snapshot!("func foo() -> i32 { return 5; }");
}

#[test]
fn pub_func_decl_with_no_params() {
    test_snapshot!("pub func foo() -> i32 { return 5; }");
}

#[test]
fn func_decl_with_self_and_multiple_params() {
    test_snapshot!("func foo(self&, x: i32, y: f64) -> i32 { return x; }");
}

#[test]
fn func_decl_with_self_mut_and_multiple_params() {
    test_snapshot!("func foo(self&mut, x: i32) -> i32 { return x; }");
}

#[test]
fn func_decl_with_generic_and_self_param() {
    test_snapshot!("func foo<T>(self&, x: T) -> T { return x; }");
}

#[test]
fn let_with_true_literal() {
    test_snapshot!("let x = true;");
}

#[test]
fn let_with_string_literal() {
    test_snapshot!(r#"let x = "hello";"#);
}

#[test]
fn struct_decl_single_field() {
    test_snapshot!("struct Foo { x: i32 }");
}

#[test]
fn struct_field_with_generic_type() {
    test_snapshot!("struct Foo { x: HashMap<String, i32>, }");
}

#[test]
fn struct_field_with_pointer_type() {
    test_snapshot!("struct Foo { x: i32*, y: String&, }");
}

#[test]
fn impl_decl_with_anonymous_constructor_only() {
    test_snapshot!("impl Person::Core { pub func new() -> self { } }");
}

#[test]
fn impl_decl_with_named_constructor() {
    test_snapshot!("impl Person::Factory { pub func from_str(s: String&) -> self { } }");
}

#[test]
fn impl_decl_with_constructor_and_methods() {
    test_snapshot!(
        "
        impl Person::Core {
            pub func new(name: String*, age: i32) -> self { }
            func get_name(self&) -> String& { return self.name; }
        }
    "
    );
}

#[test]
fn multiple_sequential_func_decls() {
    test_snapshot!(
        "
        func foo() -> i32 { return 1; }
        func bar() -> i32 { return 2; }
        func baz() -> i32 { return 3; }
    "
    );
}

#[test]
fn pub_const_with_binary_expression() {
    test_snapshot!("pub const FOO: i32 = 10 + 5 * 2;");
}

#[test]
fn enum_decl_with_no_variants() {
    test_snapshot!("enum Empty {}");
}

#[test]
fn private_func_decl_with_no_params_and_no_return_type() {
    test_snapshot!("func foo() { }");
}

#[test]
fn struct_and_impl_sequential() {
    test_snapshot!(
        "
        struct Point { x: i32, y: i32 }
        impl Point::Core {
            func distance(self&) -> f64 { return 0.0; }
        }
    "
    );
}

#[allow(dead_code)]
fn panic_print(source: &str) {
    let source = source.to_source();
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(&tokens, source);
    let ast = parser.parse_program();
    ast.panic_print();
}
