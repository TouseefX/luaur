use crate::records::label::Label;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EntryLocations {
    pub start: Label,
    pub prologueEnd: Label,
    pub epilogueStart: Label,
}

#[allow(non_upper_case_globals)]
impl EntryLocations {
    pub const start: Label = Label {
        id: 0,
        location: !0u32,
    };
    pub const prologueEnd: Label = Label {
        id: 0,
        location: !0u32,
    };
    pub const epilogueStart: Label = Label {
        id: 0,
        location: !0u32,
    };
}
