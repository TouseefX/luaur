use crate::type_aliases::type_id::TypeId;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type {
    NilType,
    Boolean,
    Number,
    Integer,
    String,
    Thread,
    Function,
    Table,
    Buffer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PrimitiveType {
    pub r#type: Type,
    pub metatable: Option<TypeId>,
}

impl PrimitiveType {
    pub const NilType: Type = Type::NilType;
    pub const Boolean: Type = Type::Boolean;
    pub const Number: Type = Type::Number;
    pub const Integer: Type = Type::Integer;
    pub const String: Type = Type::String;
    pub const Thread: Type = Type::Thread;
    pub const Function: Type = Type::Function;
    pub const Table: Type = Type::Table;
    pub const Buffer: Type = Type::Buffer;
}
