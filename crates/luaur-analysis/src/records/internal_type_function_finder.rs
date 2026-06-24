use crate::records::type_once_visitor::TypeOnceVisitor;

#[derive(Debug, Clone)]
pub struct InternalTypeFunctionFinder {
    pub(crate) base: TypeOnceVisitor,
    pub(crate) internal_functions:
        luaur_common::records::dense_hash_set::DenseHashSet<crate::type_aliases::type_id::TypeId>,
    pub(crate) internal_pack_functions: luaur_common::records::dense_hash_set::DenseHashSet<
        crate::type_aliases::type_pack_id::TypePackId,
    >,
    pub(crate) mentioned_functions:
        luaur_common::records::dense_hash_set::DenseHashSet<crate::type_aliases::type_id::TypeId>,
    pub(crate) mentioned_function_packs: luaur_common::records::dense_hash_set::DenseHashSet<
        crate::type_aliases::type_pack_id::TypePackId,
    >,
}
