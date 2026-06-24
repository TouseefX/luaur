use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_op_kind::IrOpKind;
use crate::records::fallback_stream_scope::FallbackStreamScope;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;
use luaur_vm::type_aliases::tms::TMS;

pub fn translate_binary_numeric_fallback_if_required(
    build: &mut IrBuilder,
    fallback: IrOp,
    ra: i32,
    opb: IrOp,
    opc: IrOp,
    tm: TMS,
    pcpos: i32,
) {
    if fallback.kind() != IrOpKind::None {
        let next = build.block_at_inst((pcpos + 1) as u32);
        let mut scope = FallbackStreamScope::new(build, fallback, next);

        let pcpos_op = scope.build.const_uint((pcpos + 1) as u32);
        scope.build.inst_ir_cmd_ir_op(IrCmd::SET_SAVEDPC, pcpos_op);

        let reg_ra = scope.build.vm_reg(ra as u8);
        let tm_op = scope.build.const_int(tm as i32);
        scope
            .build
            .inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::DO_ARITH, reg_ra, opb, opc, tm_op);

        scope.build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
    }
}
