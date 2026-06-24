use alloc::string::String;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct StringSingleton {
    pub value: String,
}

#[allow(non_snake_case)]
impl StringSingleton {
    pub const fn new(value: String) -> Self {
        Self { value }
    }
}

unsafe impl Send for StringSingleton {}
unsafe impl Sync for StringSingleton {}
