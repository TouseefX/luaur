use crate::enums::ir_cmd::IrCmd;

#[inline]
pub fn can_invalidate_safe_env(cmd: IrCmd) -> bool {
    match cmd {
        IrCmd::CMP_ANY
        | IrCmd::DO_ARITH
        | IrCmd::DO_LEN
        | IrCmd::GET_TABLE
        | IrCmd::SET_TABLE
        | IrCmd::CONCAT // TODO: if only strings and numbers are concatenated, there will be no user calls
        | IrCmd::CALL
        | IrCmd::FORGLOOP_FALLBACK
        | IrCmd::FALLBACK_GETGLOBAL
        | IrCmd::FALLBACK_SETGLOBAL
        | IrCmd::FALLBACK_GETTABLEKS
        | IrCmd::FALLBACK_SETTABLEKS
        | IrCmd::FALLBACK_NAMECALL
        | IrCmd::FALLBACK_FORGPREP => true,
        _ => false,
    }
}
