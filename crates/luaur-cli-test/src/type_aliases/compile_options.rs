#[allow(non_camel_case_types)]
pub type CompileOptions =
    unsafe extern "C" fn() -> *mut luaur_compiler::records::compile_options::CompileOptions;
