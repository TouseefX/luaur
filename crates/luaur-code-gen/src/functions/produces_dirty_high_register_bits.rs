use crate::enums::ir_cmd::IrCmd;

#[inline]
pub fn produces_dirty_high_register_bits(cmd: IrCmd) -> bool {
    cmd == IrCmd::NUM_TO_UINT || cmd == IrCmd::INVOKE_FASTCALL || cmd == IrCmd::CMP_ANY
}
