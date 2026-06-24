#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IrCmd {
    NOP,
    LOAD_TAG,
    LOAD_POINTER,
    LOAD_DOUBLE,
    LOAD_INT,
    LOAD_INT64,
    LOAD_FLOAT,
    LOAD_TVALUE,
    LOAD_ENV,
    GET_ARR_ADDR,
    GET_SLOT_NODE_ADDR,
    GET_HASH_NODE_ADDR,
    GET_CLOSURE_UPVAL_ADDR,
    STORE_TAG,
    STORE_EXTRA,
    STORE_POINTER,
    STORE_DOUBLE,
    STORE_INT,
    STORE_INT64,
    STORE_VECTOR,
    STORE_TVALUE,
    STORE_SPLIT_TVALUE,
    ADD_INT,
    SUB_INT,
    ADD_INT64,
    SUB_INT64,
    MUL_INT64,
    DIV_INT64,
    IDIV_INT64,
    UDIV_INT64,
    REM_INT64,
    UREM_INT64,
    MOD_INT64,
    CHECK_DIV_INT64,
    SEXTI8_INT,
    SEXTI16_INT,
    ADD_NUM,
    SUB_NUM,
    MUL_NUM,
    DIV_NUM,
    IDIV_NUM,
    MOD_NUM,
    MULADD_NUM,
    MIN_NUM,
    MAX_NUM,
    UNM_NUM,
    FLOOR_NUM,
    CEIL_NUM,
    ROUND_NUM,
    SQRT_NUM,
    ABS_NUM,
    SIGN_NUM,
    ADD_FLOAT,
    SUB_FLOAT,
    MUL_FLOAT,
    DIV_FLOAT,
    MIN_FLOAT,
    MAX_FLOAT,
    UNM_FLOAT,
    FLOOR_FLOAT,
    CEIL_FLOAT,
    SQRT_FLOAT,
    ABS_FLOAT,
    SIGN_FLOAT,
    SELECT_NUM,
    SELECT_INT64,
    SELECT_VEC,
    SELECT_IF_TRUTHY,
    ADD_VEC,
    SUB_VEC,
    MUL_VEC,
    DIV_VEC,
    IDIV_VEC,
    MULADD_VEC,
    UNM_VEC,
    MIN_VEC,
    MAX_VEC,
    FLOOR_VEC,
    CEIL_VEC,
    ABS_VEC,
    DOT_VEC,
    EXTRACT_VEC,
    NOT_ANY,
    CMP_ANY,
    CMP_INT,
    CMP_INT64,
    CMP_TAG,
    CMP_SPLIT_TVALUE,
    JUMP,
    JUMP_IF_TRUTHY,
    JUMP_IF_FALSY,
    JUMP_EQ_TAG,
    JUMP_CMP_INT,
    JUMP_EQ_POINTER,
    JUMP_CMP_NUM,
    JUMP_CMP_FLOAT,
    JUMP_FORN_LOOP_COND,
    JUMP_SLOT_MATCH,
    TABLE_LEN,
    STRING_LEN,
    NEW_TABLE,
    DUP_TABLE,
    TABLE_SETNUM,
    TRY_NUM_TO_INDEX,
    TRY_CALL_FASTGETTM,
    NEW_USERDATA,
    INT_TO_NUM,
    INT64_TO_NUM,
    UINT_TO_NUM,
    UINT_TO_FLOAT,
    NUM_TO_INT,
    NUM_TO_INT64,
    NUM_TO_UINT,
    FLOAT_TO_NUM,
    NUM_TO_FLOAT,
    FLOAT_TO_VEC,
    TAG_VECTOR,
    TRUNCATE_UINT,
    ADJUST_STACK_TO_REG,
    ADJUST_STACK_TO_TOP,
    FASTCALL,
    INVOKE_FASTCALL,
    CHECK_FASTCALL_RES,
    DO_ARITH,
    DO_LEN,
    GET_TABLE,
    SET_TABLE,
    GET_CACHED_IMPORT,
    CONCAT,
    GET_UPVALUE,
    SET_UPVALUE,
    CHECK_TAG,
    CHECK_TRUTHY,
    CHECK_READONLY,
    CHECK_NO_METATABLE,
    CHECK_SAFE_ENV,
    CHECK_ARRAY_SIZE,
    CHECK_SLOT_MATCH,
    CHECK_NODE_NO_NEXT,
    CHECK_NODE_VALUE,
    CHECK_BUFFER_LEN,
    CHECK_USERDATA_TAG,
    CHECK_CMP_NUM,
    CHECK_CMP_INT,
    CHECK_CMP_INT64,
    INTERRUPT,
    CHECK_GC,
    BARRIER_OBJ,
    BARRIER_TABLE_BACK,
    BARRIER_TABLE_FORWARD,
    SET_SAVEDPC,
    CLOSE_UPVALS,
    CAPTURE,
    SETLIST,
    CALL,
    RETURN,
    FORGLOOP,
    FORGLOOP_FALLBACK,
    FORGPREP_XNEXT_FALLBACK,
    COVERAGE,
    FALLBACK_GETGLOBAL,
    FALLBACK_SETGLOBAL,
    FALLBACK_GETTABLEKS,
    FALLBACK_SETTABLEKS,
    FALLBACK_NAMECALL,
    FALLBACK_PREPVARARGS,
    FALLBACK_GETVARARGS,
    NEWCLOSURE,
    FALLBACK_DUPCLOSURE,
    FALLBACK_FORGPREP,
    SUBSTITUTE,
    MARK_USED,
    MARK_DEAD,
    BITAND_INT64,
    BITXOR_INT64,
    BITOR_INT64,
    BITNOT_INT64,
    BITLSHIFT_INT64,
    BITRSHIFT_INT64,
    BITARSHIFT_INT64,
    BITLROTATE_INT64,
    BITRROTATE_INT64,
    BITCOUNTLZ_INT64,
    BITCOUNTRZ_INT64,
    BYTESWAP_INT64,
    BITAND_UINT,
    BITXOR_UINT,
    BITOR_UINT,
    BITNOT_UINT,
    BITLSHIFT_UINT,
    BITRSHIFT_UINT,
    BITARSHIFT_UINT,
    BITLROTATE_UINT,
    BITRROTATE_UINT,
    BITCOUNTLZ_UINT,
    BITCOUNTRZ_UINT,
    BYTESWAP_UINT,
    INVOKE_LIBM,
    GET_TYPE,
    GET_TYPEOF,
    FINDUPVAL,
    BUFFER_READI8,
    BUFFER_READU8,
    BUFFER_WRITEI8,
    BUFFER_READI16,
    BUFFER_READU16,
    BUFFER_WRITEI16,
    BUFFER_READI32,
    BUFFER_WRITEI32,
    BUFFER_READF32,
    BUFFER_WRITEF32,
    BUFFER_READF64,
    BUFFER_WRITEF64,
    BUFFER_READI64,
    BUFFER_WRITEI64,
    JUMP_CMP_PROTOID,
}

#[allow(non_upper_case_globals)]
impl IrCmd {
    pub const NOP: Self = Self::NOP;
    pub const LOAD_TAG: Self = Self::LOAD_TAG;
    pub const LOAD_POINTER: Self = Self::LOAD_POINTER;
    pub const LOAD_DOUBLE: Self = Self::LOAD_DOUBLE;
    pub const LOAD_INT: Self = Self::LOAD_INT;
    pub const LOAD_INT64: Self = Self::LOAD_INT64;
    pub const LOAD_FLOAT: Self = Self::LOAD_FLOAT;
    pub const LOAD_TVALUE: Self = Self::LOAD_TVALUE;
    pub const LOAD_ENV: Self = Self::LOAD_ENV;
    pub const GET_ARR_ADDR: Self = Self::GET_ARR_ADDR;
    pub const GET_SLOT_NODE_ADDR: Self = Self::GET_SLOT_NODE_ADDR;
    pub const GET_HASH_NODE_ADDR: Self = Self::GET_HASH_NODE_ADDR;
    pub const GET_CLOSURE_UPVAL_ADDR: Self = Self::GET_CLOSURE_UPVAL_ADDR;
    pub const STORE_TAG: Self = Self::STORE_TAG;
    pub const STORE_EXTRA: Self = Self::STORE_EXTRA;
    pub const STORE_POINTER: Self = Self::STORE_POINTER;
    pub const STORE_DOUBLE: Self = Self::STORE_DOUBLE;
    pub const STORE_INT: Self = Self::STORE_INT;
    pub const STORE_INT64: Self = Self::STORE_INT64;
    pub const STORE_VECTOR: Self = Self::STORE_VECTOR;
    pub const STORE_TVALUE: Self = Self::STORE_TVALUE;
    pub const STORE_SPLIT_TVALUE: Self = Self::STORE_SPLIT_TVALUE;
    pub const ADD_INT: Self = Self::ADD_INT;
    pub const SUB_INT: Self = Self::SUB_INT;
    pub const ADD_INT64: Self = Self::ADD_INT64;
    pub const SUB_INT64: Self = Self::SUB_INT64;
    pub const MUL_INT64: Self = Self::MUL_INT64;
    pub const DIV_INT64: Self = Self::DIV_INT64;
    pub const IDIV_INT64: Self = Self::IDIV_INT64;
    pub const UDIV_INT64: Self = Self::UDIV_INT64;
    pub const REM_INT64: Self = Self::REM_INT64;
    pub const UREM_INT64: Self = Self::UREM_INT64;
    pub const MOD_INT64: Self = Self::MOD_INT64;
    pub const CHECK_DIV_INT64: Self = Self::CHECK_DIV_INT64;
    pub const SEXTI8_INT: Self = Self::SEXTI8_INT;
    pub const SEXTI16_INT: Self = Self::SEXTI16_INT;
    pub const ADD_NUM: Self = Self::ADD_NUM;
    pub const SUB_NUM: Self = Self::SUB_NUM;
    pub const MUL_NUM: Self = Self::MUL_NUM;
    pub const DIV_NUM: Self = Self::DIV_NUM;
    pub const IDIV_NUM: Self = Self::IDIV_NUM;
    pub const MOD_NUM: Self = Self::MOD_NUM;
    pub const MULADD_NUM: Self = Self::MULADD_NUM;
    pub const MIN_NUM: Self = Self::MIN_NUM;
    pub const MAX_NUM: Self = Self::MAX_NUM;
    pub const UNM_NUM: Self = Self::UNM_NUM;
    pub const FLOOR_NUM: Self = Self::FLOOR_NUM;
    pub const CEIL_NUM: Self = Self::CEIL_NUM;
    pub const ROUND_NUM: Self = Self::ROUND_NUM;
    pub const SQRT_NUM: Self = Self::SQRT_NUM;
    pub const ABS_NUM: Self = Self::ABS_NUM;
    pub const SIGN_NUM: Self = Self::SIGN_NUM;
    pub const ADD_FLOAT: Self = Self::ADD_FLOAT;
    pub const SUB_FLOAT: Self = Self::SUB_FLOAT;
    pub const MUL_FLOAT: Self = Self::MUL_FLOAT;
    pub const DIV_FLOAT: Self = Self::DIV_FLOAT;
    pub const MIN_FLOAT: Self = Self::MIN_FLOAT;
    pub const MAX_FLOAT: Self = Self::MAX_FLOAT;
    pub const UNM_FLOAT: Self = Self::UNM_FLOAT;
    pub const FLOOR_FLOAT: Self = Self::FLOOR_FLOAT;
    pub const CEIL_FLOAT: Self = Self::CEIL_FLOAT;
    pub const SQRT_FLOAT: Self = Self::SQRT_FLOAT;
    pub const ABS_FLOAT: Self = Self::ABS_FLOAT;
    pub const SIGN_FLOAT: Self = Self::SIGN_FLOAT;
    pub const SELECT_NUM: Self = Self::SELECT_NUM;
    pub const SELECT_INT64: Self = Self::SELECT_INT64;
    pub const SELECT_VEC: Self = Self::SELECT_VEC;
    pub const SELECT_IF_TRUTHY: Self = Self::SELECT_IF_TRUTHY;
    pub const ADD_VEC: Self = Self::ADD_VEC;
    pub const SUB_VEC: Self = Self::SUB_VEC;
    pub const MUL_VEC: Self = Self::MUL_VEC;
    pub const DIV_VEC: Self = Self::DIV_VEC;
    pub const IDIV_VEC: Self = Self::IDIV_VEC;
    pub const MULADD_VEC: Self = Self::MULADD_VEC;
    pub const UNM_VEC: Self = Self::UNM_VEC;
    pub const MIN_VEC: Self = Self::MIN_VEC;
    pub const MAX_VEC: Self = Self::MAX_VEC;
    pub const FLOOR_VEC: Self = Self::FLOOR_VEC;
    pub const CEIL_VEC: Self = Self::CEIL_VEC;
    pub const ABS_VEC: Self = Self::ABS_VEC;
    pub const DOT_VEC: Self = Self::DOT_VEC;
    pub const EXTRACT_VEC: Self = Self::EXTRACT_VEC;
    pub const NOT_ANY: Self = Self::NOT_ANY;
    pub const CMP_ANY: Self = Self::CMP_ANY;
    pub const CMP_INT: Self = Self::CMP_INT;
    pub const CMP_INT64: Self = Self::CMP_INT64;
    pub const CMP_TAG: Self = Self::CMP_TAG;
    pub const CMP_SPLIT_TVALUE: Self = Self::CMP_SPLIT_TVALUE;
    pub const JUMP: Self = Self::JUMP;
    pub const JUMP_IF_TRUTHY: Self = Self::JUMP_IF_TRUTHY;
    pub const JUMP_IF_FALSY: Self = Self::JUMP_IF_FALSY;
    pub const JUMP_EQ_TAG: Self = Self::JUMP_EQ_TAG;
    pub const JUMP_CMP_INT: Self = Self::JUMP_CMP_INT;
    pub const JUMP_EQ_POINTER: Self = Self::JUMP_EQ_POINTER;
    pub const JUMP_CMP_NUM: Self = Self::JUMP_CMP_NUM;
    pub const JUMP_CMP_FLOAT: Self = Self::JUMP_CMP_FLOAT;
    pub const JUMP_FORN_LOOP_COND: Self = Self::JUMP_FORN_LOOP_COND;
    pub const JUMP_SLOT_MATCH: Self = Self::JUMP_SLOT_MATCH;
    pub const TABLE_LEN: Self = Self::TABLE_LEN;
    pub const STRING_LEN: Self = Self::STRING_LEN;
    pub const NEW_TABLE: Self = Self::NEW_TABLE;
    pub const DUP_TABLE: Self = Self::DUP_TABLE;
    pub const TABLE_SETNUM: Self = Self::TABLE_SETNUM;
    pub const TRY_NUM_TO_INDEX: Self = Self::TRY_NUM_TO_INDEX;
    pub const TRY_CALL_FASTGETTM: Self = Self::TRY_CALL_FASTGETTM;
    pub const NEW_USERDATA: Self = Self::NEW_USERDATA;
    pub const INT_TO_NUM: Self = Self::INT_TO_NUM;
    pub const INT64_TO_NUM: Self = Self::INT64_TO_NUM;
    pub const UINT_TO_NUM: Self = Self::UINT_TO_NUM;
    pub const UINT_TO_FLOAT: Self = Self::UINT_TO_FLOAT;
    pub const NUM_TO_INT: Self = Self::NUM_TO_INT;
    pub const NUM_TO_INT64: Self = Self::NUM_TO_INT64;
    pub const NUM_TO_UINT: Self = Self::NUM_TO_UINT;
    pub const FLOAT_TO_NUM: Self = Self::FLOAT_TO_NUM;
    pub const NUM_TO_FLOAT: Self = Self::NUM_TO_FLOAT;
    pub const FLOAT_TO_VEC: Self = Self::FLOAT_TO_VEC;
    pub const TAG_VECTOR: Self = Self::TAG_VECTOR;
    pub const TRUNCATE_UINT: Self = Self::TRUNCATE_UINT;
    pub const ADJUST_STACK_TO_REG: Self = Self::ADJUST_STACK_TO_REG;
    pub const ADJUST_STACK_TO_TOP: Self = Self::ADJUST_STACK_TO_TOP;
    pub const FASTCALL: Self = Self::FASTCALL;
    pub const INVOKE_FASTCALL: Self = Self::INVOKE_FASTCALL;
    pub const CHECK_FASTCALL_RES: Self = Self::CHECK_FASTCALL_RES;
    pub const DO_ARITH: Self = Self::DO_ARITH;
    pub const DO_LEN: Self = Self::DO_LEN;
    pub const GET_TABLE: Self = Self::GET_TABLE;
    pub const SET_TABLE: Self = Self::SET_TABLE;
    pub const GET_CACHED_IMPORT: Self = Self::GET_CACHED_IMPORT;
    pub const CONCAT: Self = Self::CONCAT;
    pub const GET_UPVALUE: Self = Self::GET_UPVALUE;
    pub const SET_UPVALUE: Self = Self::SET_UPVALUE;
    pub const CHECK_TAG: Self = Self::CHECK_TAG;
    pub const CHECK_TRUTHY: Self = Self::CHECK_TRUTHY;
    pub const CHECK_READONLY: Self = Self::CHECK_READONLY;
    pub const CHECK_NO_METATABLE: Self = Self::CHECK_NO_METATABLE;
    pub const CHECK_SAFE_ENV: Self = Self::CHECK_SAFE_ENV;
    pub const CHECK_ARRAY_SIZE: Self = Self::CHECK_ARRAY_SIZE;
    pub const CHECK_SLOT_MATCH: Self = Self::CHECK_SLOT_MATCH;
    pub const CHECK_NODE_NO_NEXT: Self = Self::CHECK_NODE_NO_NEXT;
    pub const CHECK_NODE_VALUE: Self = Self::CHECK_NODE_VALUE;
    pub const CHECK_BUFFER_LEN: Self = Self::CHECK_BUFFER_LEN;
    pub const CHECK_USERDATA_TAG: Self = Self::CHECK_USERDATA_TAG;
    pub const CHECK_CMP_NUM: Self = Self::CHECK_CMP_NUM;
    pub const CHECK_CMP_INT: Self = Self::CHECK_CMP_INT;
    pub const CHECK_CMP_INT64: Self = Self::CHECK_CMP_INT64;
    pub const INTERRUPT: Self = Self::INTERRUPT;
    pub const CHECK_GC: Self = Self::CHECK_GC;
    pub const BARRIER_OBJ: Self = Self::BARRIER_OBJ;
    pub const BARRIER_TABLE_BACK: Self = Self::BARRIER_TABLE_BACK;
    pub const BARRIER_TABLE_FORWARD: Self = Self::BARRIER_TABLE_FORWARD;
    pub const SET_SAVEDPC: Self = Self::SET_SAVEDPC;
    pub const CLOSE_UPVALS: Self = Self::CLOSE_UPVALS;
    pub const CAPTURE: Self = Self::CAPTURE;
    pub const SETLIST: Self = Self::SETLIST;
    pub const CALL: Self = Self::CALL;
    pub const RETURN: Self = Self::RETURN;
    pub const FORGLOOP: Self = Self::FORGLOOP;
    pub const FORGLOOP_FALLBACK: Self = Self::FORGLOOP_FALLBACK;
    pub const FORGPREP_XNEXT_FALLBACK: Self = Self::FORGPREP_XNEXT_FALLBACK;
    pub const COVERAGE: Self = Self::COVERAGE;
    pub const FALLBACK_GETGLOBAL: Self = Self::FALLBACK_GETGLOBAL;
    pub const FALLBACK_SETGLOBAL: Self = Self::FALLBACK_SETGLOBAL;
    pub const FALLBACK_GETTABLEKS: Self = Self::FALLBACK_GETTABLEKS;
    pub const FALLBACK_SETTABLEKS: Self = Self::FALLBACK_SETTABLEKS;
    pub const FALLBACK_NAMECALL: Self = Self::FALLBACK_NAMECALL;
    pub const FALLBACK_PREPVARARGS: Self = Self::FALLBACK_PREPVARARGS;
    pub const FALLBACK_GETVARARGS: Self = Self::FALLBACK_GETVARARGS;
    pub const NEWCLOSURE: Self = Self::NEWCLOSURE;
    pub const FALLBACK_DUPCLOSURE: Self = Self::FALLBACK_DUPCLOSURE;
    pub const FALLBACK_FORGPREP: Self = Self::FALLBACK_FORGPREP;
    pub const SUBSTITUTE: Self = Self::SUBSTITUTE;
    pub const MARK_USED: Self = Self::MARK_USED;
    pub const MARK_DEAD: Self = Self::MARK_DEAD;
    pub const BITAND_INT64: Self = Self::BITAND_INT64;
    pub const BITXOR_INT64: Self = Self::BITXOR_INT64;
    pub const BITOR_INT64: Self = Self::BITOR_INT64;
    pub const BITNOT_INT64: Self = Self::BITNOT_INT64;
    pub const BITLSHIFT_INT64: Self = Self::BITLSHIFT_INT64;
    pub const BITRSHIFT_INT64: Self = Self::BITRSHIFT_INT64;
    pub const BITARSHIFT_INT64: Self = Self::BITARSHIFT_INT64;
    pub const BITLROTATE_INT64: Self = Self::BITLROTATE_INT64;
    pub const BITRROTATE_INT64: Self = Self::BITRROTATE_INT64;
    pub const BITCOUNTLZ_INT64: Self = Self::BITCOUNTLZ_INT64;
    pub const BITCOUNTRZ_INT64: Self = Self::BITCOUNTRZ_INT64;
    pub const BYTESWAP_INT64: Self = Self::BYTESWAP_INT64;
    pub const BITAND_UINT: Self = Self::BITAND_UINT;
    pub const BITXOR_UINT: Self = Self::BITXOR_UINT;
    pub const BITOR_UINT: Self = Self::BITOR_UINT;
    pub const BITNOT_UINT: Self = Self::BITNOT_UINT;
    pub const BITLSHIFT_UINT: Self = Self::BITLSHIFT_UINT;
    pub const BITRSHIFT_UINT: Self = Self::BITRSHIFT_UINT;
    pub const BITARSHIFT_UINT: Self = Self::BITARSHIFT_UINT;
    pub const BITLROTATE_UINT: Self = Self::BITLROTATE_UINT;
    pub const BITRROTATE_UINT: Self = Self::BITRROTATE_UINT;
    pub const BITCOUNTLZ_UINT: Self = Self::BITCOUNTLZ_UINT;
    pub const BITCOUNTRZ_UINT: Self = Self::BITCOUNTRZ_UINT;
    pub const BYTESWAP_UINT: Self = Self::BYTESWAP_UINT;
    pub const INVOKE_LIBM: Self = Self::INVOKE_LIBM;
    pub const GET_TYPE: Self = Self::GET_TYPE;
    pub const GET_TYPEOF: Self = Self::GET_TYPEOF;
    pub const FINDUPVAL: Self = Self::FINDUPVAL;
    pub const BUFFER_READI8: Self = Self::BUFFER_READI8;
    pub const BUFFER_READU8: Self = Self::BUFFER_READU8;
    pub const BUFFER_WRITEI8: Self = Self::BUFFER_WRITEI8;
    pub const BUFFER_READI16: Self = Self::BUFFER_READI16;
    pub const BUFFER_READU16: Self = Self::BUFFER_READU16;
    pub const BUFFER_WRITEI16: Self = Self::BUFFER_WRITEI16;
    pub const BUFFER_READI32: Self = Self::BUFFER_READI32;
    pub const BUFFER_WRITEI32: Self = Self::BUFFER_WRITEI32;
    pub const BUFFER_READF32: Self = Self::BUFFER_READF32;
    pub const BUFFER_WRITEF32: Self = Self::BUFFER_WRITEF32;
    pub const BUFFER_READF64: Self = Self::BUFFER_READF64;
    pub const BUFFER_WRITEF64: Self = Self::BUFFER_WRITEF64;
    pub const BUFFER_READI64: Self = Self::BUFFER_READI64;
    pub const BUFFER_WRITEI64: Self = Self::BUFFER_WRITEI64;
    pub const JUMP_CMP_PROTOID: Self = Self::JUMP_CMP_PROTOID;
}
