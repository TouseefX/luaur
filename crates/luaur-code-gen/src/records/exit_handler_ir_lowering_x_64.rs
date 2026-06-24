use crate::records::label::Label;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct ExitHandler {
    pub self_: Label,
    pub pcpos: u32,
}

impl Default for ExitHandler {
    fn default() -> Self {
        Self {
            self_: Label::default(),
            pcpos: 0,
        }
    }
}
