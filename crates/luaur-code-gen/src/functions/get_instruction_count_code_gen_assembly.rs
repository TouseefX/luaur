use crate::functions::get_op_length::get_op_length;
use crate::type_aliases::instruction_ir_builder::Instruction;
use luaur_common::enums::luau_opcode::LuauOpcode;
use luaur_common::macros::luau_insn_op::LUAU_INSN_OP;

pub fn get_instruction_count_instruction_size(insns: *const Instruction, size: u32) -> u32 {
    let mut count: u32 = 0;
    let mut i: u32 = 0;

    while i < size {
        unsafe {
            count += 1;
            let op = LUAU_INSN_OP(*insns.add(i as usize)) as u8;
            let op_enum = LuauOpcode::from(op);
            i += get_op_length(op_enum) as u32;
        }
    }

    count
}
