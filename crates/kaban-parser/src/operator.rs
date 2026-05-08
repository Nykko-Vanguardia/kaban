use crate::node::NodeTag;

impl NodeTag {
    pub fn precedence(&self) -> u8 {
        match self {
            //Arithmetic
            NodeTag::Add => 10,
            NodeTag::Subtract => 10,
            NodeTag::Multiply => 11,
            NodeTag::Divide => 11,
            NodeTag::Modulo => 11,

            //Comparison
            NodeTag::Equal => 7,
            NodeTag::NotEqual => 7,
            NodeTag::Less => 8,
            NodeTag::Greater => 8,
            NodeTag::LessEqual => 8,
            NodeTag::GreaterEqual => 8,

            //Logical
            NodeTag::And => 3,
            NodeTag::Or => 2,

            //BitwiseBinary
            NodeTag::BAnd => 5,
            NodeTag::BOr => 4,
            NodeTag::XOr => 6,
            NodeTag::LeftShift => 9,
            NodeTag::RightShift => 9,
            NodeTag::UnsignedRightShift => 9,

            //PrefixUnary
            NodeTag::Negative => 12,
            NodeTag::Not => 12,
            NodeTag::BNot => 12,
            NodeTag::New => 12,
            NodeTag::Destruct => 12,

            //PostfixUnary
            NodeTag::Deref => 13,
            NodeTag::PanicIfErr => 13,
            NodeTag::BubbleIfErr => 13,
            NodeTag::FuncCall => 13,
            NodeTag::Index => 13,

            //MemberAccess
            NodeTag::Dot => 13,
            NodeTag::ExclamationDot => 13,
            NodeTag::QuestionDot => 13,
            NodeTag::QuestionQuestionDot => 13,
            NodeTag::Colon => 13,

            //Special
            NodeTag::UndefinedCoalescing => 1,
            NodeTag::As => 13,
            _ => {
                debug_assert!(self.is_operator(), 
                    "Tried to get a precedence of a non error node tag: {:?}", self);
                0
            }        
        }
    }

    pub fn is_operator(&self) -> bool {
        matches!(self, 
            //Arithmetic
            NodeTag::Add |
            NodeTag::Subtract |
            NodeTag::Multiply |
            NodeTag::Divide |
            NodeTag::Modulo |
            NodeTag::Equal |
            NodeTag::NotEqual |
            NodeTag::Less |
            NodeTag::Greater |
            NodeTag::LessEqual |
            NodeTag::GreaterEqual |
            NodeTag::And |
            NodeTag::Or |
            NodeTag::BAnd |
            NodeTag::BOr |
            NodeTag::XOr |
            NodeTag::LeftShift |
            NodeTag::RightShift |
            NodeTag::UnsignedRightShift |
            NodeTag::Negative |
            NodeTag::Not |
            NodeTag::BNot |
            NodeTag::New |
            NodeTag::Destruct |
            NodeTag::Deref |
            NodeTag::PanicIfErr |
            NodeTag::BubbleIfErr |
            NodeTag::FuncCall |
            NodeTag::Index |
            NodeTag::Dot |
            NodeTag::ExclamationDot |
            NodeTag::QuestionDot |
            NodeTag::QuestionQuestionDot |
            NodeTag::Colon |
            NodeTag::UndefinedCoalescing |
            NodeTag::As
        )
    }

    pub fn is_postfix(&self) -> bool {
        matches!(self, 
            NodeTag::Deref |
            NodeTag::PanicIfErr |
            NodeTag::BubbleIfErr |
            NodeTag::FuncCall |
            NodeTag::Index)
    }

    pub fn is_prefix(&self) -> bool {
        matches!(self,
            NodeTag::Negative |
            NodeTag::Not |
            NodeTag::BNot |
            NodeTag::New |
            NodeTag::Destruct)
    }

    pub fn is_member_access(&self) -> bool {
        match self {
            NodeTag::Dot => true,
            NodeTag::ExclamationDot => true,
            NodeTag::QuestionDot => true,
            NodeTag::QuestionQuestionDot => true,
            NodeTag::Colon => true,
            _ => false,
        }
    }

    /**
     * Binary here is defined as this has a left and right side pointing to expressions
     */
    pub fn is_binary_expression(&self) -> bool {
        matches!(self, 
            NodeTag::Add |
            NodeTag::Subtract |
            NodeTag::Multiply |
            NodeTag::Divide |
            NodeTag::Modulo |
            NodeTag::Equal |
            NodeTag::NotEqual |
            NodeTag::Less |
            NodeTag::Greater |
            NodeTag::LessEqual |
            NodeTag::GreaterEqual |
            NodeTag::And |
            NodeTag::Or |
            NodeTag::BAnd |
            NodeTag::BOr |
            NodeTag::XOr |
            NodeTag::LeftShift |
            NodeTag::RightShift |
            NodeTag::UnsignedRightShift |
            NodeTag::Dot |
            NodeTag::ExclamationDot |
            NodeTag::QuestionDot |
            NodeTag::QuestionQuestionDot |
            NodeTag::Colon |
            NodeTag::UndefinedCoalescing |
            NodeTag::As
        )
    }
}
