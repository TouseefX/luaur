use crate::enums::ir_cmd::IrCmd;

#[inline]
pub fn is_unsafe_to_sink(cmd: IrCmd) -> bool {
    match cmd {
        IrCmd::LOAD_TAG
        | IrCmd::LOAD_POINTER
        | IrCmd::LOAD_DOUBLE
        | IrCmd::LOAD_INT
        | IrCmd::LOAD_INT64
        | IrCmd::LOAD_FLOAT
        | IrCmd::LOAD_TVALUE
        | IrCmd::BUFFER_READI8
        | IrCmd::BUFFER_READU8
        | IrCmd::BUFFER_READI16
        | IrCmd::BUFFER_READU16
        | IrCmd::BUFFER_READI32
        | IrCmd::BUFFER_READI64
        | IrCmd::BUFFER_READF32
        | IrCmd::BUFFER_READF64
        | IrCmd::GET_UPVALUE
        | IrCmd::TABLE_LEN
        | IrCmd::GET_TYPEOF
        | IrCmd::TABLE_SETNUM
        | IrCmd::CMP_ANY
        | IrCmd::TRY_NUM_TO_INDEX
        | IrCmd::TRY_CALL_FASTGETTM => true,
        _ => false,
    }
}
