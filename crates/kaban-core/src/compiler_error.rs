pub trait CompilerError {
    fn message(&self) -> String;
}
