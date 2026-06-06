#[macro_export]
macro_rules! test_snapshot {
    ($input:expr) => {
        let (tx, rx) = std::sync::mpsc::channel();
        
        std::thread::spawn(move || {
            let input = $input;
            let source = input.to_source();
            let mut lexer = Lexer::new(source);
            let tokens = lexer.tokenize(); 
            let mut parser = Parser::new(&tokens, source);
            let ast = parser.parse_program();
            
            let formatted_string = if parser.errors.len() > 0 {
                format!("input: {}\n\n{:#?}\n\nerrors!: {:#?}", input, ast.to_debugger(), parser.errors)
            } else {
                format!("input: {}\n\n{:#?}", input, ast.to_debugger())
            };

            let _ = tx.send(formatted_string);
        });

        let print = rx.recv_timeout(std::time::Duration::from_millis(1000))
            .expect("Test execution timed out!");
        
        insta::assert_snapshot!(print);
    };
}
