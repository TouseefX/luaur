#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Nothing {
    pub(crate) _unused: Option<core::convert::Infallible>,
}
