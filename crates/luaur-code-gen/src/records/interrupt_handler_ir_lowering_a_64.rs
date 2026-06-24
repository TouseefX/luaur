use crate::records::label::Label;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct InterruptHandler {
    pub self_: Label,
    pub pcpos: u32,
    pub next: Label,
}

impl Default for InterruptHandler {
    fn default() -> Self {
        Self {
            self_: Label::default(),
            pcpos: 0,
            next: Label::default(),
        }
    }
}
