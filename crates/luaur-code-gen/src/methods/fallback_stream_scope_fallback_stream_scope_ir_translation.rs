use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_op_kind::IrOpKind;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::fallback_stream_scope::FallbackStreamScope;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;

impl<'a> FallbackStreamScope<'a> {
    pub fn fallback_stream_scope(&mut self, build: &mut IrBuilder, fallback: IrOp, next: IrOp) {
        CODEGEN_ASSERT!(fallback.kind() == IrOpKind::Block);
        CODEGEN_ASSERT!(next.kind() == IrOpKind::Block);

        build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
        build.begin_block(fallback);
    }
}
