use crate::records::label::Label;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct ModuleHelpers {
    pub exitContinueVm: Label,
    pub exitNoContinueVm: Label,
    pub exitContinueVmClearNativeFlag: Label,
    pub updatePcAndContinueInVm: Label,
    pub return_: Label,
    pub interrupt: Label,
    pub continueCall: Label,
}

#[allow(non_upper_case_globals)]
impl ModuleHelpers {
    pub const exitContinueVm: Label = Label {
        id: 0,
        location: !0u32,
    };
    pub const exitNoContinueVm: Label = Label {
        id: 0,
        location: !0u32,
    };
    pub const exitContinueVmClearNativeFlag: Label = Label {
        id: 0,
        location: !0u32,
    };
    pub const updatePcAndContinueInVm: Label = Label {
        id: 0,
        location: !0u32,
    };
    pub const return_: Label = Label {
        id: 0,
        location: !0u32,
    };
    pub const interrupt: Label = Label {
        id: 0,
        location: !0u32,
    };
    pub const continueCall: Label = Label {
        id: 0,
        location: !0u32,
    };
}
