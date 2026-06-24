#[allow(non_camel_case_types)]
pub type SyntheticNames = luaur_common::records::dense_hash_map::DenseHashMap<
    *const core::ffi::c_void,
    *mut core::ffi::c_char,
>;
