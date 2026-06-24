#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct TypeLevel {
    pub(crate) level: i32,
    pub(crate) subLevel: i32,
}

impl TypeLevel {
    pub(crate) const fn new(level: i32, sub_level: i32) -> Self {
        Self {
            level,
            subLevel: sub_level,
        }
    }
}

#[allow(non_upper_case_globals)]
pub const TypeLevel_Default: TypeLevel = TypeLevel {
    level: 0,
    subLevel: 0,
};
