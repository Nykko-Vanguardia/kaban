#[derive(Debug)]
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
    Index,
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
            Operator::FuncCall => 13,
            Operator::Index => 13,
        }
    }

}

impl Operator {
    pub fn is_postfix(&self) -> bool {
        matches!(self, Operator::PostfixUnary(_) | Operator::FuncCall | Operator::Index)
    }
}

#[derive(Debug)]
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
            Arithmetic::Add => 10,
            Arithmetic::Subtract => 10,
            Arithmetic::Multiply => 11,
            Arithmetic::Divide => 11,
            Arithmetic::Modulo => 11,
        }
    }
}

#[derive(Debug)]
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
            Comparison::Equal => 7,
            Comparison::NotEqual => 7,
            Comparison::Less => 8,
            Comparison::Greater => 8,
            Comparison::LessEqual => 8,
            Comparison::GreaterEqual => 8,
        }
    }
}

#[derive(Debug)]
pub enum Logical {
    And,
    Or,
}

impl HasPrecedence for Logical {
    fn precedence(&self) -> u8 {
        match self {
            Logical::And => 3,
            Logical::Or => 2,
        }
    }
}

/**NOTE: BNOT is in PrefixUnary*/
#[derive(Debug)]
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
            BitwiseBinary::And => 5,
            BitwiseBinary::Or => 4,
            BitwiseBinary::Xor => 6,
            BitwiseBinary::LeftShift => 9,
            BitwiseBinary::RightShift => 9,
            BitwiseBinary::UnsignedRightShift => 9,
        }
    }
}

#[derive(Debug)]
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
            PrefixUnary::Negative => 12,
            PrefixUnary::Not => 12,
            PrefixUnary::BNot => 12,
            PrefixUnary::New => 12,
            PrefixUnary::Destruct => 12,
        }
    }
}

#[derive(Debug)]
pub enum PostfixUnary  {
    Deref,
    Bang,
    Question,
}

impl HasPrecedence for PostfixUnary {
    fn precedence(&self) -> u8 {
        match self {
            PostfixUnary::Deref => 13,
            PostfixUnary::Bang => 13,
            PostfixUnary::Question => 13,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MemberAccess {
    Dot,
    ExclamationDot,
    QuestionDot,
    QuestionQuestionDot,
    Colon,
}

impl HasPrecedence for MemberAccess {
    fn precedence(&self) -> u8 {
        match self {
            MemberAccess::Dot => 13,
            MemberAccess::ExclamationDot => 13,
            MemberAccess::QuestionDot => 13,
            MemberAccess::QuestionQuestionDot => 13,
            MemberAccess::Colon => 13,
        }
    }
}

#[derive(Debug)]
pub enum Special {
    UndefinedCoalescing,
    As,
}

impl HasPrecedence for Special {
    fn precedence(&self) -> u8 {
        match self {
            Special::UndefinedCoalescing => 1,
            Special::As => 13,
        }
    }
}

pub trait HasPrecedence {
    fn precedence(&self) -> u8;
}
