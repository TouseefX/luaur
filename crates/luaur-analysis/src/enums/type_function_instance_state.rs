#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(non_camel_case_types)]
pub enum TypeFunctionInstanceState {
    /// Indicates that further reduction might be possible.
    Unsolved,

    /// Further reduction is not possible because one of the parameters is generic.
    Solved,

    /// Further reduction is not possible because the application is undefined.
    /// This always indicates an error in the code.
    ///
    /// eg add<nil, nil>
    Stuck,
}

impl TypeFunctionInstanceState {
    pub const Unsolved: Self = Self::Unsolved;
    pub const Solved: Self = Self::Solved;
    pub const Stuck: Self = Self::Stuck;
}

impl Default for TypeFunctionInstanceState {
    fn default() -> Self {
        Self::Unsolved
    }
}
