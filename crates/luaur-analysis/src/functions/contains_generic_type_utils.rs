use crate::records::contains_generics::ContainsGenerics;
use crate::records::iterative_type_visitor::IterativeTypeVisitorTrait;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_set::DenseHashSet;

pub fn contains_generic_type_id_not_null_dense_hash_set_void(
    ty: TypeId,
    generics: *mut DenseHashSet<*const core::ffi::c_void>,
) -> bool {
    let mut cg = ContainsGenerics::contains_generics_contains_generics(generics);
    cg.run_type_id(ty);
    cg.found
}
