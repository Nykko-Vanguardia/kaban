use kaban_core::{SourceIndex};

#[repr(transparent)]
pub struct TokenIndex(pub SourceIndex);
#[derive(Debug)]
#[repr(transparent)]
pub struct NodeIndex(pub SourceIndex);
#[repr(transparent)]
pub struct DataIndex(pub SourceIndex);
#[repr(transparent)]
pub struct ExtraIndex(pub SourceIndex);

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum NodeTag {
    //ATOMS--------------------

    /// # left: TokenIndex
    Identifier,
    /// # left: TokenIndex
    IntLit,
    /// # left: TokenIndex
    FloatLit,
    /// # left: u32 = element count
    /// # right: ExtraIndex -> \[...element\]
    /// - extra\[right..right + N\] = NodeId\[N\] (expressions)
    ArrayLit,
    /// # left : 0 | 1 - 1 being true, 0 being false
    BoolLit,
    /// # left: TokenIndex
    Char8Lit,
    /// # left: TokenIndex
    Char16Lit,
    /// # left: TokenIndex
    Char32Lit,
    /// # left: TokenIndex
    StringLit,
    // Undefined,
    // Garbage,
    Self_,

    //OPERATORS-----------------------------

    //Arithmetic
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    Add,
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    Subtract,
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    Multiply,
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    Divide,
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    Modulo,

    //Comparison
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    Equal,
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    NotEqual,
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    Less,
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    Greater,
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    LessEqual,
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    GreaterEqual,

    //Logical
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    And,
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    Or,
    
    //Bitise Operations
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    BAnd,
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    BOr,
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    XOr,
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    LeftShift,
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    RightShift,
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    UnsignedRightShift,

    //Prefix Unary
    /// # left: NodeIndex - Expression
    Negative,
    /// # left: NodeIndex - Expression
    Not,
    /// # left: NodeIndex - Expression
    BNot,
    /// # left: NodeIndex = Class Type
    /// # right: ExtraIndex -> \[name_id, arg_count, ...args\]
    /// - extra\[right\]: NodeIndex | u32::Max = name or anonymous constructor
    /// - extra\[right + 1\] = arg count (N)
    /// - extra\[right + 2 .. right + 2 + N\] = NodeId\[N\] (arguments)
    New,
    /// # left: NodeIndex = Class Type
    /// # right: ExtraIndex -> \[name_id, arg_count, ...args\]
    /// - extra\[right\]: NodeIndex | u32::Max = name or anonymous constructor
    /// - extra\[right + 1\] = arg count (N)
    /// - extra\[right + 2 .. right + 2 + N\] = NodeId\[N\] (arguments)
    Destruct,

    //PostfixUnary
    /// # left: NodeIndex - Expression
    Deref,
    /// # left: NodeIndex - Expression
    Bang,
    /// # left: NodeIndex - Expression
    Question,
    /// # left: NodeIndex = identifier (expression)
    /// # right: ExtraIndex -> \[arg_count, ...args\]
    /// - extra\[right\]: u32 = arg count
    /// - extra\[right + 1 .. right + 1 + N\] = NodeId\[N\] (arguments)
    FuncCall,
    /// # left: NodeIndex = identifier (expression)
    /// # right: ExtraIndex -> \[safe_bool, index_number\]
    /// - extra\[right\]: 0 | 1 = 1 if safe, 0 if not
    /// - extra\[right + 1\] = NodeId\[N\] (index by)
    Index,

    //MemberAccess
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    Dot,
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    ExclamationDot,
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    QuestionDot,
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    QuestionQuestionDot,
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    Colon,
    /// # left: NodeIndex = parent
    /// # right: ExtraIndex -> \[method_name_id, is_mutable, arg_count, ...args\]
    /// - extra\[right\]: NodeIndex | u32::Max = name or anonymous constructor
    /// - extra\[right + 1\]: 0 | 1 = 1 if mutable (: operator) 0 if not (. operator)
    /// - extra\[right + 2\]: u32 = arg count (N)
    /// - extra\[right + 3 .. right + 3 + N\] = NodeId\[N\] (arguments)
    MethodCall,

    //Special
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    UndefinedCoalescing,
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Type
    As,


    //STATEMENTS (No returns)------
    Let,
    FuncDecl,
    Return,
    Pass,
    Break,
    Continue,
    ExpressionStatement,
    Assignment,
    PlusAssignment,
    MinusAssignment,
    MultiplyAssignment,
    ModuloAssignment,
    While,
    For,
    StructDecl,
    ClassDecl,
    TypeAliasDecl,
    EnumDecl,

    //STATEMENT LIKE EXPRESSIONS-------
    Block,
    If,
    Match,
    AnonymousFuncDecl, //let x: func(i32, f64) -> i32 = func(x, y) { return x + y };
    DoWhile, //dunno if i should keep this, do while like loop is safe to pass values because they
             //will always run
    AnonymousStructDecl,
    AnonymousClassDecl, //Not sure yet

    //OTHERS-------------
    Params,
    MatchArms,

    //TYPES
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

    /// # left: NodeIndex - Type(Recursive)
    Pointer, //T*
    /// # left: NodeIndex - Type(Recursive)
    Borrow, //T&
    /// # left: NodeIndex - Type(Recursive)
    MutBorrow, //T &mut
    /// # left: NodeIndex - Type(Recursive)
    Optional, //T?
    /// # left: NodeIndex - Type(Recursive)
    OptionalGarbage, //T!

    //arrays
    /// # left: NodeIndex - Type(Recursive)
    /// # right: NodeIndex - Expression (Size) eg. T[10 + 1]
    FixedArray, // T[N]
    /// # left: NodeIndex - Type(Recursive)
    DynArray, // T[]

    //user defined
    /// # left: TokenIndex - Name(Recursive)
    Named, // Person, MyStruct etc

    //compound
    /// # left: u32 = arg count
    /// # right: ExtraIndex -> \[...args\]
    /// - extra\[right.. right + N\] = NodeIndex\[N\] (Types)
    Union, // union(i32, f64)
    //TODO:
    Result,
}

#[derive(Debug)]
pub struct NodeData {
    pub left: SourceIndex,
    pub right: SourceIndex,
}


impl TokenIndex {
    pub fn some(self) -> Option<TokenIndex> {
        Some(self)
    }
}
impl NodeIndex {
    pub fn some(self) -> Option<NodeIndex> {
        Some(self)
    }
}
impl DataIndex {
    pub fn some(self) -> Option<DataIndex> {
        Some(self)
    }
}
impl ExtraIndex {
    pub fn some(self) -> Option<ExtraIndex> {
        Some(self)
    }
}

impl SourceIndexVec for Vec<NodeIndex> {
    fn source_index(&self) -> &[SourceIndex] {
        unsafe { 
            std::mem::transmute::<&[NodeIndex], &[SourceIndex]>(self.as_slice()) 
        }
    }
}

pub trait SourceIndexVec {
    fn source_index(&self) -> &[SourceIndex];
}

// #[derive(Debug)]
// pub enum Statement {
//     Let {
//         mutable: bool,
//         name: SourceSpan,
//         let_type: Option<Type>,
//         assignment: Expression,
//     },
//
//     FuncDecl {
//         public: bool,
//         comptime: bool,
//         params: Vec<Param>,
//         name: SourceSpan,
//         return_type: Type,
//         body_block: Expression,
//     },
//
//     Return(Expression),
//     Pass(Expression),
//     Break,
//     Continue,
//
//     ExpressionStatement(Expression),
// }
//
// #[derive(Debug)]
// pub enum Expression {
//     IntLit(SourceSpan),
//     FloatLit(SourceSpan),
//     Identifier(SourceSpan),
//     ArrayLit(Vec<Expression>),
//     BoolLit(SourceSpan),
//     Char8Lit(SourceSpan),
//     Char16Lit(SourceSpan),
//     Char32Lit(SourceSpan),
//     StringLit(SourceSpan),
//     Undefined,
//     Garbage,
//     Self_,
//
//     Block {
//         statements: Vec<Statement>,
//         value: Option<Box<Expression>>
//     },
//     If {
//         condition: Box<Expression>,
//         then_block: Box<Expression>,
//         else_block: Option<Box<Expression>>,
//     },
//     Match {
//         subject: Box<Expression>,
//         arms: Vec<MatchArm>,
//     },
//     ArithmeticOperation {
//         left: Box<Expression>,
//         right: Box<Expression>,
//         operator: operator::Arithmetic,
//     },
//     ComparisonOperation {
//         left: Box<Expression>,
//         right: Box<Expression>,
//         operator: operator::Comparison,
//     },
//     LogicalOperation {
//         left: Box<Expression>,
//         right: Box<Expression>,
//         operator: operator::Logical,
//     },
//     BinaryOperation {
//         left: Box<Expression>,
//         right: Box<Expression>,
//         operator: operator::BitwiseBinary,
//     },
//     MemberAccess {
//         parent: Box<Expression>,
//         child: Box<Expression>,
//         operator: operator::MemberAccess,
//     },
//     IndexOperation {
//         parent: Box<Expression>,
//         index: Box<Expression>,
//         safe: bool,
//     },
//     UndefinedCoalescing {
//         possibly_undefined: Box<Expression>,
//         default: Box<Expression>,
//     },
//     TypeCasting {
//         value: Box<Expression>,
//         type_: Type,
//     },
//     PrefixUnaryOperation {
//         operand: Box<Expression>,
//         operator: operator::PrefixUnary,
//     },
//     PostfixUnaryOperation {
//         operand: Box<Expression>,
//         operator: operator::PostfixUnary,
//     },
//     FunctionCall {
//         callee: Box<Expression>,
//         args: Vec<Expression>,
//     },
//     MethodCall {
//         parent: Box<Expression>,
//         method_name: SourceSpan,
//         args: Vec<Expression>,
//         mutable_self: bool,
//     }
// }
//
// impl Expression {
//     pub fn to_box(self) -> Box<Expression> {
//         Box::new(self)
//     }
//
//     pub fn to_some(self) -> Option<Expression> {
//         Some(self)
//     }
// }
//
// #[derive(Debug)]
// pub enum Type {
//     //primitives
//     I8,
//     I16,
//     I32,
//     I64,
//     F32,
//     F64,
//     U8,
//     U16,
//     U32,
//     U64,
//     USize,
//     Bool,
//     Void,
//     C8,
//     C16,
//     C32,
//     Undefined,
//     Garbage,
//
//     //modifiers — recursive
//     Pointer(Box<Type>), //T*
//     Borrow(Box<Type>), //T&
//     MutBorrow(Box<Type>), //T &mut
//     Optional(Box<Type>), //T?
//     OptionalGarbage(Box<Type>), //T!
//
//     //arrays
//     FixedArray{ type_: Box<Type>, size: Box<Expression> }, // T[N]
//     DynArray(Box<Type>), // T[]
//
//     //user defined
//     Named(SourceSpan), // Person, MyStruct etc
//
//     //compound
//     Union(Vec<Type>), // union(i32, f64)
//     //TODO:
//     Result,
// }
//
// impl Type {
//     pub fn to_box(self) -> Box<Type> {
//         Box::new(self)
//     }
//
//     pub fn to_some(self) -> Option<Type> {
//         Some(self)
//     }
// }
//
// #[derive(Debug)]
// pub struct Param {
//     name: SourceSpan,
//     type_: Type,
//     mutable: bool,
// }
//
// #[derive(Debug)]
// pub struct MatchArm {
//     match_to: Box<Expression>,
//     body_block: Box<Expression>,
// }
