#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct NeverType {
    pub(crate) _unused: Option<core::convert::Infallible>,
}
