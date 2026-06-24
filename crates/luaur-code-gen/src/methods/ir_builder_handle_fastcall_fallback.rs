use crate::enums::ir_cmd::IrCmd;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;
use crate::type_aliases::instruction_ir_builder::Instruction;

use luaur_common::macros::luau_insn_c::LUAU_INSN_C;

impl IrBuilder {
    pub fn handle_fastcall_fallback(
        &mut self,
        fallback_or_undef: IrOp,
        pc: *const Instruction,
        i: i32,
    ) {
        let skip = unsafe { LUAU_INSN_C(*pc as u32) } as i32;

        if fallback_or_undef.kind() != crate::enums::ir_op_kind::IrOpKind::Undef {
            let next = self.block_at_inst((i + skip + 2) as u32);
            self.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
            self.begin_block(fallback_or_undef);

            self.active_fastcall_fallback = true;
            self.fastcall_fallback_return = next;
        } else {
            self.cmd_skip_target = i + skip + 2;
        }
    }
}
