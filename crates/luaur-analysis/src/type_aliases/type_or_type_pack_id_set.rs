use luaur_common::records::dense_hash_set::DenseHashSet;

#[allow(non_camel_case_types)]
pub type TypeOrTypePackIdSet = DenseHashSet<*const core::ffi::c_void>;
