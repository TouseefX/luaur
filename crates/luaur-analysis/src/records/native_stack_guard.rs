#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NativeStackGuard {
    pub(crate) high: usize,
    pub(crate) low: usize,
}
