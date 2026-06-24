#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(non_camel_case_types)]
pub enum TypeKind {
    Kind_Unknown,
    /// primitive type supported by VM - boolean/userdata/etc. No differentiation between types of userdata.
    Kind_Primitive,
    /// TODO: deprecated and not set, but read in 'visit'
    Kind_Vector,
    /// custom userdata type
    Kind_Userdata,
}

impl TypeKind {
    pub const Kind_Unknown: Self = Self::Kind_Unknown;
    pub const Kind_Primitive: Self = Self::Kind_Primitive;
    pub const Kind_Vector: Self = Self::Kind_Vector;
    pub const Kind_Userdata: Self = Self::Kind_Userdata;
}
