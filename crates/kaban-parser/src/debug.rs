use std::fmt::{Debug, Result};
use kaban_core::{ToBool, UIndex};
use kaban_lexer::{TokenPrinter};

use crate::{ast::{AST, StructInstantiation}, node::{NodeIndex, NodeTag, ToOption, TokenIndex, U_NONE, UIndexVec}};

pub struct NodePrinter<'a> {
    ast: &'a AST<'a>,
    index: NodeIndex,
    skip_expression_statement_indent: bool,
}

impl<'a> Debug for NodePrinter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        let tag = self.ast.get_tag(self.index);
        let (left, right) = self.ast.get_left_right(self.index);
        let index = self.index;

        match tag {
            NodeTag::Self_ => write!(f, "{:?}", tag),
            NodeTag::AnonymousEnumlit => write!(f, "{:?}", tag),
            t if t.is_token_leaf() => self.write_token(f, left),
            NodeTag::BoolLit => { write!(f, "{}", left.bool()) },
            NodeTag::ExpressionStatement => {
                if self.skip_expression_statement_indent {
                    write!(f, "{:#?}", &self.child(left))
                } else {
                    f.debug_tuple("ExpressionStatement").field(&self.child(left)).finish()
                }
            },
            t if t.is_binary_expression() => {
                f.debug_struct(format!("{:?}", t).as_str())
                    .field("left", &self.child(left))
                    .field("right", &self.child(right))
                    .finish()
            },
            NodeTag::ArrayLit |
            NodeTag::Block |
            NodeTag::Union |
            NodeTag::MultipleMatchTargets |
            NodeTag::TupleDestructure |
            NodeTag::StructDestructure |
            NodeTag::ArrayDestructure |
            NodeTag::TupleType |
            NodeTag::AnonymousStructType |
            NodeTag::TupleLit |
            NodeTag::AnonymousEnumType => {
                let general_list = self.ast.view_general_list(index);
                write!(f, "{:?} ", tag)?;
                f.debug_list()
                    .entries(self.children(general_list.indices).iter())
                    .finish()
            },
            NodeTag::Index => {
                let index = self.ast.view_index(index);
                f.debug_struct("Index")
                    .field("callee", &self.child(index.callee.0))
                    .field("index", &self.child(index.index_by.0))
                    .field("is_safe", &index.is_safe_index)
                    .finish()
            },
            NodeTag::FuncCall => {
                let func_call = self.ast.view_func_call(index);
                f.debug_struct("FuncCall")
                    .field("callee", &self.child(left))
                    .field("args", &self.children(func_call.args.uindex_slice()))
                    .finish()
            }
            NodeTag::GenericInstantiation => {
                let generic_instantiation = self.ast.view_generic_instantiation(index);
                f.debug_struct("GenericInstantiation")
                    .field("callee", &self.child(left))
                    .field("args", &self.children(generic_instantiation.args.uindex_slice()))
                    .finish()
            }
            NodeTag::StructInstantiation => {
                let struct_instantiation: StructInstantiation<'_> = self.ast.view_struct_instantiation(index);
                if let Some(struct_name) = struct_instantiation.struct_name {
                    f.debug_struct("StructInstantiation")
                        .field("name", &self.child(struct_name.0))
                        .field("fields", &self.children(struct_instantiation.field_instantiation.uindex_slice()))
                        // .field("fields", &struct_instantiation.field_instantiation)
                        .finish()
                } else {
                    f.debug_struct("StructInstantiation")
                        .field("name", &"NONE")
                        .field("fields", &self.children(struct_instantiation.field_instantiation.uindex_slice()))
                        .finish()
                }
            },
            NodeTag::StructFieldInstantiation => 
                f.debug_struct("Struct Field")
                .field("name", &self.get_token(left))
                .field("assignment", &self.child(right))
                .finish(),
            t if t.is_postfix() || t.is_prefix() => {
                f.debug_tuple(format!("{:?}", t).as_str())
                    .field(&self.child(left))
                    .finish()
            }
            NodeTag::FixedArrayType => {
                f.debug_struct("FixedArrayType")
                    .field("type", &self.child(left))
                    .field("size", &self.child(right))
                    .finish()

            },
            NodeTag::NamedType => {
                f.debug_tuple("NamedType")
                    .field(&self.get_token(left))
                    .finish()
            },
            NodeTag::TypeWithGenerics => {
                let type_with_generics = self.ast.view_type_with_generics(index);
                f.debug_struct("TypeWithGenerics")
                    .field("type", &self.child(type_with_generics.type_.0))
                    .field("generic_args", &self.children(type_with_generics.generic_args.uindex_slice()))
                    .finish()
            }
            NodeTag::FuncType => {
                let func_type = self.ast.view_func_type(index);
                if let Some(return_type) = func_type.return_type {
                    f.debug_struct("FuncType")
                        .field("params", &self.children(func_type.params.uindex_slice()))
                        .field("return type", &self.child(return_type.0))
                        .finish()
                } else {
                    f.debug_struct("FuncType")
                        .field("params", &self.children(func_type.params.uindex_slice()))
                        .field("return type", &"NONE")
                        .finish()
                }
            },
            t if t.is_simple_modifier_type() => {
                f.debug_tuple(format!("{:?}", t).as_str())
                    .field(&self.child(left))
                    .finish()
            },
            NodeTag::MethodCall => {
                let method = self.ast.view_method_call(index);
                f.debug_struct("MethodCall")
                    .field("callee", &self.child(method.callee.0))
                    .field("method name", &self.child(method.method_name.0))
                    .field("is mutable", &method.is_self_mut)
                    .field("args", &self.children(method.args.uindex_slice()))
                    .finish()
            }
            NodeTag::MethodWithGenericInstantiation => {
                let method = self.ast.view_method_call_with_generic_instantiation(index);
                f.debug_struct("MethodWithGenericInstantiation")
                    .field("callee", &self.child(method.callee.0))
                    .field("method name", &self.child(method.method_name.0))
                    .field("is mutable", &method.is_self_mut)
                    .field("generic_args", &self.children(method.generic_args.uindex_slice()))
                    .field("args", &self.children(method.args.uindex_slice()))
                    .finish()
            }
            t if t.is_type() => write!(f, "Type({:?})", t),
            NodeTag::If => {
                let if_ = self.ast.view_if_expression(index);
                if let Some(else_) = if_.else_ {
                    f.debug_struct("IfExpression")
                        .field("condition", &self.child(if_.condition.0))
                        .field("then", &self.child(if_.then.0))
                        .field("else", &self.child(else_.0))
                        .finish()
                } else {
                    f.debug_struct("IfExpression")
                        .field("condition", &self.child(if_.condition.0))
                        .field("then", &self.child(if_.then.0))
                        .finish()
                }
            },
            NodeTag::Match => {
                let match_ = self.ast.view_match_expression(index);
                f.debug_struct("Match")
                    .field("target", &self.child(match_.target.0))
                    .field("arms", &self.children(match_.arms.uindex_slice()))
                    .finish()
            },
            NodeTag::MatchArms => {
                f.debug_struct("Arm")
                    .field("left", &self.child(left))
                    .field("right", &self.child(right))
                    .finish()
            },
            NodeTag::DoWhile | NodeTag::While => {
                f.debug_struct(format!("{:?}", tag).as_str())
                    .field("block", &self.child(right))
                    .field("condition", &self.child(left))
                    .finish()
            },
            NodeTag::For => {
                let for_loop = self.ast.view_for_expression(index);
                f.debug_struct("For")
                    .field("binding", &self.child(for_loop.binding.0))
                    .field("iterator", &self.child(for_loop.iterator.0))
                    .field("block", &self.child(for_loop.block.0))
                    .finish()
            },
            NodeTag::AnonymousFuncDecl => {
                let func = self.ast.view_anonymous_func_decl(index);
                if let Some(return_type) = func.return_type {
                    f.debug_struct("AnonymousFuncDecl")
                        .field("params", &self.children(func.params.uindex_slice()))
                        .field("return type", &self.child(return_type.0))
                        .field("block", &self.child(func.block.0))
                        .finish()
                } else {
                    f.debug_struct("AnonymousFuncDecl")
                        .field("params", &self.children(func.params.uindex_slice()))
                        .field("return type", &"NONE")
                        .field("block", &self.child(func.block.0))
                        .finish()
                }
            },
            NodeTag::Params =>
                if right != U_NONE {
                    f.debug_struct("Param")
                        .field("binding", &self.child(left))
                        .field("type", &self.child(right))
                        .finish()
                } else {
                    f.debug_struct("Param")
                        .field("binding", &self.child(left))
                        .field("type", &"NONE")
                        .finish()
                }
            NodeTag::IdentifierBinding =>
                f.debug_struct("IdentifierBinding")
                .field("name", &self.get_token(left))
                .field("mutable", &right.bool())
                .finish(),
            NodeTag::StructDestructureBinding =>
                f.debug_struct("StructDestructureBinding")
                .field("field_name", &self.get_token(left))
                .field("binding", &self.child(right))
                .finish(),
            NodeTag::Let => {
                let let_ = self.ast.view_let_statement(index);
                if let Some(type_) = let_.type_ {
                    f.debug_struct("Let")
                        .field("name", &self.child(let_.name.0))
                        .field("type", &self.child(type_.0))
                        .field("assignment", &self.child(let_.assignment.0))
                        .finish()
                } else {
                    f.debug_struct("Let")
                        .field("name", &self.child(let_.name.0))
                        .field("assignment", &self.child(let_.assignment.0))
                        .finish()
                }
            },
            NodeTag::Const => {
                let const_ = self.ast.view_const_statement(index);
                f.debug_struct("Const")
                    .field("is_pub", &const_.is_pub)
                    .field("name", &self.get_token(const_.identifier.0))
                    .field("type", &self.child(const_.type_.0))
                    .field("assignment", &self.child(const_.assignment.0))
                    .finish()
            },
            NodeTag::Continue | NodeTag::Break => write!(f, "{:?}", tag),
            NodeTag::Return | 
                NodeTag::Pass |
                NodeTag::New |
                NodeTag::Destruct |
                NodeTag::CompTimeExpression => {
                if let Some(left) = left.to_option() {
                    f.debug_tuple(format!("{:?}", tag).as_str()).field(&self.child(left)).finish()
                } else {
                    f.debug_tuple(format!("{:?}", tag).as_str()).finish()
                }
            },
            NodeTag::FuncDeclWithNoGenerics => {
                let func_decl = self.ast.view_func_decl_with_no_generics(index);
                if let Some(return_type) = func_decl.return_type {
                    f.debug_struct("FuncDeclWithNoGeneric")
                        .field("is_pub", &func_decl.is_pub)
                        .field("name", &self.get_token(func_decl.name.0))
                        .field("params", &self.children(func_decl.params.uindex_slice()))
                        .field("return_type", &self.child(return_type.0))
                        .field("body", &self.child(func_decl.block.0))
                        .finish()
                } else {
                    f.debug_struct("FuncDeclWithNoGeneric")
                        .field("is_pub", &func_decl.is_pub)
                        .field("name", &self.get_token(func_decl.name.0))
                        .field("params", &self.children(func_decl.params.uindex_slice()))
                        .field("return_type", &"NONE")
                        .field("body", &self.child(func_decl.block.0))
                        .finish()
                }
            }
            NodeTag::FuncDeclWithGenerics => {
                let func_decl = self.ast.view_func_decl_with_generics(index);
                if let Some(return_type) = func_decl.return_type {
                    f.debug_struct("FuncDeclWithGeneric")
                        .field("is_pub", &func_decl.is_pub)
                        .field("name", &self.get_token(func_decl.name.0))
                        .field("params", &self.children(func_decl.params.uindex_slice()))
                        .field("generic_params", &self.children(func_decl.generic_params.uindex_slice()))
                        .field("return_type", &self.child(return_type.0))
                        .field("body", &self.child(func_decl.block.0))
                        .finish()
                } else {
                    f.debug_struct("FuncDeclWithNoGeneric")
                        .field("is_pub", &func_decl.is_pub)
                        .field("name", &self.get_token(func_decl.name.0))
                        .field("params", &self.children(func_decl.params.uindex_slice()))
                        .field("generic_params", &self.children(func_decl.generic_params.uindex_slice()))
                        .field("return_type", &"NONE")
                        .field("body", &self.child(func_decl.block.0))
                        .finish()
                }
            }
            NodeTag::FuncNoBodyWithNoGenerics => {
                let func_decl = self.ast.view_func_no_body_with_no_generics(index);
                if let Some(return_type) = func_decl.return_type {
                    f.debug_struct("FuncNoBodyWithNoGeneric")
                        .field("is_pub", &func_decl.is_pub)
                        .field("name", &self.get_token(func_decl.name.0))
                        .field("params", &self.children(func_decl.params.uindex_slice()))
                        .field("return_type", &self.child(return_type.0))
                        .finish()
                } else {
                    f.debug_struct("FuncNoBodyWithNoGeneric")
                        .field("is_pub", &func_decl.is_pub)
                        .field("name", &self.get_token(func_decl.name.0))
                        .field("params", &self.children(func_decl.params.uindex_slice()))
                        .field("return_type", &"NONE")
                        .finish()
                }
            }
            NodeTag::FuncNoBodyWithGenerics => {
                let func_decl = self.ast.view_func_no_body_with_generics(index);
                if let Some(return_type) = func_decl.return_type {
                    f.debug_struct("FuncNoBodyWithGeneric")
                        .field("is_pub", &func_decl.is_pub)
                        .field("name", &self.get_token(func_decl.name.0))
                        .field("params", &self.children(func_decl.params.uindex_slice()))
                        .field("generic_params", &self.children(func_decl.generic_params.uindex_slice()))
                        .field("return_type", &self.child(return_type.0))
                        .finish()
                } else {
                    f.debug_struct("FuncNoBodyWithNoGeneric")
                        .field("is_pub", &func_decl.is_pub)
                        .field("name", &self.get_token(func_decl.name.0))
                        .field("params", &self.children(func_decl.params.uindex_slice()))
                        .field("generic_params", &self.children(func_decl.generic_params.uindex_slice()))
                        .field("return_type", &"NONE")
                        .finish()
                }
            }
            NodeTag::StructDeclWithNoGeneric => {
                let struct_decl = self.ast.view_struct_decl_with_no_generics(index);
                f.debug_struct("StructDeclWithNoGeneric")
                    .field("is_pub", &struct_decl.is_pub)
                    .field("name", &self.get_token(struct_decl.struct_name.0))
                    .field("field_decls", &self.children(struct_decl.field_decls.uindex_slice()))
                    .finish()
            }
            NodeTag::StructDeclWithGeneric => {
                let struct_decl = self.ast.view_struct_decl_with_generics(index);
                f.debug_struct("StructDeclWithGeneric")
                    .field("is_pub", &struct_decl.is_pub)
                    .field("name", &self.get_token(struct_decl.struct_name.0))
                    .field("generic_params", &self.children(struct_decl.generic_params.uindex_slice()))
                    .field("field_decls", &self.children(struct_decl.field_decls.uindex_slice()))
                    .finish()
            }
            NodeTag::StructFieldDecleration => {
                let field_decl = self.ast.view_struct_field_decl(index);
                f.debug_struct("StructFieldDecleration")
                    .field("is_pub", &field_decl.is_pub)
                    .field("field_name", &self.get_token(field_decl.field_name.0))
                    .field("type", &self.child(field_decl.type_.0))
                    .finish()
            }
            NodeTag::AnonymousStructFieldDecl =>
                f.debug_struct("AnonymousStructFieldDecl")
                .field("field_name", &self.get_token(left))
                .field("type", &self.child(right))
                .finish(),
            NodeTag::EnumDeclWithNoGeneric => {
                let enum_decl = self.ast.view_enum_decl_with_no_generics(index);
                f.debug_struct("EnumDeclWithNoGeneric")
                    .field("is_pub", &enum_decl.is_pub)
                    .field("name", &self.get_token(enum_decl.name.0))
                    .field("variant_decls", &self.children(enum_decl.variant_decls.uindex_slice()))
                    .finish()
            }
            NodeTag::EnumDeclWithGeneric => {
                let enum_decl = self.ast.view_enum_decl_with_generics(index);
                f.debug_struct("EnumDeclWithGeneric")
                    .field("is_pub", &enum_decl.is_pub)
                    .field("name", &self.get_token(enum_decl.name.0))
                    .field("generic_params", &self.children(enum_decl.generic_params.uindex_slice()))
                    .field("variant_decls", &self.children(enum_decl.variant_decls.uindex_slice()))
                    .finish()
            }
            NodeTag::ImplDeclWithNoGeneric => {
                let impl_decl = self.ast.view_impl_decl_with_no_generics(index);
                f.debug_struct("ImplDeclWithNoGeneric")
                    .field("is_pub", &impl_decl.is_pub)
                    .field("impl_name", &self.get_token(impl_decl.impl_name.0))
                    .field("self_type", &self.child(impl_decl.self_type.0))
                    .field("statements", &self.children(impl_decl.statements.uindex_slice()))
                    .finish()
            }
            NodeTag::ImplDeclWithGeneric => {
                let impl_decl = self.ast.view_impl_decl_with_generics(index);
                f.debug_struct("ImplDeclWithGeneric")
                    .field("is_pub", &impl_decl.is_pub)
                    .field("impl_name", &self.get_token(impl_decl.impl_name.0))
                    .field("self_type", &self.child(impl_decl.self_type.0))
                    .field("generic_params", &self.children(impl_decl.generic_params.uindex_slice()))
                    .field("statements", &self.children(impl_decl.statements.uindex_slice()))
                    .finish()
            }
            NodeTag::ImplForDeclWithNoGeneric => {
                let impl_decl = self.ast.view_impl_for_decl_with_no_generics(index);
                f.debug_struct("ImplForDeclWithNoGeneric")
                    .field("is_pub", &impl_decl.is_pub)
                    .field("impl_name", &self.get_token(impl_decl.impl_name.0))
                    .field("self_type", &self.child(impl_decl.self_type.0))
                    .field("interface", &self.child(impl_decl.interface.0))
                    .field("statements", &self.children(impl_decl.statements.uindex_slice()))
                    .finish()
            }
            NodeTag::ImplForDeclWithGeneric => {
                let impl_decl = self.ast.view_impl_for_decl_with_generics(index);
                f.debug_struct("ImplForDeclWithGeneric")
                    .field("is_pub", &impl_decl.is_pub)
                    .field("impl_name", &self.get_token(impl_decl.impl_name.0))
                    .field("self_type", &self.child(impl_decl.self_type.0))
                    .field("interface", &self.child(impl_decl.interface.0))
                    .field("generic_params", &self.children(impl_decl.generic_params.uindex_slice()))
                    .field("statements", &self.children(impl_decl.statements.uindex_slice()))
                    .finish()
            }
            NodeTag::InterfaceDeclWithNoGenerics => {
                let interface_decl = self.ast.view_interface_decl_with_no_generics(index);
                if let Some(shape) = interface_decl.shape {
                    f.debug_struct("InterfaceDeclWithNoGeneric")
                        .field("is_pub", &interface_decl.is_pub)
                        .field("interface_name", &self.get_token(interface_decl.name.0))
                        .field("shape", &self.child(shape.0))
                        .field("statements", &self.children(interface_decl.statements.uindex_slice()))
                        .finish()
                } else {
                    f.debug_struct("InterfaceDeclWithNoGeneric")
                        .field("is_pub", &interface_decl.is_pub)
                        .field("interface_name", &self.get_token(interface_decl.name.0))
                        .field("statements", &self.children(interface_decl.statements.uindex_slice()))
                        .finish()
                }
            }
            NodeTag::InterfaceDeclWithGenerics => {
                let interface_decl = self.ast.view_interface_decl_with_generics(index);
                if let Some(shape) = interface_decl.shape {
                    f.debug_struct("InterfaceDeclWithGeneric")
                        .field("is_pub", &interface_decl.is_pub)
                        .field("interface_name", &self.get_token(interface_decl.name.0))
                        .field("shape", &self.child(shape.0))
                        .field("statements", &self.children(interface_decl.statements.uindex_slice()))
                        .field("generic_params", &self.children(interface_decl.generic_params.uindex_slice()))
                        .finish()
                } else {
                    f.debug_struct("InterfaceDeclWithGeneric")
                        .field("is_pub", &interface_decl.is_pub)
                        .field("interface_name", &self.get_token(interface_decl.name.0))
                        .field("statements", &self.children(interface_decl.statements.uindex_slice()))
                        .field("generic_params", &self.children(interface_decl.generic_params.uindex_slice()))
                        .finish()
                }
            }
            NodeTag::EnumVariantDecl =>
                if right != U_NONE {
                    f.debug_struct("EnumVariantDecl")
                        .field("variant_name", &self.get_token(left))
                        .field("type", &self.child(right))
                        .finish()
                } else {
                    f.debug_struct("EnumVariantDecl")
                        .field("variant_name", &self.get_token(left))
                        .field("type", &"NONE")
                        .finish()
                },
            NodeTag::GenericParam =>
                if right != U_NONE {
                    f.debug_struct("GenericParam")
                        .field("name", &self.get_token(left))
                        .field("constraint", &self.child(right))
                        .finish()
                } else {
                    f.debug_struct("GenericParam")
                        .field("name", &self.get_token(left))
                        .finish()
                },
            NodeTag::AndGenericConstaint | NodeTag::OrGenericConstaint => {
                f.debug_struct(format!("{:?}", tag).as_str())
                    .field("left", &self.child(left))
                    .field("right", &self.child(right))
                    .finish()
            },
            NodeTag::InterfaceConstraint => 
                f.debug_tuple("InterfaceConstraint")
                .field(&self.get_token(left))
                .finish(),
            // NodeTag::SelfParam => if left != U_NONE {
            //     if right.bool() {
            //         f.debug_tuple("MutSelfParam").field(&self.get_token(left)).finish()
            //     } else {
            //         f.debug_tuple("SelfParam").field(&self.get_token(left)).finish()
            //     }
            // } else {
            //     if right.bool() {
            //         f.debug_tuple("MutSelfParam").finish()
            //     } else {
            //         f.debug_tuple("SelfParam").finish()
            //     }
            // }
            NodeTag::SelfParam => if left != U_NONE {
                f.debug_tuple("SelfParam").field(&self.get_token(left)).finish()
            } else {
                f.debug_tuple("SelfParam").finish()
            }
            _ => todo!("NOT IMPLEMENTED YET: {:?}", tag)
        }
    }
}

impl<'a> NodePrinter<'a> {
    fn child(&self, index: UIndex) -> Self {
        Self {
            ast: self.ast,
            index: NodeIndex(index),
            skip_expression_statement_indent: self.skip_expression_statement_indent,
        }
    }

    fn children(&self, indices: &[u32]) -> Vec<Self> {
        let mut children = Vec::new();
        for index in indices.iter().copied()  {
            children.push(Self {
                ast: self.ast,
                index: NodeIndex(index),
                skip_expression_statement_indent: self.skip_expression_statement_indent,
            });
        };
        children
    }

    fn get_token(&self, index: UIndex) -> TokenPrinter<'a> {
        let token = self.ast.get_token(TokenIndex(index));
        token.to_debugger(self.ast.get_source())
    }

    fn write_token(&self, f: &mut std::fmt::Formatter<'_>, index: UIndex) -> std::fmt::Result {
        write!(f, "{:?}", self.get_token(index))
    }
}

impl<'a> AST<'a> {
    pub fn to_debugger(&'a self) -> NodePrinter<'a> {
        NodePrinter {
            ast: self,
            index: self.root,
            skip_expression_statement_indent: false,
        }
    }

    pub fn to_debugger_options(&'a self, skip_expression_statement: bool) -> NodePrinter<'a> {
        NodePrinter {
            ast: self,
            index: self.root,
            skip_expression_statement_indent: skip_expression_statement,
        }
    }

    pub fn create_debugger(&'a self, node: NodeIndex) -> NodePrinter<'a> {
        NodePrinter {
            ast: self,
            index: node,
            skip_expression_statement_indent: false,
        }
    }
}
