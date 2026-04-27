use crate::operator;

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
    UndefinedCoalescing {
        possibly_undefined: Box<Expression<'a>>,
        default: Box<Expression<'a>>,
    },
    TypeCasting {
        value: Box<Expression<'a>>,
        type_: Type<'a>,
    },
    PrefixUnaryOperator {
        operand: Box<Expression<'a>>,
        operator: operator::PrefixUnary,
    },
    PostfixUnaryOperator {
        operand: Box<Expression<'a>>,
        operator: operator::PostfixUnary,
    }
}

impl<'a> Expression<'a> {
}

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
    Char8,
    Char16,
    Char32,
    Undefined,
    Garbage,

    //modifiers — recursive
    Pointer(Box<Type<'a>>), //T*
    Borrow(Box<Type<'a>>), //T&
    MutBorrow(Box<Type<'a>>), //T &mut
    Optional(Box<Type<'a>>), //T?
    OptionalGarbage(Box<Type<'a>>), //T!

    //arrays
    FixedArray(Box<Type<'a>>, usize), // T[N]
    DynArray(Box<Type<'a>>), // T[]

    //user defined
    Named(&'a str), // Person, MyStruct etc

    //compound
    Union(Vec<Type<'a>>), // union(i32, f64)
}

pub struct Param<'a> {
    name: &'a str,
    type_: Type<'a>,
    mutable: bool,
}

pub struct MatchArm<'a> {
    match_to: Box<Expression<'a>>,
    body_block: Box<Expression<'a>>,
}
