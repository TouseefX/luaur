use crate::records::ir_builder::IrBuilder;

#[allow(non_camel_case_types)]
pub type HostUserdataNamecallHandler = Option<
    unsafe extern "C" fn(
        builder: *mut IrBuilder,
        r#type: u8,
        member: *const core::ffi::c_char,
        member_length: usize,
        arg_res_reg: i32,
        source_reg: i32,
        params: i32,
        results: i32,
        pcpos: i32,
    ) -> bool,
>;
