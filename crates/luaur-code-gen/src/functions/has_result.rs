use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_value_kind::IrValueKind;
use crate::functions::get_cmd_value_kind::get_cmd_value_kind;

#[inline]
pub fn has_result(cmd: IrCmd) -> bool {
    get_cmd_value_kind(cmd) != IrValueKind::None
}
