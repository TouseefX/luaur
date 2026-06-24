use crate::enums::ir_cmd::IrCmd;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_translation::Instruction;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;
use luaur_common::macros::luau_insn_d::LUAU_INSN_D;

pub fn translate_inst_get_import(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
    let pc_val = unsafe { *pc };
    let ra = LUAU_INSN_A(pc_val) as u8;
    let k = LUAU_INSN_D(pc_val) as u32;
    let aux = unsafe { *pc.add(1) };

    build.check_safe_env(pcpos);

    let ra_op = build.vm_reg(ra);
    let k_op = build.vm_const(k);
    let aux_op = build.const_import(aux);
    let pcpos_op = build.const_uint((pcpos + 1) as u32);

    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
        IrCmd::GET_CACHED_IMPORT,
        ra_op,
        k_op,
        aux_op,
        pcpos_op,
    );
}
