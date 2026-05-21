use kaban_core::{UIndex};

pub const U_NONE: UIndex = UIndex::MAX;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct TokenIndex(pub UIndex);
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct NodeIndex(pub UIndex);
#[repr(transparent)]
pub struct ExtraIndex(pub UIndex);
// #[repr(transparent)]
// pub struct DataIndex(pub UIndex);

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
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
    /// # left: u32 = element count
    /// # right: ExtraIndex -> \[...element\]
    /// - extra\[right..right + N\] = NodeId\[N\] (expressions)
    TupleLit,
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

    //Range
    ExclusiveRange,
    InclusiveRange,

    //Prefix Unary
    /// # left: NodeIndex - Expression
    Negative,
    /// # left: NodeIndex - Expression
    Not,
    /// # left: NodeIndex - Expression
    BNot,
    /// # left: NodeIndex = Class Type
    /// # right: ExtraIndex -> \[name_id, arg_count, ...args\]
    /// - extra\[right\]: NodeIndex | [UNONE] = name or anonymous constructor
    /// - extra\[right + 1\] = arg count (N)
    /// - extra\[right + 2 .. right + 2 + N\] = NodeId\[N\] (arguments)
    New,
    /// # left: NodeIndex = Class Type
    /// # right: ExtraIndex -> \[name_id, arg_count, ...args\]
    /// - extra\[right\]: NodeIndex | [UNONE] = name or anonymous constructor
    /// - extra\[right + 1\] = arg count (N)
    /// - extra\[right + 2 .. right + 2 + N\] = NodeId\[N\] (arguments)
    Destruct,

    //PostfixUnary
    /// # left: NodeIndex - Expression
    Deref,
    /// # left: NodeIndex - Expression
    PanicIfErrOrNone,
    /// # left: NodeIndex - Expression
    BubbleIfErrOrNone,
    /// # left: NodeIndex = identifier (expression)
    /// # right: ExtraIndex -> \[arg_count, ...args\]
    /// - extra\[right\]: u32 = arg count
    /// - extra\[right + 1 .. right + 1 + N\] = NodeId\[N\] (arguments)
    FuncCall,
    /// # left: NodeIndex | U_NONE = identifier/struct name (expression)
    /// # right: ExtraIndex -> \[field_instantiation_count, ...field_instantiation\]
    /// - extra\[right\]: u32 = field instantiation count
    /// - extra\[right + 1 .. right + 1 + N\]: NodeIndex = field_instantiations
    StructInstantiation,
    /// # left: NodeIndex = identifier (expression)
    /// # right: ExtraIndex -> \[safe_bool, index_number\]
    /// - extra\[right\]: 0 | 1 = 1 if safe, 0 if not
    /// - extra\[right + 1\] = NodeId\[N\] (index by)
    Index,

    //MemberAccess
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    MemberAccess,
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    UndefinedChainingAccess,
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    Colon,
    /// # left: NodeIndex = parent
    /// # right: ExtraIndex -> \[method_name_id, is_mutable, arg_count, ...args\]
    /// - extra\[right\]: NodeIndex = name
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

    //Assignment (NO RETURNS)
    Assignment,
    PlusAssignment,
    MinusAssignment,
    MultiplyAssignment,
    DivideAssignment,
    ModuloAssignment,



    //STATEMENTS (No returns)------
    /// # left: NodeIndex = IDENTIFIER BINDING NOT TOKEN!!!!
    /// # right: ExtraIndex -> \[type, expression\]
    /// - extra\[right\]: NodeIndex | U_NONE = Pointer to type
    /// - extra\[right + 1\] = Expression
    Let,
    FuncDecl,
    /// # left: NodeIndex | U_NONE = return value
    Return,
    /// # left: NodeIndex | U_NONE = pass value
    Pass,
    Break,
    Continue,
    ExpressionStatement,
    /// # left: NodeIndex = condition (expression)
    /// # right: NodeIndex = block (Block)
    While,
    /// # left: NodeIndex = IDENTIFIER BINDING NOT TOKEN!!!!
    /// # right: ExtraIndex -> \[iterator, block\]
    /// - extra\[right\]: NodeIndex = Iterator
    /// - extra\[right + 1\]: NodeIndex = Block
    For,
    /// # left: TokenIndex = Name (Token)
    /// # right: ExtraIndex -> \[Pub, field_count, fields...\]
    /// - extra\[right\]: 1 | 0 = is entire stuct pub?
    /// - extra\[right + 1\]: UIndex = number of fields (N)
    /// - extra\[right + 2..right + 2 + N\]: NodeIndex\[N\] = [StructFieldDecleration]
    StructDeclWithNoGeneric,
    /// # left: TokenIndex = Name (Token)
    /// # right: ExtraIndex -> [Pub, generic_count, field_count, generics..., fields...]
    /// - extra\[right\]: 1 | 0 = is entire struct pub?
    /// - extra\[right + 1\]: UIndex = number of generic params (N)
    /// - extra\[right + 2\]: UIndex = number of template fields (M)
    /// - extra\[right + 3..right + 3 + N\]: NodeIndex\[N\] = \[GenericParam\]
    /// - extra\[right + 3 + N..right + 3 + N + M\]: NodeIndex\[M\] = \[StructFieldDecleration\]
    StructDeclWithGeneric,
    ClassDecl,
    TypeAliasDecl,
    EnumDecl,

    //STATEMENT LIKE EXPRESSIONS-------
    /// # left: u32 = statements count
    /// # right: ExtraIndex -> \[...statements\]
    /// - extra\[right..right + N\] = NodeIndex\[N\] (statements)
    Block,
    /// # left: NodeIndex = condition (expression)
    /// # right: ExtraIndex -> \[then, else\]
    /// - extra\[right\]: NodeIndex = then (Block)
    /// - extra\[right + 1\]: NodeIndex | [UNONE] = else (Block or If)
    If,
    /// # left: NodeIndex = target (expression)
    /// # right: ExtraIndex -> \[arms_count, arms\]
    /// - extra\[right\]: UIndex = arms_count (N)
    /// - extra\[right + 1 .. right + 1 + N\]: NodeIndex = arms
    Match,
    /// # left: NodeIndex = Body
    /// # right: ExtraIndex -> \[return_type, param_count, args...\]
    /// - extra\[right\]: NodeIndex | U_NONE = return_type
    /// - extra\[right + 1\]: UIndex = param_count
    /// - extra\[right + 2 .. right + 2 + N\]: NodeIndex = parameters
    AnonymousFuncDecl, //let x: func(i32, f64) -> i32 = func(x, y) { return x + y };
    
    //NOTE: THIS WAS REMOVED, blurry chance this gets added back
    /// # left: NodeIndex = Body
    /// # right: ExtraIndex -> [return_type, generic_count, param_count, generics..., parameters...]
    /// - extra\[right\]: NodeIndex | U_NONE = return_type
    /// - extra\[right + 1\]: UIndex = number of generic params (N)
    /// - extra\[right + 2\]: UIndex = number of parameters (M)
    /// - extra\[right + 3..right + 3 + N\]: NodeIndex\[N\] = \[GenericParam\]
    /// - extra\[right + 3 + N..right + 3 + N + M\]: NodeIndex\[M\] = \[FuncParameterDeclaration\]
    // AnonymousFuncDeclWithGenerics,
    /// # left: NodeIndex = condition (expression)
    /// # right: NodeIndex = block (Block)
    DoWhile, //dunno if i should keep this, do while like loop is safe to pass values because they
             //will always run
    // AnonymousClassDecl, //Not sure yet


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
    FixedArrayType, // T[N]
    /// # left: NodeIndex - Type(Recursive)
    DynArrayType, // T[]

    //user defined
    /// # left: TokenIndex - Name(Recursive)
    NamedType, // Person, MyStruct etc

    //compound
    /// # left: u32 = arg count
    /// # right: ExtraIndex -> \[...args\]
    /// - extra\[right.. right + N\] = NodeIndex\[N\] (Types)
    Union, // union(i32, f64)
    //TODO:
    Result,
    
    //Generic Constaints
    /// # left: TokenIndex = Interface (Identifier)
    /// eg. <T: impl Serializable>
    InterfaceConstraint,
    /// # left: NodeIndex = Interface Constaint or Type
    /// # right: NodeIndex = Interface Constaint or Type
    /// eg. <T: impl Serializable & impl Printable>
    AndGenericConstaint,
    /// # left: NodeIndex = Interface Constaint or Type
    /// # right: NodeIndex = Interface Constaint or Type
    /// eg. <T: impl Serializable | impl Printable>
    OrGenericConstaint,

    //OTHERS-------------
    /// # left: NodeIndex = Identifier Binding (Expression)
    /// # right: NodeIndex = Type (Statement or Block)
    Params,
    /// # left: TokenIndex = Identifier (like T)
    /// # right: NodeIndex | U_NONE = GenericConstaint
    GenericParam,
    /// This is different from [StructFieldInstantiation], this is for struct
    /// decleration (eg. struct Person {id: i32})
    /// # left: TokenIndex = Field Name (Identifier)
    /// # right: NodeIndex = Expression
    /// # right: ExtraIndex -> \[is pub?, type\]
    /// - extra\[right\]: 1 | 0 = is pub?
    /// - extra\[right + 1\]: NodeIndex = type
    StructFieldDecleration,
    /// This is different from [StructFieldDecleration], this is for struct
    /// decleration (eg. Person {id: i32})
    /// # left: TokenIndex = Field Name (Identifier)
    /// # right: NodeIndex = Expression
    StructFieldInstantiation,
    /// # left: NodeIndex = Match Target (Expression)
    /// # right: NodeIndex = Then (Statement or Block)
    MatchArms,
    ///SPECIAL -------------
    /// # left: TokenIndex = Identifier token
    /// # right: 1 | 0 = Mutable or not
    IdentifierBinding, //Decided to added this for binding identifiers let mut x; its also
                       //applicable to destructures such as let (mut x, mut y);

    /// # left: TokenIndex = Identifier token for the field name
    /// # right: NodeIndex = To [IdentifierBinding]
    /// this is for cases like let {mut x: new_name, y};
    /// left is the token to x, right is an identifier node containing new_name and mut true
    /// in the case of y, its automatically added to identifier binding
    StructDestructureBinding,
    /// # left: u32 = arg count
    /// # right: ExtraIndex -> \[...args\]
    /// - extra\[right.. right + N\] = NodeIndex\[N\] (Identifiers)
    StructDestructure,
    /// # left: u32 = arg count
    /// # right: ExtraIndex -> \[...args\]
    /// - extra\[right.. right + N\] = NodeIndex\[N\] (Identifiers)
    TupleDestructure,
    /// # left: u32 = arg count
    /// # right: ExtraIndex -> \[...args\]
    /// - extra\[right.. right + N\] = NodeIndex\[N\] (Identifiers)
    ArrayDestructure,
}

impl NodeTag {
    /// Returns true if token is a leaf node just containing a token index
    /// This means bool lit is not true here since it's left side contains 0 or 1
    /// Self is also not included here since it doesnt contain anything
    pub fn is_token_leaf(&self) -> bool {
        matches!(self,
            NodeTag::Identifier |
            NodeTag::IntLit |
            NodeTag::FloatLit |
            NodeTag::Char8Lit |
            NodeTag::Char16Lit |
            NodeTag::Char32Lit |
            NodeTag::StringLit
        )
    }

    /// Returns true if token is a leaf node
    /// Bool lits are detected here, to target pure token leafs, use is_token_leaf
    pub fn is_leaf(&self) -> bool {
        matches!(self,
            NodeTag::Identifier |
            NodeTag::IntLit |
            NodeTag::FloatLit |
            NodeTag::BoolLit |
            NodeTag::Char8Lit |
            NodeTag::Char16Lit |
            NodeTag::Char32Lit |
            NodeTag::StringLit |
            NodeTag::Self_
        )
    }

    pub fn is_type(&self) -> bool {
        matches!(self,
            NodeTag::I8 |
            NodeTag::I16 |
            NodeTag::I32 |
            NodeTag::I64 |
            NodeTag::F32 |
            NodeTag::F64 |
            NodeTag::U8 |
            NodeTag::U16 |
            NodeTag::U32 |
            NodeTag::U64 |
            NodeTag::USize |
            NodeTag::Bool |
            NodeTag::Void |
            NodeTag::C8 |
            NodeTag::C16 |
            NodeTag::C32 |
            NodeTag::Undefined |
            NodeTag::Garbage |
            NodeTag::Pointer |
            NodeTag::Borrow |
            NodeTag::MutBorrow |
            NodeTag::Optional |
            NodeTag::OptionalGarbage |
            NodeTag::FixedArrayType |
            NodeTag::DynArrayType |
            NodeTag::Union |
            NodeTag::Result
        )
    }

    pub fn is_atomic_type(&self) -> bool {
        matches!(self,
            NodeTag::I8 |
            NodeTag::I16 |
            NodeTag::I32 |
            NodeTag::I64 |
            NodeTag::F32 |
            NodeTag::F64 |
            NodeTag::U8 |
            NodeTag::U16 |
            NodeTag::U32 |
            NodeTag::U64 |
            NodeTag::USize |
            NodeTag::Bool |
            NodeTag::Void |
            NodeTag::C8 |
            NodeTag::C16 |
            NodeTag::C32 |
            NodeTag::Undefined |
            NodeTag::Garbage
        )
    }

    pub fn is_simple_modifier_type(&self) -> bool {
        matches!(self,
            NodeTag::Pointer |
            NodeTag::Borrow |
            NodeTag::MutBorrow |
            NodeTag::Optional |
            NodeTag::OptionalGarbage |
            NodeTag::DynArrayType
            // NodeTag::FixedArray 
            // NodeTag::Named |
            // NodeTag::Union |
            // NodeTag::Result
        )
    }

    pub fn doesnt_require_semicolon(&self) -> bool {
        matches!(self,
            NodeTag::If |
            NodeTag::Match |
            NodeTag::Block |
            NodeTag::While |
            NodeTag::For
            // NodeTag::DoWhile // you need a semicolon here
        )
    }

}

#[derive(Debug, Clone, Copy)]
pub struct NodeData {
    pub left: UIndex,
    pub right: UIndex,
}


impl TokenIndex {
    pub fn some(self) -> Option<TokenIndex> {
        Some(self)
    }

}

impl ToOption for TokenIndex {
    fn to_option(self) -> Option<Self> {
        match self.0 {
            U_NONE => None,
            _ => Some(self)
        }
    }
}
impl NodeIndex {
    pub fn some(self) -> Option<NodeIndex> {
        Some(self)
    }
}

impl ToOption for NodeIndex {
    ///Returns None if U_NONE, else it returns Some
    fn to_option(self) -> Option<Self> {
        match self.0 {
            U_NONE => None,
            _ => Some(self)
        }
    }
}

impl OptionalNode for Option<NodeIndex> {
    ///returns UNONE if none, else returns itself
    #[inline(always)]
    fn to_index_or_u_none(self) -> UIndex {
        if let Some(index) = self {
            index.0
        } else {
            U_NONE
        }
    }
}

pub trait OptionalNode {
    fn to_index_or_u_none(self) -> UIndex;
}
// impl DataIndex {
//     pub fn some(self) -> Option<DataIndex> {
//         Some(self)
//     }
// }
// impl ExtraIndex {
//     pub fn some(self) -> Option<ExtraIndex> {
//         Some(self)
//     }
// }

impl UIndexVec for Vec<NodeIndex> {
    fn uindex_slice(&self) -> &[UIndex] {
        unsafe { 
            std::mem::transmute::<&[NodeIndex], &[UIndex]>(self.as_slice()) 
        }
    }
}

impl UIndexVec for &[NodeIndex] {
    fn uindex_slice(&self) -> &[UIndex] {
        unsafe { 
            std::mem::transmute::<&[NodeIndex], &[UIndex]>(self) 
        }
    }
}

pub trait UIndexVec {
    fn uindex_slice(&self) -> &[UIndex];
}

impl<'a> NodeIndexVec<'a> for &[UIndex] {
    fn node_index_slice(&self) -> &'a [NodeIndex] {
        unsafe {
            std::mem::transmute::<&[UIndex], &[NodeIndex]>(self)
        }
    }
}

pub trait NodeIndexVec<'a> {
    fn node_index_slice(&self) -> &'a [NodeIndex];
}

impl ToWrapper for UIndex {
    #[inline(always)]
    fn node_index(self) -> NodeIndex {
        NodeIndex(self)
    }

    #[inline(always)]
    fn token_index(self) -> TokenIndex {
        TokenIndex(self)
    }

}

pub trait ToWrapper {
    fn node_index(self) -> NodeIndex;
    #[allow(dead_code)]
    fn token_index(self) -> TokenIndex;
}

impl ToOption for UIndex {
    fn to_option(self) -> Option<Self> {
        match self {
            U_NONE => None,
            _ => Some(self)
        }
    }
}

pub trait ToOption: Sized {
    fn to_option(self) -> Option<Self>;
}
