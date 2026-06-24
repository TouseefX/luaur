use crate::type_aliases::lua_state::lua_State;
use crate::type_aliases::luau_fast_function::luau_fast_function;
use core::ffi::{c_double, c_int, c_uint};
use luaur_vm::type_aliases::stk_id::StkId;
use luaur_vm::type_aliases::t_value::TValue;
use luaur_vm::type_aliases::tms::TMS;

#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct NativeContext {
    pub gateEntry: *mut u8,
    pub gateExit: *mut u8,

    pub luaV_lessthan: Option<
        unsafe extern "C" fn(L: *mut lua_State, l: *const TValue, r: *const TValue) -> c_int,
    >,
    pub luaV_lessequal: Option<
        unsafe extern "C" fn(L: *mut lua_State, l: *const TValue, r: *const TValue) -> c_int,
    >,
    pub luaV_equalval: Option<
        unsafe extern "C" fn(L: *mut lua_State, t1: *const TValue, t2: *const TValue) -> c_int,
    >,
    pub luaV_doarithadd: Option<
        unsafe extern "C" fn(L: *mut lua_State, ra: StkId, rb: *const TValue, rc: *const TValue),
    >,
    pub luaV_doarithsub: Option<
        unsafe extern "C" fn(L: *mut lua_State, ra: StkId, rb: *const TValue, rc: *const TValue),
    >,
    pub luaV_doarithmul: Option<
        unsafe extern "C" fn(L: *mut lua_State, ra: StkId, rb: *const TValue, rc: *const TValue),
    >,
    pub luaV_doarithdiv: Option<
        unsafe extern "C" fn(L: *mut lua_State, ra: StkId, rb: *const TValue, rc: *const TValue),
    >,
    pub luaV_doarithidiv: Option<
        unsafe extern "C" fn(L: *mut lua_State, ra: StkId, rb: *const TValue, rc: *const TValue),
    >,
    pub luaV_doarithmod: Option<
        unsafe extern "C" fn(L: *mut lua_State, ra: StkId, rb: *const TValue, rc: *const TValue),
    >,
    pub luaV_doarithpow: Option<
        unsafe extern "C" fn(L: *mut lua_State, ra: StkId, rb: *const TValue, rc: *const TValue),
    >,
    pub luaV_doarithunm: Option<
        unsafe extern "C" fn(L: *mut lua_State, ra: StkId, rb: *const TValue, rc: *const TValue),
    >,
    pub luaV_dolen: Option<unsafe extern "C" fn(L: *mut lua_State, ra: StkId, rb: *const TValue)>,
    pub luaV_gettable: Option<
        unsafe extern "C" fn(L: *mut lua_State, t: *const TValue, key: *mut TValue, val: StkId),
    >,
    pub luaV_settable: Option<
        unsafe extern "C" fn(L: *mut lua_State, t: *const TValue, key: *mut TValue, val: StkId),
    >,
    pub luaV_concat: Option<unsafe extern "C" fn(L: *mut lua_State, total: c_int, last: c_int)>,

    pub luaH_getn: Option<unsafe extern "C" fn(t: *mut core::ffi::c_void) -> c_int>,
    pub luaH_new: Option<
        unsafe extern "C" fn(
            L: *mut lua_State,
            narray: c_int,
            lnhash: c_int,
        ) -> *mut core::ffi::c_void,
    >,
    pub luaH_clone: Option<
        unsafe extern "C" fn(
            L: *mut lua_State,
            tt: *mut core::ffi::c_void,
        ) -> *mut core::ffi::c_void,
    >,
    pub luaH_resizearray:
        Option<unsafe extern "C" fn(L: *mut lua_State, t: *mut core::ffi::c_void, nasize: c_int)>,
    pub luaH_setnum: Option<
        unsafe extern "C" fn(
            L: *mut lua_State,
            t: *mut core::ffi::c_void,
            key: c_int,
        ) -> *mut TValue,
    >,

    pub luaC_barriertable: Option<
        unsafe extern "C" fn(
            L: *mut lua_State,
            t: *mut core::ffi::c_void,
            v: *mut core::ffi::c_void,
        ),
    >,
    pub luaC_barrierf: Option<
        unsafe extern "C" fn(
            L: *mut lua_State,
            o: *mut core::ffi::c_void,
            v: *mut core::ffi::c_void,
        ),
    >,
    pub luaC_barrierback: Option<
        unsafe extern "C" fn(
            L: *mut lua_State,
            o: *mut core::ffi::c_void,
            gclist: *mut *mut core::ffi::c_void,
        ),
    >,
    pub luaC_step: Option<unsafe extern "C" fn(L: *mut lua_State, assist: bool) -> usize>,

    pub luaF_close: Option<unsafe extern "C" fn(L: *mut lua_State, level: StkId)>,
    pub luaF_findupval:
        Option<unsafe extern "C" fn(L: *mut lua_State, level: StkId) -> *mut core::ffi::c_void>,
    pub luaF_newLclosure: Option<
        unsafe extern "C" fn(
            L: *mut lua_State,
            nelems: c_int,
            e: *mut core::ffi::c_void,
            p: *mut core::ffi::c_void,
        ) -> *mut core::ffi::c_void,
    >,

    pub luaT_gettm: Option<
        unsafe extern "C" fn(
            events: *mut core::ffi::c_void,
            event: TMS,
            ename: *mut core::ffi::c_void,
        ) -> *const TValue,
    >,
    pub luaT_objtypenamestr: Option<
        unsafe extern "C" fn(L: *mut lua_State, o: *const TValue) -> *const core::ffi::c_void,
    >,

    pub libm_exp: Option<unsafe extern "C" fn(c_double) -> c_double>,
    pub libm_pow: Option<unsafe extern "C" fn(c_double, c_double) -> c_double>,
    pub libm_fmod: Option<unsafe extern "C" fn(c_double, c_double) -> c_double>,
    pub libm_asin: Option<unsafe extern "C" fn(c_double) -> c_double>,
    pub libm_sin: Option<unsafe extern "C" fn(c_double) -> c_double>,
    pub libm_sinh: Option<unsafe extern "C" fn(c_double) -> c_double>,
    pub libm_acos: Option<unsafe extern "C" fn(c_double) -> c_double>,
    pub libm_cos: Option<unsafe extern "C" fn(c_double) -> c_double>,
    pub libm_cosh: Option<unsafe extern "C" fn(c_double) -> c_double>,
    pub libm_atan: Option<unsafe extern "C" fn(c_double) -> c_double>,
    pub libm_atan2: Option<unsafe extern "C" fn(c_double, c_double) -> c_double>,
    pub libm_tan: Option<unsafe extern "C" fn(c_double) -> c_double>,
    pub libm_tanh: Option<unsafe extern "C" fn(c_double) -> c_double>,
    pub libm_log: Option<unsafe extern "C" fn(c_double) -> c_double>,
    pub libm_log2: Option<unsafe extern "C" fn(c_double) -> c_double>,
    pub libm_log10: Option<unsafe extern "C" fn(c_double) -> c_double>,
    pub libm_ldexp: Option<unsafe extern "C" fn(c_double, c_int) -> c_double>,
    pub libm_round: Option<unsafe extern "C" fn(c_double) -> c_double>,
    pub libm_frexp: Option<unsafe extern "C" fn(c_double, *mut c_int) -> c_double>,
    pub libm_modf: Option<unsafe extern "C" fn(c_double, *mut c_double) -> c_double>,

    pub forgLoopTableIter: Option<
        unsafe extern "C" fn(
            L: *mut lua_State,
            h: *mut core::ffi::c_void,
            index: c_int,
            ra: *mut TValue,
        ) -> bool,
    >,
    pub forgLoopNodeIter: Option<
        unsafe extern "C" fn(
            L: *mut lua_State,
            h: *mut core::ffi::c_void,
            index: c_int,
            ra: *mut TValue,
        ) -> bool,
    >,
    pub forgLoopNonTableFallback:
        Option<unsafe extern "C" fn(L: *mut lua_State, insnA: c_int, aux: c_int) -> c_int>,
    pub forgLoopNonTableFallback_DEPRECATED:
        Option<unsafe extern "C" fn(L: *mut lua_State, insnA: c_int, aux: c_int) -> bool>,
    pub forgPrepXnextFallback:
        Option<unsafe extern "C" fn(L: *mut lua_State, ra: *mut TValue, pc: c_int)>,
    pub callProlog: Option<
        unsafe extern "C" fn(
            L: *mut lua_State,
            ra: *mut TValue,
            argtop: StkId,
            nresults: c_int,
        ) -> *mut core::ffi::c_void,
    >,
    pub callEpilogC: Option<unsafe extern "C" fn(L: *mut lua_State, nresults: c_int, n: c_int)>,
    pub newUserdata: Option<
        unsafe extern "C" fn(L: *mut lua_State, s: usize, tag: c_int) -> *mut core::ffi::c_void,
    >,
    pub getImport:
        Option<unsafe extern "C" fn(L: *mut lua_State, res: StkId, id: c_uint, pc: c_uint)>,

    pub callFallback: Option<
        unsafe extern "C" fn(
            L: *mut lua_State,
            ra: StkId,
            argtop: StkId,
            nresults: c_int,
        ) -> *mut core::ffi::c_void,
    >,

    pub executeGETGLOBAL: Option<
        unsafe extern "C" fn(
            L: *mut lua_State,
            pc: *const u32,
            base: StkId,
            k: *mut TValue,
        ) -> *const u32,
    >,
    pub executeSETGLOBAL: Option<
        unsafe extern "C" fn(
            L: *mut lua_State,
            pc: *const u32,
            base: StkId,
            k: *mut TValue,
        ) -> *const u32,
    >,
    pub executeGETTABLEKS: Option<
        unsafe extern "C" fn(
            L: *mut lua_State,
            pc: *const u32,
            base: StkId,
            k: *mut TValue,
        ) -> *const u32,
    >,
    pub executeSETTABLEKS: Option<
        unsafe extern "C" fn(
            L: *mut lua_State,
            pc: *const u32,
            base: StkId,
            k: *mut TValue,
        ) -> *const u32,
    >,
    pub executeNAMECALL: Option<
        unsafe extern "C" fn(
            L: *mut lua_State,
            pc: *const u32,
            base: StkId,
            k: *mut TValue,
        ) -> *const u32,
    >,
    pub executeSETLIST: Option<
        unsafe extern "C" fn(
            L: *mut lua_State,
            pc: *const u32,
            base: StkId,
            k: *mut TValue,
        ) -> *const u32,
    >,
    pub executeFORGPREP: Option<
        unsafe extern "C" fn(
            L: *mut lua_State,
            pc: *const u32,
            base: StkId,
            k: *mut TValue,
        ) -> *const u32,
    >,
    pub executeGETVARARGSMultRet:
        Option<unsafe extern "C" fn(L: *mut lua_State, pc: *const u32, base: StkId, rai: c_int)>,
    pub executeGETVARARGSConst:
        Option<unsafe extern "C" fn(L: *mut lua_State, base: StkId, rai: c_int, b: c_int)>,
    pub executeDUPCLOSURE: Option<
        unsafe extern "C" fn(
            L: *mut lua_State,
            pc: *const u32,
            base: StkId,
            k: *mut TValue,
        ) -> *const u32,
    >,
    pub executePREPVARARGS: Option<
        unsafe extern "C" fn(
            L: *mut lua_State,
            pc: *const u32,
            base: StkId,
            k: *mut TValue,
        ) -> *const u32,
    >,

    pub luauF_table: [luau_fast_function; 256],
}

impl Default for NativeContext {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
