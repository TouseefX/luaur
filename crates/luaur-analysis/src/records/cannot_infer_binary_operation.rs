use crate::enums::op_kind::OpKind;
use alloc::string::String;
use luaur_ast::records::ast_expr_binary::AstExprBinary_Op;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CannotInferBinaryOperation {
    pub(crate) op: AstExprBinary_Op,
    pub(crate) suggested_to_annotate: Option<String>,
    pub(crate) kind: OpKind,
}

impl core::hash::Hash for CannotInferBinaryOperation {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        (self.op as i32).hash(state);
        self.suggested_to_annotate.hash(state);
        self.kind.hash(state);
    }
}

#[allow(non_snake_case)]
impl CannotInferBinaryOperation {
    pub fn op(&self) -> AstExprBinary_Op {
        self.op
    }

    pub fn suggestedToAnnotate(&self) -> Option<&str> {
        self.suggested_to_annotate.as_deref()
    }

    pub fn kind(&self) -> OpKind {
        self.kind
    }
}

impl CannotInferBinaryOperation {
    pub const fn new(
        op: AstExprBinary_Op,
        suggested_to_annotate: Option<String>,
        kind: OpKind,
    ) -> Self {
        Self {
            op,
            suggested_to_annotate,
            kind,
        }
    }
}
