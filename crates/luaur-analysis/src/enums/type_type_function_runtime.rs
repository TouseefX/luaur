#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum Type {
    NilType,
    Boolean,
    Number,
    Integer,
    String,
    Thread,
    Buffer,
}

impl Type {
    pub const NilType: Self = Self::NilType;
    pub const Boolean: Self = Self::Boolean;
    pub const Number: Self = Self::Number;
    pub const Integer: Self = Self::Integer;
    pub const String: Self = Self::String;
    pub const Thread: Self = Self::Thread;
    pub const Buffer: Self = Self::Buffer;
}
