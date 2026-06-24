#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum State {
    Initial,
    Normal,
    Property,
    PendingIs,
    PendingAs,
    PendingWhich,
}
