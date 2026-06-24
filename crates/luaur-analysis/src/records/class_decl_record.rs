use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_stat_class::AstStatClass;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ClassDeclRecord {
    pub data_decl: *mut AstStatClass,
    pub ty: TypeId,
}

impl luaur_common::records::dense_hash_table::DenseDefault for ClassDeclRecord {
    fn dense_default() -> Self {
        Self {
            data_decl: core::ptr::null_mut(),
            ty: core::ptr::null(),
        }
    }
}
