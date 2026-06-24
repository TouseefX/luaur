//! Node: `cxx:Enum:Luau.Analysis:Analysis/include/Luau/Type.h:160:type`
//! Source: `Analysis/include/Luau/Type.h` (Type.h:158-178, hand-ported)

// PrimitiveType::Type — the nested enum of struct PrimitiveType.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    NilType, // ObjC #defines Nil :(
    Boolean,
    Number,
    Integer,
    String,
    Thread,
    Function,
    Table,
    Buffer,
}

// Keep the previous placeholder name alive for any early consumers.
pub use Type as TypeItem;
