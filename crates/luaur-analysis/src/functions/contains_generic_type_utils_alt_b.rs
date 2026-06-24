use luaur_common::records::dense_hash_set::DenseHashSet;

use crate::records::contains_generics::ContainsGenerics;
use crate::records::iterative_type_visitor::IterativeTypeVisitorTrait;
use crate::type_aliases::type_pack_id::TypePackId;

pub fn contains_generic(
    tp: TypePackId,
    generics: *mut DenseHashSet<*const core::ffi::c_void>,
) -> bool {
    let mut cg = ContainsGenerics::contains_generics_contains_generics(generics);
    cg.run_type_pack_id(tp);
    cg.found
}
