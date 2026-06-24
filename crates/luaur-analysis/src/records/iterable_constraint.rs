use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;
use luaur_ast::records::ast_node::AstNode;
use luaur_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug, Clone)]
pub struct IterableConstraint {
    pub(crate) iterator: TypePackId,
    pub(crate) variables: Vec<TypeId>,
    pub(crate) next_ast_fragment: *const AstNode,
    pub(crate) ast_for_in_next_types: *mut DenseHashMap<*const AstNode, TypeId>,
}
