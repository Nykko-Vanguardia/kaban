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

impl Operator {
    pub fn precedence(&self) -> u8 {
        match self {
            Operator::Arithmetic(op) => match op {
                Arithmetic::Add => 9,
                Arithmetic::Subtract => 9,
                Arithmetic::Multiply => 10,
                Arithmetic::Divide => 10,
                Arithmetic::Modulo => 10,
            }
            Operator::Comparison(op) => match op {
                Comparison::Equal => 6,
                Comparison::NotEqual => 6,
                Comparison::Less => 7,
                Comparison::Greater => 7,
                Comparison::LessEqual => 7,
                Comparison::GreaterEqual => 7,
            }
            Operator::Logical(op) => match op {
                Logical::And => 2,
                Logical::Or => 1,
            }
            Operator::BitwiseBinary(op) => match op {
                BitwiseBinary::And => 4,
                BitwiseBinary::Or => 3,
                BitwiseBinary::Xor => 5,
                BitwiseBinary::LeftShift => 8,
                BitwiseBinary::RightShift => 8,
                BitwiseBinary::UnsignedRightShift => 8,
            }
            Operator::PrefixUnary(op) => match op {
                PrefixUnary::Negative => 11,
                PrefixUnary::Not => 11,
                PrefixUnary::BNot => 11,
            },
            Operator::PostfixUnary(op) => match op {
                PostfixUnary::Deref => 12,
                PostfixUnary::Bang => 12,
                PostfixUnary::Question => 12,

            },
            Operator::MemberAccess(op) => match op {
                MemberAccess::Index => 12,
                MemberAccess::Dot => 12,
                MemberAccess::ExclamationDot => 12,
                MemberAccess::QuestionDot => 12,
                MemberAccess::QuestionQuestionDot => 12,
            },
            Operator::Special(op) => match op {
                Special::UndefinedCoalescing => 0,
                Special::As => 12,
            },
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

pub enum Comparison {
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
}

pub enum Logical {
    And,
    Or,
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

pub enum PrefixUnary {
    Negative,
    Not,
    BNot,
}

pub enum PostfixUnary  {
    Deref,
    Bang,
    Question,
}

pub enum MemberAccess {
    Index,
    Dot,
    ExclamationDot,
    QuestionDot,
    QuestionQuestionDot,
}

pub enum Special {
    UndefinedCoalescing,
    As,
}
