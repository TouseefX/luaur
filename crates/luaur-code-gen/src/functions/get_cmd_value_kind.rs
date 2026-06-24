use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_value_kind::IrValueKind;
use luaur_common::macros::luau_unreachable::LUAU_UNREACHABLE;

pub fn get_cmd_value_kind(cmd: IrCmd) -> IrValueKind {
    match cmd {
        IrCmd::NOP => IrValueKind::None,
        IrCmd::LOAD_TAG => IrValueKind::Tag,
        IrCmd::LOAD_POINTER => IrValueKind::Pointer,
        IrCmd::LOAD_DOUBLE => IrValueKind::Double,
        IrCmd::LOAD_INT => IrValueKind::Int,
        IrCmd::LOAD_FLOAT => IrValueKind::Float,
        IrCmd::LOAD_TVALUE => IrValueKind::Tvalue,
        IrCmd::LOAD_ENV
        | IrCmd::GET_ARR_ADDR
        | IrCmd::GET_SLOT_NODE_ADDR
        | IrCmd::GET_HASH_NODE_ADDR
        | IrCmd::GET_CLOSURE_UPVAL_ADDR => IrValueKind::Pointer,
        IrCmd::STORE_TAG
        | IrCmd::STORE_EXTRA
        | IrCmd::STORE_POINTER
        | IrCmd::STORE_DOUBLE
        | IrCmd::STORE_INT
        | IrCmd::STORE_INT64
        | IrCmd::STORE_VECTOR
        | IrCmd::STORE_TVALUE
        | IrCmd::STORE_SPLIT_TVALUE
        | IrCmd::CHECK_DIV_INT64 => IrValueKind::None,
        IrCmd::LOAD_INT64
        | IrCmd::ADD_INT64
        | IrCmd::SUB_INT64
        | IrCmd::MUL_INT64
        | IrCmd::DIV_INT64
        | IrCmd::IDIV_INT64
        | IrCmd::UDIV_INT64
        | IrCmd::REM_INT64
        | IrCmd::UREM_INT64
        | IrCmd::MOD_INT64
        | IrCmd::SELECT_INT64
        | IrCmd::BITAND_INT64
        | IrCmd::BITXOR_INT64
        | IrCmd::BITOR_INT64
        | IrCmd::BITNOT_INT64
        | IrCmd::BITLSHIFT_INT64
        | IrCmd::BITRSHIFT_INT64
        | IrCmd::BITARSHIFT_INT64
        | IrCmd::BITLROTATE_INT64
        | IrCmd::BITRROTATE_INT64
        | IrCmd::BITCOUNTLZ_INT64
        | IrCmd::BITCOUNTRZ_INT64
        | IrCmd::BYTESWAP_INT64 => IrValueKind::Int64,
        IrCmd::ADD_INT | IrCmd::SUB_INT | IrCmd::SEXTI8_INT | IrCmd::SEXTI16_INT => {
            IrValueKind::Int
        }
        IrCmd::ADD_NUM
        | IrCmd::SUB_NUM
        | IrCmd::MUL_NUM
        | IrCmd::DIV_NUM
        | IrCmd::IDIV_NUM
        | IrCmd::MOD_NUM
        | IrCmd::MIN_NUM
        | IrCmd::MAX_NUM
        | IrCmd::UNM_NUM
        | IrCmd::FLOOR_NUM
        | IrCmd::CEIL_NUM
        | IrCmd::ROUND_NUM
        | IrCmd::SQRT_NUM
        | IrCmd::ABS_NUM
        | IrCmd::SIGN_NUM
        | IrCmd::SELECT_NUM
        | IrCmd::MULADD_NUM => IrValueKind::Double,
        IrCmd::ADD_FLOAT
        | IrCmd::SUB_FLOAT
        | IrCmd::MUL_FLOAT
        | IrCmd::DIV_FLOAT
        | IrCmd::MIN_FLOAT
        | IrCmd::MAX_FLOAT
        | IrCmd::UNM_FLOAT
        | IrCmd::FLOOR_FLOAT
        | IrCmd::CEIL_FLOAT
        | IrCmd::SQRT_FLOAT
        | IrCmd::ABS_FLOAT
        | IrCmd::SIGN_FLOAT => IrValueKind::Float,
        IrCmd::ADD_VEC
        | IrCmd::SUB_VEC
        | IrCmd::MUL_VEC
        | IrCmd::DIV_VEC
        | IrCmd::IDIV_VEC
        | IrCmd::UNM_VEC
        | IrCmd::MIN_VEC
        | IrCmd::MAX_VEC
        | IrCmd::FLOOR_VEC
        | IrCmd::CEIL_VEC
        | IrCmd::ABS_VEC
        | IrCmd::SELECT_VEC
        | IrCmd::SELECT_IF_TRUTHY
        | IrCmd::MULADD_VEC => IrValueKind::Tvalue,
        IrCmd::DOT_VEC | IrCmd::EXTRACT_VEC => IrValueKind::Float,
        IrCmd::NOT_ANY
        | IrCmd::CMP_ANY
        | IrCmd::CMP_INT
        | IrCmd::CMP_INT64
        | IrCmd::CMP_TAG
        | IrCmd::CMP_SPLIT_TVALUE => IrValueKind::Int,
        IrCmd::JUMP
        | IrCmd::JUMP_IF_TRUTHY
        | IrCmd::JUMP_IF_FALSY
        | IrCmd::JUMP_EQ_TAG
        | IrCmd::JUMP_CMP_INT
        | IrCmd::JUMP_EQ_POINTER
        | IrCmd::JUMP_CMP_NUM
        | IrCmd::JUMP_CMP_FLOAT
        | IrCmd::JUMP_FORN_LOOP_COND
        | IrCmd::JUMP_SLOT_MATCH => IrValueKind::None,
        IrCmd::TABLE_LEN => IrValueKind::Int,
        IrCmd::TABLE_SETNUM => IrValueKind::Pointer,
        IrCmd::STRING_LEN => IrValueKind::Int,
        IrCmd::NEW_TABLE | IrCmd::DUP_TABLE => IrValueKind::Pointer,
        IrCmd::TRY_NUM_TO_INDEX => IrValueKind::Int,
        IrCmd::TRY_CALL_FASTGETTM | IrCmd::NEW_USERDATA => IrValueKind::Pointer,
        IrCmd::INT64_TO_NUM | IrCmd::INT_TO_NUM | IrCmd::UINT_TO_NUM => IrValueKind::Double,
        IrCmd::UINT_TO_FLOAT => IrValueKind::Float,
        IrCmd::NUM_TO_INT | IrCmd::NUM_TO_UINT => IrValueKind::Int,
        IrCmd::NUM_TO_INT64 => IrValueKind::Int64,
        IrCmd::FLOAT_TO_NUM => IrValueKind::Double,
        IrCmd::NUM_TO_FLOAT => IrValueKind::Float,
        IrCmd::FLOAT_TO_VEC | IrCmd::TAG_VECTOR => IrValueKind::Tvalue,
        IrCmd::TRUNCATE_UINT => IrValueKind::Int,
        IrCmd::ADJUST_STACK_TO_REG | IrCmd::ADJUST_STACK_TO_TOP | IrCmd::FASTCALL => {
            IrValueKind::None
        }
        IrCmd::INVOKE_FASTCALL => IrValueKind::Int,
        IrCmd::CHECK_FASTCALL_RES
        | IrCmd::DO_ARITH
        | IrCmd::DO_LEN
        | IrCmd::GET_TABLE
        | IrCmd::SET_TABLE
        | IrCmd::GET_CACHED_IMPORT
        | IrCmd::CONCAT => IrValueKind::None,
        IrCmd::GET_UPVALUE => IrValueKind::Tvalue,
        IrCmd::SET_UPVALUE
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
        | IrCmd::INTERRUPT
        | IrCmd::CHECK_GC
        | IrCmd::BARRIER_OBJ
        | IrCmd::BARRIER_TABLE_BACK
        | IrCmd::BARRIER_TABLE_FORWARD
        | IrCmd::SET_SAVEDPC
        | IrCmd::CLOSE_UPVALS
        | IrCmd::CAPTURE
        | IrCmd::SETLIST
        | IrCmd::CALL
        | IrCmd::RETURN
        | IrCmd::FORGLOOP
        | IrCmd::FORGLOOP_FALLBACK
        | IrCmd::FORGPREP_XNEXT_FALLBACK
        | IrCmd::COVERAGE
        | IrCmd::FALLBACK_GETGLOBAL
        | IrCmd::FALLBACK_SETGLOBAL
        | IrCmd::FALLBACK_GETTABLEKS
        | IrCmd::FALLBACK_SETTABLEKS
        | IrCmd::FALLBACK_NAMECALL
        | IrCmd::FALLBACK_PREPVARARGS
        | IrCmd::FALLBACK_GETVARARGS => IrValueKind::None,
        IrCmd::NEWCLOSURE => IrValueKind::Pointer,
        IrCmd::FALLBACK_DUPCLOSURE | IrCmd::FALLBACK_FORGPREP => IrValueKind::None,
        IrCmd::SUBSTITUTE => IrValueKind::Unknown,
        IrCmd::MARK_USED | IrCmd::MARK_DEAD => IrValueKind::None,
        IrCmd::BITAND_UINT
        | IrCmd::BITXOR_UINT
        | IrCmd::BITOR_UINT
        | IrCmd::BITNOT_UINT
        | IrCmd::BITLSHIFT_UINT
        | IrCmd::BITRSHIFT_UINT
        | IrCmd::BITARSHIFT_UINT
        | IrCmd::BITLROTATE_UINT
        | IrCmd::BITRROTATE_UINT
        | IrCmd::BITCOUNTLZ_UINT
        | IrCmd::BITCOUNTRZ_UINT
        | IrCmd::BYTESWAP_UINT => IrValueKind::Int,
        IrCmd::INVOKE_LIBM => IrValueKind::Double,
        IrCmd::GET_TYPE | IrCmd::GET_TYPEOF | IrCmd::FINDUPVAL => IrValueKind::Pointer,
        IrCmd::BUFFER_READI8
        | IrCmd::BUFFER_READU8
        | IrCmd::BUFFER_READI16
        | IrCmd::BUFFER_READU16
        | IrCmd::BUFFER_READI32 => IrValueKind::Int,
        IrCmd::BUFFER_READI64 => IrValueKind::Int64,
        IrCmd::BUFFER_WRITEI8
        | IrCmd::BUFFER_WRITEI16
        | IrCmd::BUFFER_WRITEI32
        | IrCmd::BUFFER_WRITEF32
        | IrCmd::BUFFER_WRITEF64
        | IrCmd::BUFFER_WRITEI64 => IrValueKind::None,
        IrCmd::BUFFER_READF32 => IrValueKind::Float,
        IrCmd::BUFFER_READF64 => IrValueKind::Double,
        IrCmd::JUMP_CMP_PROTOID => IrValueKind::None,
        _ => LUAU_UNREACHABLE!(),
    }
}
