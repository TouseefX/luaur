#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(non_camel_case_types)]
pub enum Context {
    CovariantContext,
    InvariantContext,
}

impl Context {
    pub const Covariant: Context = Context::CovariantContext;
    pub const Invariant: Context = Context::InvariantContext;
}
