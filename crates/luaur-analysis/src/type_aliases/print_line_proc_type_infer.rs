#[allow(non_camel_case_types)]
pub type PrintLineProc = Option<extern "C" fn(line: &alloc::string::String)>;
