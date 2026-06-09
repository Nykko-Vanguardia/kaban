use std::ops::Add;

use kaban_core::UIndex;

pub const U_NONE: UIndex = UIndex::MAX;

#[derive(Debug, Clone, Copy)]
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
    /// # main token: TokenIndex
    Identifier,
    /// # main token: TokenIndex
    IntLit,
    /// # main token: TokenIndex
    FloatLit,
    /// # main token: [
    /// # left: u32 = element count
    /// # right: ExtraIndex -> \[...element\]
    /// - extra\[right..right + N\] = NodeId\[N\] (expressions)
    ArrayLit,
    /// # main token: (
    /// # left: u32 = element count
    /// # right: ExtraIndex -> \[...element\]
    /// - extra\[right..right + N\] = NodeId\[N\] (expressions)
    TupleLit,
    /// # main token: true
    TrueLit,
    /// # main token: false
    FalseLit,
    /// # main token: TokenIndex
    Char8Lit,
    /// # main token: TokenIndex
    Char16Lit,
    /// # main token: TokenIndex
    Char32Lit,
    /// # main token: TokenIndex
    StringLit,
    // Undefined,
    // Garbage,
    /// # main token: TokenIndex
    Self_,
    /// # main token: enum
    /// eg. enum.Day
    AnonymousEnumlit,

    //OPERATORS-----------------------------

    //Arithmetic
    /// # main token: +
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    Add,
    /// # main token: -
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    Subtract,
    /// # main token: *
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    Multiply,
    /// # main token: /
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    Divide,
    /// # main token: %
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
    /// #left: NodeIndex = Expression
    ReferenceOf,
    /// #left: NodeIndex = Expression
    MutReferenceOf,
    /// #left: NodeIndex = Expression
    OwnershipOf,

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
    /// # left: NodeIndex = identifier (expression)
    /// # right: ExtraIndex -> \[arg_count, ...args\]
    /// - extra\[right\]: u32 = arg count
    /// - extra\[right + 1 .. right + 1 + N\] = NodeId\[N\] (arguments)
    GenericInstantiation,
    /// # main token = {
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
    ImplAccess,
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    UndefinedChainingAccess,
    /// # left: NodeIndex - Expression
    /// # right: NodeIndex - Expression
    Colon,
    /// # main token = . or :
    /// # left: NodeIndex = parent
    /// # right: ExtraIndex -> \[is_mutable, arg_count, ...args\]
    /// - extra\[right\]: 0 | 1 = 1 if mutable (: operator) 0 if not (. operator)
    /// - extra\[right + 1\]: u32 = arg count (N)
    /// - extra\[right + 2 .. right + 2 + N\] = NodeId\[N\] (arguments)
    MethodCall,
    /// # main token = . or :
    /// # left: NodeIndex = parent
    /// # right: ExtraIndex -> \[is_mutable, arg_count, generic_arg_count, ...args, ...generic_args\]
    /// - extra\[right\]: 0 | 1 = 1 if mutable (: operator) 0 if not (. operator)
    /// - extra\[right + 1\]: u32 = arg count (N)
    /// - extra\[right + 2\]: u32 = generic arg count (M)
    /// - extra\[right + 3 .. right + 3 + N\] = NodeId\[N\] (arguments)
    /// - extra\[right + 3 + N .. right + 3 + N + M\] = NodeId\[M\] (arguments)
    MethodWithGenericInstantiation,

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
    /// # main token: let
    /// # left: NodeIndex = IDENTIFIER BINDING NOT TOKEN!!!!
    /// # right: ExtraIndex -> \[type, expression\]
    /// - extra\[right\]: NodeIndex | U_NONE = Pointer to type
    /// - extra\[right + 1\] = Expression
    Let,
    /// # main token: const
    /// # left: TokenIndex = Identifier token
    /// # right: ExtraIndex -> \[is_pub?, type, expression\]
    /// - extra\[right\]: 1 | 0 = is public?
    /// - extra\[right + 1\]: NodeIndex= Pointer to type
    /// - extra\[right + 2\] = Expression
    Const,
    /// # main token = func
    /// # left: 1 | 0 = is_pub
    /// # right: ExtraIndex -> \[return_type, body, param_count, params...\]
    /// - extra\[right\]: NodeIndex | U_NONE = return_type
    /// - extra\[right + 1\]: NodeIndex = body
    /// - extra\[right + 2\]: UIndex = param_count
    /// - extra\[right + 3 .. right + 3 + N\]: NodeIndex = parameters
    FuncDeclWithNoGenerics,
    /// # main token = func
    /// # left: 1 | 0 = is_pub
    /// # right: ExtraIndex -> \[return_type, body, param_count, params...\]
    /// - extra\[right\]: NodeIndex | U_NONE = return_type
    /// - extra\[right + 1\]: NodeIndex = body
    /// - extra\[right + 2\]: UIndex = generic_param_count (N)
    /// - extra\[right + 3\]: UIndex = param_count (M)
    /// - extra\[right + 4 .. right + 4 + N\]: NodeIndex = generic parameters
    /// - extra\[right + 4 + N .. right + 4 + N + M\]: NodeIndex = parameters
    FuncDeclWithGenerics,
    /// # main token = func
    /// # left: 1 | 0 = is_pub
    /// # right: ExtraIndex -> \[return_type, body, param_count, params...\]
    /// - extra\[right\]: NodeIndex | U_NONE = return_type
    /// - extra\[right + 1\]: UIndex = param_count
    /// - extra\[right + 2 .. right + 2 + N\]: NodeIndex = parameters
    FuncNoBodyWithNoGenerics,
    /// # main token = func
    /// # left: 1 | 0 = is_pub
    /// # right: ExtraIndex -> \[return_type, body, param_count, params...\]
    /// - extra\[right\]: NodeIndex | U_NONE = return_type
    /// - extra\[right + 1\]: UIndex = generic_param_count (N)
    /// - extra\[right + 2\]: UIndex = param_count (M)
    /// - extra\[right + 3 .. right + 3 + N\]: NodeIndex = generic parameters
    /// - extra\[right + 3 + N .. right + 3 + N + M\]: NodeIndex = parameters
    FuncNoBodyWithGenerics,
    /// # left: NodeIndex | U_NONE = return value
    Return,
    /// # left: NodeIndex | U_NONE = pass value
    Pass,
    Break,
    Continue,
    /// # left: NodeIndex = condition (expression)
    /// # right: NodeIndex = block (Block)
    While,
    /// # left: NodeIndex = IDENTIFIER BINDING NOT TOKEN!!!!
    /// # right: ExtraIndex -> \[iterator, block\]
    /// - extra\[right\]: NodeIndex = Iterator
    /// - extra\[right + 1\]: NodeIndex = Block
    For,
    /// # main token = struct
    /// # left: 1 | 0 = is entire stuct pub?
    /// # right: ExtraIndex -> \[field_count, fields...\]
    /// - extra\[right\]: UIndex = number of fields (N)
    /// - extra\[right + 1..right + 1 + N\]: NodeIndex\[N\] = [StructFieldDecleration]
    StructDeclWithNoGeneric,
    /// # main token = struct
    /// # left: 1 | 0 = is entire struct pub?
    /// # right: ExtraIndex -> [generic_count, field_count, generics..., fields...]
    /// - extra\[right\]: UIndex = number of generic params (N)
    /// - extra\[right + 1\]: UIndex = number of template fields (M)
    /// - extra\[right + 2..right + 2 + N\]: NodeIndex\[N\] = \[GenericParam\]
    /// - extra\[right + 2 + N..right + 2 + N + M\]: NodeIndex\[M\] = \[StructFieldDecleration\]
    StructDeclWithGeneric,
    /// # main token = enum
    /// # left: 1 | 0 = is pub?
    /// # right: ExtraIndex -> \[enum_variant_count, enum_variants...\]
    /// - extra\[right\]: UIndex = number of enum_variants (N)
    /// - extra\[right + 1..right + 1 + N\]: NodeIndex\[N\] = [EnumVariantDecl]
    EnumDeclWithNoGeneric,
    /// # main token = enum
    /// # left: 1 | 0 = is pub?
    /// # right: ExtraIndex -> [generic_count, enum_variant_count, generics..., enum_variants...]
    /// - extra\[right\]: UIndex = number of generic params (N)
    /// - extra\[right + 1\]: UIndex = number of enum_variants (M)
    /// - extra\[right + 2..right + 2 + N\]: NodeIndex\[N\] = \[GenericParam\]
    /// - extra\[right + 2 + N..right + 2 + N + M\]: NodeIndex\[M\] = \[EnumVariantDecl\]
    EnumDeclWithGeneric,
    /// # main token = impl
    /// # left: 1 | 0 = is entire impl pub?
    /// # right: ExtraIndex -> \[self_type, statement_counts, ...statements\]
    /// - extra\[right\]: NodeIndex = self type
    /// - extra\[right + 1\]: UIndex = number of statements (N)
    /// - extra\[right + 2..right + 2 + N\]: NodeIndex\[N\] = Statements
    ImplDeclWithNoGeneric,
    /// # main token = impl
    /// # left: 1 | 0 = is entire impl pub?
    /// # right: ExtraIndex -> \[self_type, generic_count, statement_counts, ...generics, ...statements\]
    /// - extra\[right + 0\]: NodeIndex = self type (N)
    /// - extra\[right + 1\]: UIndex = number of generics (M)
    /// - extra\[right + 2\]: UIndex = number of statements (M)
    /// - extra\[right + 3..right + 3 + N\]: NodeIndex\[N\] = \[GenericParam\]
    /// - extra\[right + 3 + N..right + 3 + N + M\]: NodeIndex\[M\] = Statements
    ImplDeclWithGeneric,
    /// # main token = impl
    /// # left: 1 | 0 = is entire impl pub?
    /// # right: ExtraIndex -> \[self_type, interface, statement_counts, ...statements\]
    /// - extra\[right + 0\]: NodeIndex = self type
    /// - extra\[right + 1\]: NodeIndex = interface
    /// - extra\[right + 2\]: UIndex = number of statements (N)
    /// - extra\[right + 3..right + 3 + N\]: NodeIndex\[N\] = Statements
    ImplForDeclWithNoGeneric,
    /// # main token = struct
    /// # left: 1 | 0 = is entire impl pub?
    /// # right: ExtraIndex -> \[self_type, interface, generic_count, statement_counts, ...generics, ...statements\]
    /// - extra\[right + 0\]: NodeIndex = self type
    /// - extra\[right + 1\]: NodeIndex = inteface
    /// - extra\[right + 2\]: UIndex = number of generics (N)
    /// - extra\[right + 3\]: UIndex = number of statements (M)
    /// - extra\[right + 4..right + 4 + N\]: NodeIndex\[N\] = \[GenericParam\]
    /// - extra\[right + 4 + N..right + 4 + N + M\]: NodeIndex\[M\] = Statements
    ImplForDeclWithGeneric,
    /// # main token = interface
    /// # left: 1 | 0 = is entire impl pub?
    /// # right: ExtraIndex -> \[shape, statement_counts, ...statements\]
    /// - extra\[right + 0\]: NodeIndex | U_NONE = shape
    /// - extra\[right + 1\]: UIndex = number of statements (N)
    /// - extra\[right + 2..right + 2 + N\]: NodeIndex\[N\] = Statements
    InterfaceDeclWithNoGenerics,
    /// # main token = interface
    /// # left: 1 | 0 = is entire impl pub?
    /// # right: ExtraIndex -> \[shape, generic_count, statement_counts, ...generics, ...statements\]
    /// - extra\[right + 0\]: NodeIndex | U_NONE = shape
    /// - extra\[right + 1\]: UIndex = number of generics (N)
    /// - extra\[right + 2\]: UIndex = number of statements (M)
    /// - extra\[right + 3..right + 3 + N\]: NodeIndex\[N\] = \[GenericParam\]
    /// - extra\[right + 3 + N..right + 3 + N + M\]: NodeIndex\[M\] = Statements
    InterfaceDeclWithGenerics,

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
    /// T<i32, f64>
    /// # left: NodeIndex - Type(Recursive)
    /// # right: ExtraIndex -> \[arg_count, ...args\]
    /// - extra\[right\]: u32 = arg count
    /// - extra\[right + 1 .. right + 1 + N\] = NodeId\[N\] (Types)
    TypeWithGenerics,

    //user defined
    /// # left: TokenIndex - Name(Recursive)
    NamedType, // Person, MyStruct etc

    //compound
    /// # left: u32 = type count
    /// # right: ExtraIndex -> \[...types\]
    /// - extra\[right.. right + N\] = NodeIndex\[N\] (Types)
    Union, // union(i32, f64)
    // Result,
    /// # left: u32 = type count
    /// # right: ExtraIndex -> \[...types\]
    /// - extra\[right.. right + N\] = NodeIndex\[N\] (Types)
    TupleType,
    /// # left: u32 = field count (N)
    /// # right: ExtraIndex -> \[...field\]
    /// - extra\[right.. right + N\] = NodeIndex\[N\] (AnonymousStructFieldDecl)
    AnonymousStructType,
    /// # left: NodeIndex | U_NONE = return_type
    /// # right: ExtraIndex -> \[param_count, args...\]
    /// - extra\[right\]: UIndex = param_count
    /// - extra\[right + 1 .. right + 1 + N\]: NodeIndex = parameters
    /// NOTE: PARAM LEFT IS ALWAYS AN IDENTIFIER BINDING, NEVER DESTRUCTURS, ALWAYS IN THE FORM OF
    /// MUT OR NOT MUT THEN IDENTIFIER
    FuncType,
    /// # left: u32 = variant count
    /// # right: ExtraIndex -> \[...variants\]
    /// - extra\[right.. right + N\] = NodeIndex\[N\] \(variants\)
    AnonymousEnumType,

    //Generic Constaints
    /// # main token: Interface Name (Identifier)
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
    /// # main token = always the first token
    /// # left: NodeIndex = Identifier Binding (Expression)
    /// # right: NodeIndex = Type (Statement or Block)
    Params,
    //NOTE:
    ///NOTE: I might get rid of the right data. If you want to pass a copy of self, the dot
    ///operator is invalid. You must pass it explicitly. I also might change this to always have a
    ///modifier token. Self must be a pointer to the original object. Otherwise explicitly pass the
    ///copy
    ///
    /// # main token = self
    /// # left: TokenIndex = modifier token index (the & or &mut or * token)
    /// # right: 1 | 0 = Is mutable
    SelfParam,
    /// # main token = Identifier (Like T)
    /// # left: NodeIndex | U_NONE = GenericConstaint //NOTE: MIGHT REMOVE THIS
    GenericParam,
    /// This is different from [StructFieldInstantiation], this is for struct
    /// decleration (eg. struct Person {id: i32})
    /// # main_token: TokenIndex = Field Name (Identifier)
    /// # left: 1 | 0 = is pub?
    /// # right: NodeIndex = type
    StructFieldDecleration,
    /// # main token: Field Name (Identifier)
    /// # left: U_NONE
    /// # right: NodeIndex = Type
    AnonymousStructFieldDecl,
    /// This is different from [StructFieldDecleration], this is for struct
    /// decleration (eg. Person {id: i32})
    /// # main_token: TokenIndex = Field Name (Identifier)
    /// # left: NodeIndex = Expression
    StructFieldInstantiation,
    /// # main_token: TokenIndex = Identifier
    /// # left: NodeIndex | U_NONE = Type (could be none)
    EnumVariantDecl,
    /// # main token = =>
    /// # left: NodeIndex = Match Target (Expression)
    /// # right: NodeIndex = Then (Statement or Block)
    MatchArms,
    /// # main token: TokenIndex = the first |
    /// # left: u32 = match target count
    /// # right: ExtraIndex -> \[...match targets\]
    /// - extra\[right.. right + N\] = NodeIndex\[N\] (Match Targets)
    MultipleMatchTargets,

    ///SPECIAL -------------
    /// # main token: Token Index = Identifier token
    /// # left: 1 | 0 = Mutable or not
    IdentifierBinding, //Decided to added this for binding identifiers let mut x; its also
    //applicable to destructures such as let (mut x, mut y);
    /// # main token: TokenIndex = identifier token for the field name
    /// # left: NodeIndex = To [IdentifierBinding]
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

    /// # left: u32 = Expression
    CompTimeExpression,

    ///eg. if (x is type i32) or if (x is Day.Monday)
    ///just like binary expressions like 10 + 5
    // Is,
    /// eg. if \(x: time is Day.Monday\)
    /// # main token = is
    /// # left: NodeIndex = original expression (x) (Expression)
    /// # right: ExtraIndex -> \[...identifier binding, is_target\]
    /// - extra\[right\]: NodeIndex = Identifier binding (time)
    /// - extra\[right + 1\]: NodeIndex = Is target (Day.Monday)
    ToIs,
}

impl NodeTag {
    // pub fn is_token_leaf(&self) -> bool {
    //     matches!(
    //         self,
    //         NodeTag::Identifier
    //             | NodeTag::IntLit
    //             | NodeTag::FloatLit
    //             | NodeTag::Char8Lit
    //             | NodeTag::Char16Lit
    //             | NodeTag::Char32Lit
    //             | NodeTag::StringLit
    //     )
    // }

    pub fn is_leaf(&self) -> bool {
        matches!(
            self,
            NodeTag::Identifier
                | NodeTag::IntLit
                | NodeTag::FloatLit
                | NodeTag::TrueLit
                | NodeTag::FalseLit
                | NodeTag::Char8Lit
                | NodeTag::Char16Lit
                | NodeTag::Char32Lit
                | NodeTag::StringLit
                | NodeTag::Self_
        )
    }

    pub fn is_type(&self) -> bool {
        matches!(
            self,
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
            // NodeTag::Result
            NodeTag::TupleType |
            NodeTag::AnonymousStructType |
            NodeTag::FuncType |
            NodeTag::AnonymousEnumType
        )
    }

    pub fn is_atomic_type(&self) -> bool {
        matches!(
            self,
            NodeTag::I8
                | NodeTag::I16
                | NodeTag::I32
                | NodeTag::I64
                | NodeTag::F32
                | NodeTag::F64
                | NodeTag::U8
                | NodeTag::U16
                | NodeTag::U32
                | NodeTag::U64
                | NodeTag::USize
                | NodeTag::Bool
                | NodeTag::Void
                | NodeTag::C8
                | NodeTag::C16
                | NodeTag::C32
                | NodeTag::Undefined
                | NodeTag::Garbage
        )
    }

    pub fn is_simple_modifier_type(&self) -> bool {
        matches!(
            self,
            NodeTag::Pointer
                | NodeTag::Borrow
                | NodeTag::MutBorrow
                | NodeTag::Optional
                | NodeTag::OptionalGarbage
                | NodeTag::DynArrayType // NodeTag::FixedArray
                                        // NodeTag::Named |
                                        // NodeTag::Union |
                                        // NodeTag::Result
        )
    }

    /// These nodes don't require a semicolon terminator IF they are used as a statement
    /// AND if they ARE terminated with a }. Otherwise, the expression statement forces a semicolon.
    ///
    /// # For example
    /// ```kaban
    /// if (true) x += 1; // Requires a semicolon because expression statements expect semicolons
    /// x = 10 + (if (true) pass 10) + 20; // Doesn't require semicolon because if itself doesn't need a semicolon
    /// if (true) { x += 1 } // Doesn't require a semicolon because the expression statement semicolon is overridden by the }
    /// let x = if (true) { x += 1 }; // Requires a semicolon that belongs to let
    /// ```
    pub fn can_omit_semicolon(&self) -> bool {
        matches!(
            self,
            NodeTag::If
                | NodeTag::Match
                | NodeTag::Block
                | NodeTag::While
                | NodeTag::For
                | NodeTag::CompTimeExpression // NodeTag::DoWhile // you need a semicolon here
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

impl Add<u8> for TokenIndex {
    type Output = TokenIndex;
    fn add(self, rhs: u8) -> Self::Output {
        TokenIndex(self.0 + rhs as u32)
    }
}

impl ToOption for TokenIndex {
    fn to_option(self) -> Option<Self> {
        match self.0 {
            U_NONE => None,
            _ => Some(self),
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
            _ => Some(self),
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
        unsafe { std::mem::transmute::<&[NodeIndex], &[UIndex]>(self.as_slice()) }
    }
}

impl UIndexVec for &[NodeIndex] {
    fn uindex_slice(&self) -> &[UIndex] {
        unsafe { std::mem::transmute::<&[NodeIndex], &[UIndex]>(self) }
    }
}

pub trait UIndexVec {
    fn uindex_slice(&self) -> &[UIndex];
}

impl<'a> NodeIndexVec<'a> for &[UIndex] {
    fn node_index_slice(&self) -> &'a [NodeIndex] {
        unsafe { std::mem::transmute::<&[UIndex], &[NodeIndex]>(self) }
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
            _ => Some(self),
        }
    }
}

pub trait ToOption: Sized {
    fn to_option(self) -> Option<Self>;
}
