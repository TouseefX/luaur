use crate::records::ir_builder::IrBuilder;

#[allow(non_camel_case_types)]
pub type HostVectorAccessHandler = Option<
    unsafe extern "C" fn(
        builder: *mut IrBuilder,
        member: *const core::ffi::c_char,
        member_length: usize,
        result_reg: i32,
        source_reg: i32,
        pcpos: i32,
    ) -> bool,
>;
