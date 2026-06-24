use crate::enums::ir_cmd::IrCmd;

#[inline]
pub fn is_non_terminating_jump(cmd: IrCmd) -> bool {
    match cmd {
        IrCmd::TRY_NUM_TO_INDEX
        | IrCmd::TRY_CALL_FASTGETTM
        | IrCmd::CHECK_FASTCALL_RES
        | IrCmd::CHECK_TAG
        | IrCmd::CHECK_TRUTHY
        | IrCmd::CHECK_READONLY
        | IrCmd::CHECK_NO_METATABLE
        | IrCmd::CHECK_SAFE_ENV
        | IrCmd::CHECK_ARRAY_SIZE
        | IrCmd::CHECK_SLOT_MATCH
        | IrCmd::CHECK_NODE_NO_NEXT
        | IrCmd::CHECK_NODE_VALUE
        | IrCmd::CHECK_BUFFER_LEN
        | IrCmd::CHECK_USERDATA_TAG
        | IrCmd::CHECK_CMP_NUM
        | IrCmd::CHECK_CMP_INT
        | IrCmd::CHECK_CMP_INT64
        | IrCmd::CHECK_DIV_INT64 => true,
        _ => false,
    }
}
