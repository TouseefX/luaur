#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct SharedCodeGenContextDeleter {
    pub(crate) _unused: [u8; 0],
}
