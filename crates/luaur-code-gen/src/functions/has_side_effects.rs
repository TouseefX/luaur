use crate::enums::ir_cmd::IrCmd;
use crate::functions::has_result::has_result;
use crate::functions::is_pseudo::is_pseudo;

pub fn has_side_effects(cmd: IrCmd) -> bool {
    if cmd == IrCmd::INVOKE_FASTCALL {
        return true;
    }

    if is_pseudo(cmd) {
        return false;
    }

    // Instructions that don't produce a result most likely have other side-effects to make them useful
    // Right now, a full switch would mirror the 'hasResult' function, so we use this simple condition
    !has_result(cmd)
}
