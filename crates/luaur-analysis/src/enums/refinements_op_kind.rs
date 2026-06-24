#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum RefinementsOpKind {
    Intersect,
    Refine,
    None,
}

impl RefinementsOpKind {
    pub const Intersect: Self = Self::Intersect;
    pub const Refine: Self = Self::Refine;
    pub const None: Self = Self::None;
}

impl Default for RefinementsOpKind {
    fn default() -> Self {
        Self::None
    }
}
