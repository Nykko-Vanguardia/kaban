use kaban_core::SourceSpan;
use crate::operator;

#[derive(Debug)]
pub enum Statement {
    Let {
        mutable: bool,
        name: SourceSpan,
        let_type: Option<Type>,
        assignment: Expression,
    },

    FuncDecl {
        public: bool,
        comptime: bool,
        params: Vec<Param>,
        name: SourceSpan,
        return_type: Type,
        body_block: Expression,
    },

    Return(Expression),
    Pass(Expression),
    Break,
    Continue,

    ExpressionStatement(Expression),
}

#[derive(Debug)]
pub enum Expression {
    IntLit(SourceSpan),
    FloatLit(SourceSpan),
    Identifier(SourceSpan),
    ArrayLit(Vec<Expression>),
    BoolLit(SourceSpan),
    Char8Lit(SourceSpan),
    Char16Lit(SourceSpan),
    Char32Lit(SourceSpan),
    StringLit(SourceSpan),
    Undefined,
    Garbage,
    Self_,

    Block {
        statements: Vec<Statement>,
        value: Option<Box<Expression>>
    },
    If {
        condition: Box<Expression>,
        then_block: Box<Expression>,
        else_block: Option<Box<Expression>>,
    },
    Match {
        subject: Box<Expression>,
        arms: Vec<MatchArm>,
    },
    ArithmeticOperation {
        left: Box<Expression>,
        right: Box<Expression>,
        operator: operator::Arithmetic,
    },
    ComparisonOperation {
        left: Box<Expression>,
        right: Box<Expression>,
        operator: operator::Comparison,
    },
    LogicalOperation {
        left: Box<Expression>,
        right: Box<Expression>,
        operator: operator::Logical,
    },
    BinaryOperation {
        left: Box<Expression>,
        right: Box<Expression>,
        operator: operator::BitwiseBinary,
    },
    MemberAccess {
        parent: Box<Expression>,
        child: Box<Expression>,
        operator: operator::MemberAccess,
    },
    IndexOperation {
        parent: Box<Expression>,
        index: Box<Expression>,
        safe: bool,
    },
    UndefinedCoalescing {
        possibly_undefined: Box<Expression>,
        default: Box<Expression>,
    },
    TypeCasting {
        value: Box<Expression>,
        type_: Type,
    },
    PrefixUnaryOperation {
        operand: Box<Expression>,
        operator: operator::PrefixUnary,
    },
    PostfixUnaryOperation {
        operand: Box<Expression>,
        operator: operator::PostfixUnary,
    },
    FunctionCall {
        callee: Box<Expression>,
        args: Vec<Expression>,
    },
    MethodCall {
        parent: Box<Expression>,
        method_name: SourceSpan,
        args: Vec<Expression>,
        mutable_self: bool,
    }
}

impl Expression {
    pub fn to_box(self) -> Box<Expression> {
        Box::new(self)
    }

    pub fn to_some(self) -> Option<Expression> {
        Some(self)
    }
}

#[derive(Debug)]
pub enum Type {
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
    Pointer(Box<Type>), //T*
    Borrow(Box<Type>), //T&
    MutBorrow(Box<Type>), //T &mut
    Optional(Box<Type>), //T?
    OptionalGarbage(Box<Type>), //T!

    //arrays
    FixedArray{ type_: Box<Type>, size: Box<Expression> }, // T[N]
    DynArray(Box<Type>), // T[]

    //user defined
    Named(SourceSpan), // Person, MyStruct etc

    //compound
    Union(Vec<Type>), // union(i32, f64)
}

impl Type {
    pub fn to_box(self) -> Box<Type> {
        Box::new(self)
    }

    pub fn to_some(self) -> Option<Type> {
        Some(self)
    }
}

#[derive(Debug)]
pub struct Param {
    name: SourceSpan,
    type_: Type,
    mutable: bool,
}

#[derive(Debug)]
pub struct MatchArm {
    match_to: Box<Expression>,
    body_block: Box<Expression>,
}
