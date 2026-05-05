use crate::operator;

#[derive(Debug)]
pub enum Statement<'a> {
    Let {
        mutable: bool,
        name: &'a str,
        let_type: Option<Type<'a>>,
        assignment: Expression<'a>,
    },

    FuncDecl {
        public: bool,
        comptime: bool,
        params: Vec<Param<'a>>,
        name: &'a str,
        return_type: Type<'a>,
        body_block: Expression<'a>,
    },

    Return(Expression<'a>),
    Pass(Expression<'a>),
    Break,
    Continue,

    ExpressionStatement(Expression<'a>),
}

#[derive(Debug)]
pub enum Expression<'a> {
    IntLit(&'a str),
    FloatLit(&'a str),
    Identifier(&'a str),
    ArrayLit(Vec<Expression<'a>>),
    BoolLit(bool),
    Char8Lit(u8),
    Char16Lit(&'a [u8]),
    Char32Lit(&'a [u8]),
    Undefined,
    Garbage,
    Self_,

    Block {
        statements: Vec<Statement<'a>>,
        value: Option<Box<Expression<'a>>>
    },
    If {
        condition: Box<Expression<'a>>,
        then_block: Box<Expression<'a>>,
        else_block: Option<Box<Expression<'a>>>,
    },
    Match {
        subject: Box<Expression<'a>>,
        arms: Vec<MatchArm<'a>>,
    },
    ArithmeticOperation {
        left: Box<Expression<'a>>,
        right: Box<Expression<'a>>,
        operator: operator::Arithmetic,
    },
    ComparisonOperation {
        left: Box<Expression<'a>>,
        right: Box<Expression<'a>>,
        operator: operator::Comparison,
    },
    LogicalOperation {
        left: Box<Expression<'a>>,
        right: Box<Expression<'a>>,
        operator: operator::Logical,
    },
    BinaryOperation {
        left: Box<Expression<'a>>,
        right: Box<Expression<'a>>,
        operator: operator::BitwiseBinary,
    },
    MemberAccess {
        parent: Box<Expression<'a>>,
        child: Box<Expression<'a>>,
        operator: operator::MemberAccess,
    },
    IndexOperation {
        parent: Box<Expression<'a>>,
        index: Box<Expression<'a>>,
        safe: bool,
    },
    UndefinedCoalescing {
        possibly_undefined: Box<Expression<'a>>,
        default: Box<Expression<'a>>,
    },
    TypeCasting {
        value: Box<Expression<'a>>,
        type_: Type<'a>,
    },
    PrefixUnaryOperation {
        operand: Box<Expression<'a>>,
        operator: operator::PrefixUnary,
    },
    PostfixUnaryOperation {
        operand: Box<Expression<'a>>,
        operator: operator::PostfixUnary,
    },
    FunctionCall {
        callee: Box<Expression<'a>>,
        args: Vec<Expression<'a>>,
    },
    MethodCall {
        parent: Box<Expression<'a>>,
        method_name: &'a str,
        args: Vec<Expression<'a>>,
        mutable_self: bool,
    }
}

impl<'a> Expression<'a> {
    pub fn to_box(self) -> Box<Expression<'a>> {
        Box::new(self)
    }

    pub fn to_some(self) -> Option<Expression<'a>> {
        Some(self)
    }
}

#[derive(Debug)]
pub enum Type<'a> {
    //primitives
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    U8,
    U16,
    U32,
    U64,
    USize,
    Bool,
    Void,
    C8,
    C16,
    C32,
    Undefined,
    Garbage,

    //modifiers — recursive
    Pointer(Box<Type<'a>>), //T*
    Borrow(Box<Type<'a>>), //T&
    MutBorrow(Box<Type<'a>>), //T &mut
    Optional(Box<Type<'a>>), //T?
    OptionalGarbage(Box<Type<'a>>), //T!

    //arrays
    FixedArray{ type_: Box<Type<'a>>, size: Box<Expression<'a>> }, // T[N]
    DynArray(Box<Type<'a>>), // T[]

    //user defined
    Named(&'a str), // Person, MyStruct etc

    //compound
    Union(Vec<Type<'a>>), // union(i32, f64)
}

impl<'a> Type<'a> {
    pub fn to_box(self) -> Box<Type<'a>> {
        Box::new(self)
    }

    pub fn to_some(self) -> Option<Type<'a>> {
        Some(self)
    }
}

#[derive(Debug)]
pub struct Param<'a> {
    name: &'a str,
    type_: Type<'a>,
    mutable: bool,
}

#[derive(Debug)]
pub struct MatchArm<'a> {
    match_to: Box<Expression<'a>>,
    body_block: Box<Expression<'a>>,
}
