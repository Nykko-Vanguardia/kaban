pub enum Operator {
    Arithmetic(Arithmetic),
    Comparison(Comparison),
    Logical(Logical),
    BitwiseBinary(BitwiseBinary),
    PrefixUnary(PrefixUnary),
    PostfixUnary(PostfixUnary),
    MemberAccess(MemberAccess),
    Special(Special),
    FuncCall,
}

impl HasPrecedence for Operator {
    fn precedence(&self) -> u8 {
        match self {
            Operator::Arithmetic(op) => op.precedence(),
            Operator::Comparison(op) => op.precedence(),
            Operator::Logical(op) => op.precedence(),
            Operator::BitwiseBinary(op) => op.precedence(),
            Operator::PrefixUnary(op) => op.precedence(),
            Operator::PostfixUnary(op) => op.precedence(),
            Operator::MemberAccess(op) => op.precedence(),
            Operator::Special(op) => op.precedence(),
            Operator::FuncCall => 12,
        }
    }
}

pub enum Arithmetic {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

impl HasPrecedence for Arithmetic {
    fn precedence(&self) -> u8 {
        match self {
            Arithmetic::Add => 9,
            Arithmetic::Subtract => 9,
            Arithmetic::Multiply => 10,
            Arithmetic::Divide => 10,
            Arithmetic::Modulo => 10,
        }
    }
}

pub enum Comparison {
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
}

impl HasPrecedence for Comparison {
    fn precedence(&self) -> u8 {
        match self {
            Comparison::Equal => 6,
            Comparison::NotEqual => 6,
            Comparison::Less => 7,
            Comparison::Greater => 7,
            Comparison::LessEqual => 7,
            Comparison::GreaterEqual => 7,
        }
    }
}

pub enum Logical {
    And,
    Or,
}

impl HasPrecedence for Logical {
    fn precedence(&self) -> u8 {
        match self {
            Logical::And => 2,
            Logical::Or => 1,
        }
    }
}

/**NOTE: BNOT is in PrefixUnary*/
pub enum BitwiseBinary {
    And,
    Or,
    Xor,
    LeftShift,
    RightShift,
    UnsignedRightShift,
}

impl HasPrecedence for BitwiseBinary {
    fn precedence(&self) -> u8 {
        match self {
            BitwiseBinary::And => 4,
            BitwiseBinary::Or => 3,
            BitwiseBinary::Xor => 5,
            BitwiseBinary::LeftShift => 8,
            BitwiseBinary::RightShift => 8,
            BitwiseBinary::UnsignedRightShift => 8,
        }
    }
}

pub enum PrefixUnary {
    Negative,
    Not,
    BNot,
    New,
    Destruct,
}

impl HasPrecedence for PrefixUnary {
    fn precedence(&self) -> u8 {
        match self {
            PrefixUnary::Negative => 11,
            PrefixUnary::Not => 11,
            PrefixUnary::BNot => 11,
            PrefixUnary::New => 11,
            PrefixUnary::Destruct => 11,
        }
    }
}

pub enum PostfixUnary  {
    Deref,
    Bang,
    Question,
}

impl HasPrecedence for PostfixUnary {
    fn precedence(&self) -> u8 {
        match self {
            PostfixUnary::Deref => 12,
            PostfixUnary::Bang => 12,
            PostfixUnary::Question => 12,
        }
    }
}

pub enum MemberAccess {
    Index,
    Dot,
    ExclamationDot,
    QuestionDot,
    QuestionQuestionDot,
}

impl HasPrecedence for MemberAccess {
    fn precedence(&self) -> u8 {
        match self {
            MemberAccess::Index => 12,
            MemberAccess::Dot => 12,
            MemberAccess::ExclamationDot => 12,
            MemberAccess::QuestionDot => 12,
            MemberAccess::QuestionQuestionDot => 12,
        }
    }
}

pub enum Special {
    UndefinedCoalescing,
    As,
}

impl HasPrecedence for Special {
    fn precedence(&self) -> u8 {
        match self {
            Special::UndefinedCoalescing => 0,
            Special::As => 12,
        }
    }
}

pub trait HasPrecedence {
    fn precedence(&self) -> u8;
}
