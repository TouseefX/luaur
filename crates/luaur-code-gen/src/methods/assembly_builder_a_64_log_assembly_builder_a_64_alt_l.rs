use crate::enums::condition_a_64::ConditionA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn log_c_char_register_a_64_register_a_64_register_a_64_condition_a_64(
        &mut self,
        opcode: *const core::ffi::c_char,
        dst: RegisterA64,
        src1: RegisterA64,
        src2: RegisterA64,
        cond: ConditionA64,
    ) {
        self.log_append(format_args!(" {:<12}", unsafe {
            core::ffi::CStr::from_ptr(opcode).to_string_lossy()
        }));
        self.log_register_a_64(dst);

        let wzr = RegisterA64::wzr;
        let xzr = RegisterA64::xzr;

        if !src1.register_a_64_operator_eq(wzr) && !src1.register_a_64_operator_eq(xzr)
            || !src2.register_a_64_operator_eq(wzr) && !src2.register_a_64_operator_eq(xzr)
        {
            self.log_append(format_args!(","));
            self.log_register_a_64(src1);
            self.log_append(format_args!(","));
            self.log_register_a_64(src2);
        }

        self.log_append(format_args!(","));

        let text_for_condition = [
            "eq", "ne", "cs", "cc", "mi", "pl", "vs", "vc", "hi", "ls", "ge", "lt", "gt", "le",
            "al", "nv",
        ];

        let cond_idx = cond as usize;
        if cond_idx < text_for_condition.len() {
            self.log_append(format_args!("{}", text_for_condition[cond_idx]));
        }

        self.log_append(format_args!("\n"));
    }
}
