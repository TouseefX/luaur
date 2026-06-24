use crate::records::function_bytecode_summary::FunctionBytecodeSummary;
use crate::type_aliases::instruction_ir_builder::Instruction;
use alloc::string::String;
use core::ffi::c_char;
use luaur_common::enums::luau_opcode::LuauOpcode;
use luaur_common::macros::luau_insn_op::LUAU_INSN_OP;
use luaur_vm::macros::getstr::getstr;

impl FunctionBytecodeSummary {
    pub fn from_proto(proto: *mut luaur_vm::records::proto::Proto, nesting_limit: u32) -> Self {
        unsafe {
            let source_ptr = getstr((*proto).source);
            let source_cstr = core::ffi::CStr::from_ptr(source_ptr);
            let mut source = source_cstr.to_string_lossy().into_owned();

            // Strip leading '=' or '@' if present
            if source.starts_with('=') || source.starts_with('@') {
                source = source[1..].to_string();
            } else {
                source = "[string]".to_string();
            }

            let name = if !(*proto).debugname.is_null() {
                let name_ptr = getstr((*proto).debugname);
                core::ffi::CStr::from_ptr(name_ptr)
                    .to_string_lossy()
                    .into_owned()
            } else {
                String::new()
            };

            let line = (*proto).linedefined;

            let mut summary = Self::new(source, name, line, nesting_limit);

            let mut i: usize = 0;
            while i < (*proto).sizecode as usize {
                let insn: Instruction = *(*proto).code.add(i);
                let op = LUAU_INSN_OP(insn) as u8;
                summary.inc_count(0, op);
                i += crate::functions::get_op_length::get_op_length(LuauOpcode::from(op)) as usize;
            }

            summary
        }
    }
}
