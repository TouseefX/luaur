use crate::enums::ir_cmd::IrCmd;

#[inline]
pub fn is_block_terminator(cmd: IrCmd) -> bool {
    match cmd {
        IrCmd::JUMP
        | IrCmd::JUMP_IF_TRUTHY
        | IrCmd::JUMP_IF_FALSY
        | IrCmd::JUMP_EQ_TAG
        | IrCmd::JUMP_CMP_INT
        | IrCmd::JUMP_EQ_POINTER
        | IrCmd::JUMP_CMP_NUM
        | IrCmd::JUMP_CMP_FLOAT
        | IrCmd::JUMP_FORN_LOOP_COND
        | IrCmd::JUMP_SLOT_MATCH
        | IrCmd::RETURN
        | IrCmd::FORGLOOP
        | IrCmd::FORGLOOP_FALLBACK
        | IrCmd::FORGPREP_XNEXT_FALLBACK
        | IrCmd::FALLBACK_FORGPREP
        | IrCmd::JUMP_CMP_PROTOID => true,
        _ => false,
    }
}
