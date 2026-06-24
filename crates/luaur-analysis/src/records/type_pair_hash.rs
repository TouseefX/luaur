#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct TypePairHash {
    pub(crate) _unused: Option<core::convert::Infallible>,
}

unsafe impl Send for TypePairHash {}
unsafe impl Sync for TypePairHash {}
