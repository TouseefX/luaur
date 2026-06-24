#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Variant {
    Pack,
    Union,
    Intersection,
}

impl Default for Variant {
    fn default() -> Self {
        Self::Pack
    }
}

#[allow(non_upper_case_globals)]
impl Variant {
    pub const Pack: Self = Self::Pack;
    pub const Union: Self = Self::Union;
    pub const Intersection: Self = Self::Intersection;
}
