use crate::node::NodeTag;

impl NodeTag {
    pub fn precedence(&self) -> u8 {
        match self {
            //Arithmetic
            NodeTag::Add => 11,
            NodeTag::Subtract => 11,
            NodeTag::Multiply => 12,
            NodeTag::Divide => 12,
            NodeTag::Modulo => 12,

            //Comparison
            NodeTag::Equal => 8,
            NodeTag::NotEqual => 8,
            NodeTag::Less => 9,
            NodeTag::Greater => 9,
            NodeTag::LessEqual => 9,
            NodeTag::GreaterEqual => 9,

            //Logical
            NodeTag::And => 4,
            NodeTag::Or => 3,

            //BitwiseBinary
            NodeTag::BAnd => 6,
            NodeTag::BOr => 5,
            NodeTag::XOr => 7,
            NodeTag::LeftShift => 10,
            NodeTag::RightShift => 10,
            NodeTag::UnsignedRightShift => 10,

            //Range
            NodeTag::InclusiveRange => 3,
            NodeTag::ExclusiveRange => 3,

            //PrefixUnary
            NodeTag::Negative => 13,
            NodeTag::Not => 13,
            NodeTag::BNot => 13,
            NodeTag::New => 13,
            NodeTag::Destruct => 13,
            NodeTag::ReferenceOf => 13,
            NodeTag::MutReferenceOf => 13,
            NodeTag::OwnershipOf => 13,

            //PostfixUnary
            NodeTag::Deref => 14,
            NodeTag::PanicIfErrOrNone => 14,
            NodeTag::BubbleIfErrOrNone => 14,
            NodeTag::FuncCall => 14,
            NodeTag::GenericInstantiation => 14,
            NodeTag::StructInstantiation => 14,
            NodeTag::Index => 14,

            //MemberAccess
            NodeTag::MemberAccess => 14,
            NodeTag::ImplAccess => 14,
            NodeTag::UndefinedChainingAccess => 14,
            NodeTag::Colon => 14,

            //Special
            NodeTag::UndefinedCoalescing => 2,
            NodeTag::As => 14,

            //Assignment
            NodeTag::Assignment  => 1,
            NodeTag::PlusAssignment  => 1,
            NodeTag::MinusAssignment  => 1,
            NodeTag::MultiplyAssignment  => 1,
            NodeTag::DivideAssignment  => 1,
            NodeTag::ModuloAssignment => 1,
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
            NodeTag::InclusiveRange |
            NodeTag::ExclusiveRange |
            NodeTag::Negative |
            NodeTag::Not |
            NodeTag::BNot |
            NodeTag::New |
            NodeTag::Destruct |
            NodeTag::ReferenceOf |
            NodeTag::MutReferenceOf |
            NodeTag::OwnershipOf |
            NodeTag::Deref |
            NodeTag::PanicIfErrOrNone |
            NodeTag::BubbleIfErrOrNone |
            NodeTag::FuncCall |
            NodeTag::GenericInstantiation |
            NodeTag::StructInstantiation |
            NodeTag::Index |
            NodeTag::MemberAccess |
            NodeTag::ImplAccess |
            NodeTag::UndefinedChainingAccess |
            NodeTag::Colon |
            NodeTag::UndefinedCoalescing |
            NodeTag::As |
            NodeTag::Assignment |
            NodeTag::PlusAssignment |
            NodeTag::MinusAssignment |
            NodeTag::MultiplyAssignment |
            NodeTag::DivideAssignment |
            NodeTag::ModuloAssignment
        )
    }

    pub fn is_postfix(&self) -> bool {
        matches!(self, 
            NodeTag::Deref |
            NodeTag::PanicIfErrOrNone |
            NodeTag::BubbleIfErrOrNone |
            NodeTag::FuncCall |
            NodeTag::GenericInstantiation |
            NodeTag::StructInstantiation |
            NodeTag::Index)
    }

    pub fn is_prefix(&self) -> bool {
        matches!(self,
            NodeTag::Negative |
            NodeTag::Not |
            NodeTag::BNot |
            NodeTag::New |
            NodeTag::Destruct |
            NodeTag::ReferenceOf |
            NodeTag::MutReferenceOf |
            NodeTag::OwnershipOf
        )
    }

    pub fn is_member_access(&self) -> bool {
        match self {
            NodeTag::MemberAccess |
            NodeTag::ImplAccess |
            NodeTag::UndefinedChainingAccess |
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
            NodeTag::InclusiveRange |
            NodeTag::ExclusiveRange |
            NodeTag::MemberAccess |
            NodeTag::ImplAccess |
            NodeTag::UndefinedChainingAccess |
            NodeTag::Colon |
            NodeTag::UndefinedCoalescing |
            NodeTag::As |
            NodeTag::Assignment |
            NodeTag::PlusAssignment |
            NodeTag::MinusAssignment |
            NodeTag::MultiplyAssignment |
            NodeTag::DivideAssignment |
            NodeTag::ModuloAssignment
        )
    }

    #[allow(dead_code)]
    fn is_assignment(&self) -> bool {
        matches!(self, 
            NodeTag::Assignment |
            NodeTag::PlusAssignment |
            NodeTag::MinusAssignment |
            NodeTag::MultiplyAssignment |
            NodeTag::DivideAssignment |
            NodeTag::ModuloAssignment
        )
    }
}
