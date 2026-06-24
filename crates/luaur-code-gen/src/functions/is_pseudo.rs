use crate::enums::ir_cmd::IrCmd;

#[inline]
pub fn is_pseudo(cmd: IrCmd) -> bool {
    // Instructions that are used for internal needs and are not a part of final lowering
    match cmd {
        IrCmd::NOP | IrCmd::SUBSTITUTE | IrCmd::MARK_USED | IrCmd::MARK_DEAD => true,
        _ => false,
    }
}
