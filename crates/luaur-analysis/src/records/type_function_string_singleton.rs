#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TypeFunctionStringSingleton {
    pub value: alloc::string::String,
}

unsafe impl Send for TypeFunctionStringSingleton {}
unsafe impl Sync for TypeFunctionStringSingleton {}
