use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr_binary::AstExprBinary_Op;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CannotCompareUnrelatedTypes {
    pub(crate) left: TypeId,
    pub(crate) right: TypeId,
    pub(crate) op: AstExprBinary_Op,
}

impl core::hash::Hash for CannotCompareUnrelatedTypes {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.left.hash(state);
        self.right.hash(state);
        (self.op as i32).hash(state);
    }
}
