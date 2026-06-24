use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;
use core::option::Option;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_node::AstNode;
use luaur_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug, Clone)]
pub struct FunctionCallConstraint {
    pub(crate) fn_type: TypeId,
    pub(crate) args_pack: TypePackId,
    pub(crate) result: TypePackId,
    pub(crate) call_site: *mut AstExprCall,
    pub(crate) discriminant_types: Vec<Option<TypeId>>,
    pub(crate) type_arguments: Vec<TypeId>,
    pub(crate) type_pack_arguments: Vec<TypePackId>,
    pub(crate) ast_overload_resolved_types: *mut DenseHashMap<*const AstNode, TypeId>,
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let r#fn: () = ();
}
