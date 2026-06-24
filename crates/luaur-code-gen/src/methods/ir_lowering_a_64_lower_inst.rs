//! Node: `cxx:Method:Luau.CodeGen:CodeGen/src/IrLoweringA64.cpp:308:lowerInst`
//! Mechanically transpiled (translation/scripts/lowerinst_rewrite.py) + compiler-driven repair.
#![allow(
    unused_parens,
    unused_braces,
    unused_variables,
    unused_unsafe,
    unused_imports,
    unused_mut
)]
use crate::enums::address_kind_a_64::AddressKindA64;
use crate::enums::condition_a_64::ConditionA64;
use crate::enums::features_a_64::FeaturesA64;
use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_condition::IrCondition;
use crate::enums::ir_const_kind::IrConstKind;
use crate::enums::ir_op_kind::IrOpKind;
use crate::enums::ir_value_kind::IrValueKind;
use crate::enums::kind_a_64::KindA64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::address_a_64::AddressA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::interrupt_handler_ir_lowering_a_64::InterruptHandler;
use crate::records::ir_block::IrBlock;
use crate::records::ir_const::IrConst;
use crate::records::ir_inst::IrInst;
use crate::records::ir_lowering_a_64::IrLoweringA64;
use crate::records::ir_op::IrOp;
use crate::records::label::Label;
use crate::records::native_context::NativeContext;
use crate::records::register_a_64::RegisterA64;
use luaur_common::FFlag;
use luaur_vm::enums::lua_type::lua_Type;
use luaur_vm::macros::blackbit::BLACKBIT;
use luaur_vm::macros::lua_multret::LUA_MULTRET;
use luaur_vm::macros::setnvalue::setnvalue;
use luaur_vm::records::call_info::CallInfo;
use luaur_vm::records::closure::{Closure, LClosure};
use luaur_vm::records::g_cheader::GCheader;
use luaur_vm::records::global_state::global_State;
use luaur_vm::records::lua_t_value::TValue;
use luaur_vm::records::luau_buffer::Buffer;
use luaur_vm::records::proto::Proto;
use luaur_vm::records::t_string::TString;
use luaur_vm::records::udata::Udata;
use luaur_vm::records::up_val::UpVal;
use luaur_vm::type_aliases::instruction::Instruction;
use luaur_vm::type_aliases::lua_node::LuaNode;
use luaur_vm::type_aliases::lua_state::lua_State;
use luaur_vm::type_aliases::lua_table::LuaTable;
use luaur_vm::type_aliases::luau_fast_function::luau_FastFunction;
use luaur_vm::type_aliases::tms::TMS;
use luaur_vm::type_aliases::value::Value;
// local register/address constants (mirrors EmitCommonA64.h)
const K_MAX_IMMEDIATE: u32 = 4095;
const INT_MAX: i32 = i32::MAX;
const kTValueSizeLog2: i32 = 4;
const kLuaNodeSizeLog2: i32 = 5;
const kOffsetOfInstructionC: i32 = 3;
const K_TSTRING_LEN_OFFSET: i32 = 36;
const K_BUFFER_LEN_OFFSET: i32 = 16;
const kOffsetOfTKeyTagNext: i32 = 12;
const kTKeyTagBits: i32 = 4;
const kTKeyTagMask: i32 = (1 << kTKeyTagBits) - 1;
const kInvalidInstIdx: u32 = IrLoweringA64::kInvalidInstIdx;
const Feature_JSCVT: u32 = FeaturesA64::Feature_JSCVT as u32;
const Feature_AdvSIMD: u32 = FeaturesA64::Feature_AdvSIMD as u32;
const LUA_TNIL: u8 = lua_Type::LUA_TNIL as u8;
const LUA_TBOOLEAN: u8 = lua_Type::LUA_TBOOLEAN as u8;
const LUA_TNUMBER: u8 = lua_Type::LUA_TNUMBER as u8;
const LUA_TINTEGER: u8 = lua_Type::LUA_TINTEGER as u8;
const LUA_TVECTOR: u8 = lua_Type::LUA_TVECTOR as u8;
const LUA_TSTRING: u8 = lua_Type::LUA_TSTRING as u8;
const LUA_TTABLE: u8 = lua_Type::LUA_TTABLE as u8;
const LUA_TFUNCTION: u8 = lua_Type::LUA_TFUNCTION as u8;
const LUA_TUSERDATA: u8 = lua_Type::LUA_TUSERDATA as u8;
const LUA_TBUFFER: u8 = lua_Type::LUA_TBUFFER as u8;
const LUA_TUPVAL: u8 = lua_Type::LUA_TUPVAL as u8;
const K_TVALUE_VALUE_GC_OFFSET: i32 =
    (core::mem::offset_of!(TValue, value) + core::mem::offset_of!(Value, gc)) as i32;
const K_TVALUE_VALUE_N_OFFSET: i32 =
    (core::mem::offset_of!(TValue, value) + core::mem::offset_of!(Value, n)) as i32;
const K_TVALUE_VALUE_L_OFFSET: i32 =
    (core::mem::offset_of!(TValue, value) + core::mem::offset_of!(Value, l)) as i32;
const K_TVALUE_VALUE_P_OFFSET: i32 =
    (core::mem::offset_of!(TValue, value) + core::mem::offset_of!(Value, p)) as i32;
const K_CLOSURE_L_P_OFFSET: i32 =
    (core::mem::offset_of!(Closure, inner) + core::mem::offset_of!(LClosure, p)) as i32;
const K_CLOSURE_L_UPREFS_OFFSET: i32 =
    (core::mem::offset_of!(Closure, inner) + core::mem::offset_of!(LClosure, uprefs)) as i32;

const fn reg(kind: KindA64, index: u8) -> RegisterA64 {
    RegisterA64 {
        bits: kind as u8 | (index << RegisterA64::INDEX_SHIFT),
    }
}

const x0: RegisterA64 = reg(KindA64::x, 0);
const x1: RegisterA64 = reg(KindA64::x, 1);
const x2: RegisterA64 = reg(KindA64::x, 2);
const x3: RegisterA64 = reg(KindA64::x, 3);
const x4: RegisterA64 = reg(KindA64::x, 4);
const x5: RegisterA64 = reg(KindA64::x, 5);
const x6: RegisterA64 = reg(KindA64::x, 6);
const x9: RegisterA64 = reg(KindA64::x, 9);
const x15: RegisterA64 = reg(KindA64::x, 15);
const w0: RegisterA64 = reg(KindA64::w, 0);
const w1: RegisterA64 = reg(KindA64::w, 1);
const w2: RegisterA64 = reg(KindA64::w, 2);
const w3: RegisterA64 = reg(KindA64::w, 3);
const w4: RegisterA64 = reg(KindA64::w, 4);
const w5: RegisterA64 = reg(KindA64::w, 5);
const d0: RegisterA64 = reg(KindA64::d, 0);
const d1: RegisterA64 = reg(KindA64::d, 1);
const q0: RegisterA64 = reg(KindA64::q, 0);
const wzr: RegisterA64 = reg(KindA64::w, 31);
const xzr: RegisterA64 = reg(KindA64::x, 31);
const sp: RegisterA64 = reg(KindA64::x, 31);
const noreg: RegisterA64 = RegisterA64::noreg;
const rState: RegisterA64 = reg(KindA64::x, 19);
const rNativeContext: RegisterA64 = reg(KindA64::x, 20);
const rGlobalState: RegisterA64 = reg(KindA64::x, 21);
const rConstants: RegisterA64 = reg(KindA64::x, 22);
const rClosure: RegisterA64 = reg(KindA64::x, 23);
const rCode: RegisterA64 = reg(KindA64::x, 24);
const rBase: RegisterA64 = reg(KindA64::x, 25);
const fn r_state() -> RegisterA64 {
    rState
}
const fn r_base() -> RegisterA64 {
    rBase
}
const fn r_constants() -> RegisterA64 {
    rConstants
}
const fn r_native_context() -> RegisterA64 {
    rNativeContext
}
use crate::functions::call_fallback::call_fallback;
use crate::functions::cast_reg::cast_reg;
use crate::functions::condition_op::condition_op;
use crate::functions::emit_abort::emit_abort;
use crate::functions::emit_add_offset::emit_add_offset as emit_add_offset_impl;
use crate::functions::emit_builtin_ir_lowering_a_64::emit_builtin_assembly_builder_a_64_ir_function_ir_reg_alloc_a_64_i32_i32_i32_i32 as emit_builtin;
use crate::functions::emit_fallback_ir_lowering_a_64::emit_fallback_assembly_builder_a_64_i32_i32 as emit_fallback_impl;
use crate::functions::emit_update_base_emit_common_a_64::emit_update_base;
use crate::functions::get_cmd_value_kind::get_cmd_value_kind;
use crate::functions::get_condition_fp::get_condition_fp;
use crate::functions::get_condition_int_64::get_condition_int_64;
use crate::functions::get_condition_int_ir_lowering_a_64::get_condition_int;
use crate::functions::get_double_bits::get_double_bits;
use crate::functions::get_float_bits::get_float_bits;
use crate::functions::get_inverse_condition_condition_a_64::get_inverse_condition;
use crate::functions::get_native_context_offset::get_native_context_offset;
use crate::functions::get_negated_condition_ir_utils::get_negated_condition_ir_condition as get_negated_condition;
use crate::functions::get_op_ir_data::get_op_mut;
use crate::functions::is_gco::is_gco;
use crate::functions::produces_dirty_high_register_bits::produces_dirty_high_register_bits;
use crate::functions::vm_const_op::vm_const_op;
use crate::functions::vm_reg_op::vm_reg_op;
use crate::functions::vm_upvalue_op::vm_upvalue_op as vm_upvalue_op_raw;

trait MemArg {
    fn address(self, base: RegisterA64) -> AddressA64;
}

impl MemArg for RegisterA64 {
    fn address(self, base: RegisterA64) -> AddressA64 {
        AddressA64::address_a_64_register_a_64_register_a_64(base, self)
    }
}

impl MemArg for i32 {
    fn address(self, base: RegisterA64) -> AddressA64 {
        AddressA64::address_a_64_register_a_64_i32_address_kind_a_64(
            base,
            self,
            AddressKindA64::imm,
        )
    }
}

impl MemArg for u32 {
    fn address(self, base: RegisterA64) -> AddressA64 {
        (self as i32).address(base)
    }
}

impl MemArg for usize {
    fn address(self, base: RegisterA64) -> AddressA64 {
        (self as i32).address(base)
    }
}

fn mem<T: MemArg>(base: RegisterA64, data: T) -> AddressA64 {
    data.address(base)
}

fn mem_kind(base: RegisterA64, data: i32, kind: AddressKindA64) -> AddressA64 {
    AddressA64::address_a_64_register_a_64_i32_address_kind_a_64(base, data, kind)
}

trait OffsetArg {
    fn to_usize(self) -> usize;
}

impl OffsetArg for usize {
    fn to_usize(self) -> usize {
        self
    }
}

impl OffsetArg for u32 {
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl OffsetArg for i32 {
    fn to_usize(self) -> usize {
        self as usize
    }
}

fn emit_add_offset<T: OffsetArg>(
    build: &mut AssemblyBuilderA64,
    dst: RegisterA64,
    src: RegisterA64,
    offset: T,
) {
    emit_add_offset_impl(build, dst, src, offset.to_usize());
}

trait I32Arg {
    fn to_i32(self) -> i32;
}

impl I32Arg for i32 {
    fn to_i32(self) -> i32 {
        self
    }
}

impl I32Arg for u32 {
    fn to_i32(self) -> i32 {
        self as i32
    }
}

fn emit_fallback<T: I32Arg>(build: &mut AssemblyBuilderA64, offset: i32, pcpos: T) {
    emit_fallback_impl(build, offset, pcpos.to_i32());
}

fn vm_upvalue_op(op: IrOp) -> i32 {
    vm_upvalue_op_raw(op) as i32
}

trait MovArg {
    fn mov_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64);
}

impl MovArg for RegisterA64 {
    fn mov_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64) {
        build.mov_register_a_64_register_a_64(dst, self);
    }
}

impl MovArg for i32 {
    fn mov_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64) {
        build.mov_register_a_64_i32(dst, self);
    }
}

impl MovArg for u32 {
    fn mov_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64) {
        build.mov_register_a_64_i32(dst, self as i32);
    }
}

impl MovArg for u16 {
    fn mov_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64) {
        build.mov_register_a_64_i32(dst, self as i32);
    }
}

impl MovArg for u8 {
    fn mov_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64) {
        build.mov_register_a_64_i32(dst, self as i32);
    }
}

trait AddSubArg {
    fn add_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64);
    fn sub_from(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64);
}

impl AddSubArg for RegisterA64 {
    fn add_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
        build.add_register_a_64_register_a_64_register_a_64_i32(dst, src1, self, 0);
    }
    fn sub_from(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
        build.sub_register_a_64_register_a_64_register_a_64_i32(dst, src1, self, 0);
    }
}

impl AddSubArg for u16 {
    fn add_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
        build.add_register_a_64_register_a_64_u16(dst, src1, self);
    }
    fn sub_from(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
        build.sub_register_a_64_register_a_64_u16(dst, src1, self);
    }
}

impl AddSubArg for i32 {
    fn add_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
        build.add_register_a_64_register_a_64_u16(dst, src1, self as u16);
    }
    fn sub_from(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
        build.sub_register_a_64_register_a_64_u16(dst, src1, self as u16);
    }
}

impl AddSubArg for u32 {
    fn add_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
        build.add_register_a_64_register_a_64_u16(dst, src1, self as u16);
    }
    fn sub_from(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
        build.sub_register_a_64_register_a_64_u16(dst, src1, self as u16);
    }
}

trait LogicArg {
    fn and_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64);
    fn orr_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64);
    fn eor_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64);
}

impl LogicArg for RegisterA64 {
    fn and_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
        build.and_register_a_64_register_a_64_register_a_64_i32(dst, src1, self, 0);
    }
    fn orr_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
        build.orr_register_a_64_register_a_64_register_a_64_i32(dst, src1, self, 0);
    }
    fn eor_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
        build.eor_register_a_64_register_a_64_register_a_64_i32(dst, src1, self, 0);
    }
}

impl LogicArg for u32 {
    fn and_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
        build.and_register_a_64_register_a_64_u32(dst, src1, self);
    }
    fn orr_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
        build.orr_register_a_64_register_a_64_u32(dst, src1, self);
    }
    fn eor_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
        build.eor_register_a_64_register_a_64_u32(dst, src1, self);
    }
}

impl LogicArg for i32 {
    fn and_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
        (self as u32).and_to(build, dst, src1);
    }
    fn orr_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
        (self as u32).orr_to(build, dst, src1);
    }
    fn eor_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
        (self as u32).eor_to(build, dst, src1);
    }
}

trait ShiftArg {
    fn lsl_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64);
    fn lsr_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64);
    fn asr_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64);
    fn ror_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64);
}

impl ShiftArg for RegisterA64 {
    fn lsl_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
        build.lsl_register_a_64_register_a_64_register_a_64(dst, src1, self);
    }
    fn lsr_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
        build.lsr_register_a_64_register_a_64_register_a_64(dst, src1, self);
    }
    fn asr_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
        build.asr_register_a_64_register_a_64_register_a_64(dst, src1, self);
    }
    fn ror_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
        build.ror_register_a_64_register_a_64_register_a_64(dst, src1, self);
    }
}

impl ShiftArg for u8 {
    fn lsl_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
        build.lsl_register_a_64_register_a_64_u8(dst, src1, self);
    }
    fn lsr_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
        build.lsr_register_a_64_register_a_64_u8(dst, src1, self);
    }
    fn asr_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
        build.asr_register_a_64_register_a_64_u8(dst, src1, self);
    }
    fn ror_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
        build.ror_register_a_64_register_a_64_u8(dst, src1, self);
    }
}

impl ShiftArg for i32 {
    fn lsl_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
        (self as u8).lsl_to(build, dst, src1);
    }
    fn lsr_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
        (self as u8).lsr_to(build, dst, src1);
    }
    fn asr_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
        (self as u8).asr_to(build, dst, src1);
    }
    fn ror_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64, src1: RegisterA64) {
        (self as u8).ror_to(build, dst, src1);
    }
}

trait CmpArg {
    fn cmp_with(self, build: &mut AssemblyBuilderA64, src1: RegisterA64);
}

impl CmpArg for RegisterA64 {
    fn cmp_with(self, build: &mut AssemblyBuilderA64, src1: RegisterA64) {
        build.cmp_register_a_64_register_a_64(src1, self);
    }
}

impl CmpArg for u16 {
    fn cmp_with(self, build: &mut AssemblyBuilderA64, src1: RegisterA64) {
        build.cmp_register_a_64_u16(src1, self);
    }
}

impl CmpArg for i32 {
    fn cmp_with(self, build: &mut AssemblyBuilderA64, src1: RegisterA64) {
        build.cmp_register_a_64_u16(src1, self as u16);
    }
}

impl CmpArg for u32 {
    fn cmp_with(self, build: &mut AssemblyBuilderA64, src1: RegisterA64) {
        build.cmp_register_a_64_u16(src1, self as u16);
    }
}

trait FmovArg {
    fn fmov_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64);
}

impl FmovArg for RegisterA64 {
    fn fmov_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64) {
        build.fmov_register_a_64_register_a_64(dst, self);
    }
}

impl FmovArg for f32 {
    fn fmov_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64) {
        build.fmov_register_a_64_f32(dst, self);
    }
}

impl FmovArg for f64 {
    fn fmov_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64) {
        build.fmov_register_a_64_f64(dst, self);
    }
}

trait AdrArg {
    fn adr_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64);
}

impl AdrArg for u64 {
    fn adr_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64) {
        build.adr_register_a_64_u64(dst, self);
    }
}

impl AdrArg for f32 {
    fn adr_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64) {
        build.adr_register_a_64_f32(dst, self);
    }
}

impl AdrArg for f64 {
    fn adr_to(self, build: &mut AssemblyBuilderA64, dst: RegisterA64) {
        build.adr_register_a_64_f64(dst, self);
    }
}

trait CcmnArg {
    fn ccmn_with(
        self,
        build: &mut AssemblyBuilderA64,
        src1: RegisterA64,
        cond: ConditionA64,
        nzcv: u8,
    );
}

impl CcmnArg for RegisterA64 {
    fn ccmn_with(
        self,
        build: &mut AssemblyBuilderA64,
        src1: RegisterA64,
        cond: ConditionA64,
        nzcv: u8,
    ) {
        build.ccmn_register_a_64_register_a_64_condition_a_64_u8(src1, self, cond, nzcv);
    }
}

impl CcmnArg for u8 {
    fn ccmn_with(
        self,
        build: &mut AssemblyBuilderA64,
        src1: RegisterA64,
        cond: ConditionA64,
        nzcv: u8,
    ) {
        build.ccmn_register_a_64_u8_condition_a_64_u8(src1, self, cond, nzcv);
    }
}

impl CcmnArg for i32 {
    fn ccmn_with(
        self,
        build: &mut AssemblyBuilderA64,
        src1: RegisterA64,
        cond: ConditionA64,
        nzcv: u8,
    ) {
        (self as u8).ccmn_with(build, src1, cond, nzcv);
    }
}

trait TstArg {
    fn tst_with(self, build: &mut AssemblyBuilderA64, src1: RegisterA64);
}

impl TstArg for RegisterA64 {
    fn tst_with(self, build: &mut AssemblyBuilderA64, src1: RegisterA64) {
        build.tst_register_a_64_register_a_64_i32(src1, self, 0);
    }
}

impl TstArg for u32 {
    fn tst_with(self, build: &mut AssemblyBuilderA64, src1: RegisterA64) {
        build.tst_register_a_64_u32(src1, self);
    }
}

impl TstArg for i32 {
    fn tst_with(self, build: &mut AssemblyBuilderA64, src1: RegisterA64) {
        (self as u32).tst_with(build, src1);
    }
}

trait LowerInstBuilderExt {
    fn b(&mut self, label: &mut Label);
    fn mov<T: MovArg>(&mut self, dst: RegisterA64, src: T);
    fn add<T: AddSubArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T);
    fn sub<T: AddSubArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T);
    fn and_<T: LogicArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T);
    fn orr<T: LogicArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T);
    fn eor<T: LogicArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T);
    fn lsl<T: ShiftArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T);
    fn lsr<T: ShiftArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T);
    fn asr<T: ShiftArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T);
    fn ror<T: ShiftArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T);
    fn cmp<T: CmpArg>(&mut self, src1: RegisterA64, src2: T);
    fn fmov<T: FmovArg>(&mut self, dst: RegisterA64, src: T);
    fn adr<T: AdrArg>(&mut self, dst: RegisterA64, value: T);
    fn ccmn<T: CcmnArg>(&mut self, src1: RegisterA64, src2: T, cond: ConditionA64, nzcv: u8);
    fn tst<T: TstArg>(&mut self, src1: RegisterA64, src2: T);
    fn sbfx(&mut self, dst: RegisterA64, src: RegisterA64, offset: u8, size: u8);
}

impl LowerInstBuilderExt for AssemblyBuilderA64 {
    fn b(&mut self, label: &mut Label) {
        self.b_label(label);
    }
    fn mov<T: MovArg>(&mut self, dst: RegisterA64, src: T) {
        src.mov_to(self, dst);
    }
    fn add<T: AddSubArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T) {
        src2.add_to(self, dst, src1);
    }
    fn sub<T: AddSubArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T) {
        src2.sub_from(self, dst, src1);
    }
    fn and_<T: LogicArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T) {
        src2.and_to(self, dst, src1);
    }
    fn orr<T: LogicArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T) {
        src2.orr_to(self, dst, src1);
    }
    fn eor<T: LogicArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T) {
        src2.eor_to(self, dst, src1);
    }
    fn lsl<T: ShiftArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T) {
        src2.lsl_to(self, dst, src1);
    }
    fn lsr<T: ShiftArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T) {
        src2.lsr_to(self, dst, src1);
    }
    fn asr<T: ShiftArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T) {
        src2.asr_to(self, dst, src1);
    }
    fn ror<T: ShiftArg>(&mut self, dst: RegisterA64, src1: RegisterA64, src2: T) {
        src2.ror_to(self, dst, src1);
    }
    fn cmp<T: CmpArg>(&mut self, src1: RegisterA64, src2: T) {
        src2.cmp_with(self, src1);
    }
    fn fmov<T: FmovArg>(&mut self, dst: RegisterA64, src: T) {
        src.fmov_to(self, dst);
    }
    fn adr<T: AdrArg>(&mut self, dst: RegisterA64, value: T) {
        value.adr_to(self, dst);
    }
    fn ccmn<T: CcmnArg>(&mut self, src1: RegisterA64, src2: T, cond: ConditionA64, nzcv: u8) {
        src2.ccmn_with(self, src1, cond, nzcv);
    }
    fn tst<T: TstArg>(&mut self, src1: RegisterA64, src2: T) {
        src2.tst_with(self, src1);
    }
    fn sbfx(&mut self, dst: RegisterA64, src: RegisterA64, offset: u8, size: u8) {
        self.sbfx_register_a_64_register_a_64_u8_u8(dst, src, offset, size);
    }
}

fn has_op_b(inst: &IrInst) -> bool {
    1 < inst.ops.size() as usize && inst.ops[1].kind() != IrOpKind::None
}

fn has_op_c(inst: &IrInst) -> bool {
    2 < inst.ops.size() as usize && inst.ops[2].kind() != IrOpKind::None
}

fn has_op_d(inst: &IrInst) -> bool {
    3 < inst.ops.size() as usize && inst.ops[3].kind() != IrOpKind::None
}

fn has_op_e(inst: &IrInst) -> bool {
    4 < inst.ops.size() as usize && inst.ops[4].kind() != IrOpKind::None
}

fn get_condition_int64(cond: IrCondition) -> ConditionA64 {
    get_condition_int_64(cond)
}

fn get_condition_f_p(cond: IrCondition) -> ConditionA64 {
    get_condition_fp(cond)
}

impl IrLoweringA64 {
    fn int_op(&self, op: IrOp) -> i32 {
        unsafe { (*self.function).int_op(op) }
    }

    fn uint_op(&self, op: IrOp) -> u32 {
        unsafe { (*self.function).uint_op(op) }
    }

    fn int_64_op(&mut self, op: IrOp) -> i64 {
        unsafe { (*self.function).int64_op(op) }
    }

    fn double_op(&self, op: IrOp) -> f64 {
        unsafe { (*self.function).double_op(op) }
    }

    fn tag_op(&self, op: IrOp) -> u8 {
        unsafe { (*self.function).tag_op(op) }
    }

    fn const_op(&self, op: IrOp) -> IrConst {
        unsafe { (*self.function).const_op(op) }
    }

    fn import_op(&self, op: IrOp) -> u32 {
        unsafe { (*self.function).import_op(op) }
    }

    fn reg_op(&mut self, op: IrOp) -> RegisterA64 {
        self.ir_lowering_a_64_reg_op(op)
    }

    fn label_op(&mut self, op: IrOp) -> &mut Label {
        self.ir_lowering_a_64_label_op(op)
    }

    fn block_op(&self, op: IrOp) -> &mut IrBlock {
        self.ir_lowering_a_64_block_op(op)
    }

    fn temp_addr(&mut self, op: IrOp, offset: i32) -> AddressA64 {
        self.ir_lowering_a_64_temp_addr(op, offset, RegisterA64::noreg)
    }

    fn temp_double(&mut self, op: IrOp) -> RegisterA64 {
        self.ir_lowering_a_64_temp_double(op)
    }

    fn temp_int(&mut self, op: IrOp) -> RegisterA64 {
        self.ir_lowering_a_64_temp_int(op)
    }

    fn temp_uint(&mut self, op: IrOp) -> RegisterA64 {
        self.ir_lowering_a_64_temp_uint(op)
    }

    fn temp_int64(&mut self, op: IrOp) -> RegisterA64 {
        self.ir_lowering_a_64_temp_int_64(op)
    }

    fn temp_addr_buffer(&mut self, buffer_op: IrOp, index_op: IrOp, tag: u8) -> AddressA64 {
        self.ir_lowering_a_64_temp_addr_buffer(buffer_op, index_op, tag)
    }

    fn temp_float(&mut self, op: IrOp) -> RegisterA64 {
        if op.kind() == IrOpKind::Inst {
            self.reg_op(op)
        } else if op.kind() == IrOpKind::Constant {
            let val = self.double_op(op) as f32;

            if unsafe { (*self.build).is_fmov_supported_fp_32(val) } {
                let temp = self.regs.alloc_temp(KindA64::s);
                unsafe { (*self.build).fmov_register_a_64_f32(temp, val) };
                temp
            } else {
                let temp = self.regs.alloc_temp(KindA64::s);
                let vali = get_float_bits(val);

                if (vali & 0xffff) == 0 {
                    let temp2 = self.regs.alloc_temp(KindA64::w);
                    unsafe {
                        (*self.build).movz(temp2, (vali >> 16) as u16, 16);
                        (*self.build).fmov_register_a_64_register_a_64(temp, temp2);
                    }
                } else {
                    let temp2 = self.regs.alloc_temp(KindA64::x);
                    unsafe {
                        (*self.build).adr_register_a_64_f32(temp2, val);
                        (*self.build).ldr(temp, mem(temp2, 0));
                    }
                }

                temp
            }
        } else {
            CODEGEN_ASSERT!(false, "Unsupported instruction form");
            RegisterA64::noreg
        }
    }

    fn check_safe_env(&mut self, target: IrOp, index: u32, next: &IrBlock) {
        self.ir_lowering_a_64_check_safe_env(target, index, next)
    }

    fn is_fallthrough_block(&self, target: &IrBlock, next: &IrBlock) -> bool {
        self.ir_lowering_a_64_is_fallthrough_block(target, next)
    }

    fn jump_or_fallthrough(&mut self, target: &mut IrBlock, next: &IrBlock) {
        self.ir_lowering_a_64_jump_or_fallthrough(target, next)
    }

    fn jump_or_fallthrough_op(&mut self, op: IrOp, next: &IrBlock) {
        let target = self.block_op(op) as *mut IrBlock;
        unsafe { self.jump_or_fallthrough(&mut *target, next) }
    }

    fn check_object_barrier_conditions(
        &mut self,
        object: RegisterA64,
        temp: RegisterA64,
        ra: RegisterA64,
        ra_op: IrOp,
        ratag: i32,
        skip: &mut Label,
    ) {
        self.ir_lowering_a_64_check_object_barrier_conditions(object, temp, ra, ra_op, ratag, skip)
    }

    pub fn ir_lowering_a_64_lower_inst(&mut self, inst: &mut IrInst, index: u32, next: &IrBlock) {
        unsafe {
            self.regs.curr_inst_idx = index;

            self.value_tracker.before_inst_lowering(inst);
            match inst.cmd {
                IrCmd::LOAD_TAG => {
                    inst.reg_a64 = self.regs.alloc_reg(KindA64::w, index);
                    let mut addr = self.temp_addr(
                        (*get_op_mut(inst, 0)),
                        (core::mem::offset_of!(TValue, tt) as i32),
                    );
                    (*self.build).ldr(inst.reg_a64, addr);
                }
                IrCmd::LOAD_POINTER => {
                    inst.reg_a64 = self.regs.alloc_reg(KindA64::x, index);
                    let mut addr =
                        self.temp_addr((*get_op_mut(inst, 0)), (K_TVALUE_VALUE_GC_OFFSET as i32));
                    (*self.build).ldr(inst.reg_a64, addr);
                }
                IrCmd::LOAD_DOUBLE => {
                    inst.reg_a64 = self.regs.alloc_reg(KindA64::d, index);
                    let mut addr =
                        self.temp_addr((*get_op_mut(inst, 0)), (K_TVALUE_VALUE_N_OFFSET as i32));
                    (*self.build).ldr(inst.reg_a64, addr);
                }
                IrCmd::LOAD_INT => {
                    inst.reg_a64 = self.regs.alloc_reg(KindA64::w, index);
                    let mut addr = self.temp_addr(
                        (*get_op_mut(inst, 0)),
                        (core::mem::offset_of!(TValue, value) as i32),
                    );
                    (*self.build).ldr(inst.reg_a64, addr);
                }
                IrCmd::LOAD_INT64 => {
                    inst.reg_a64 = self.regs.alloc_reg(KindA64::x, index);
                    let mut addr =
                        self.temp_addr((*get_op_mut(inst, 0)), (K_TVALUE_VALUE_L_OFFSET as i32));
                    (*self.build).ldr(inst.reg_a64, addr);
                }
                IrCmd::LOAD_FLOAT => {
                    inst.reg_a64 = self.regs.alloc_reg(KindA64::s, index);
                    let mut addr =
                        self.temp_addr((*get_op_mut(inst, 0)), self.int_op((*get_op_mut(inst, 1))));

                    (*self.build).ldr(inst.reg_a64, addr);
                }
                IrCmd::LOAD_TVALUE => {
                    inst.reg_a64 = self.regs.alloc_reg(KindA64::q, index);

                    let mut addrOffset = if has_op_b(inst) {
                        self.int_op((*get_op_mut(inst, 1)))
                    } else {
                        0
                    };
                    let mut addr = self.temp_addr((*get_op_mut(inst, 0)), addrOffset);
                    (*self.build).ldr(inst.reg_a64, addr);
                }
                IrCmd::LOAD_ENV => {
                    inst.reg_a64 = self.regs.alloc_reg(KindA64::x, index);
                    (*self.build).ldr(
                        inst.reg_a64,
                        mem(rClosure, (core::mem::offset_of!(Closure, env) as i32)),
                    );
                }
                IrCmd::GET_ARR_ADDR => {
                    {
                        inst.reg_a64 =
                            self.regs
                                .alloc_reuse(KindA64::x, index, &[(*get_op_mut(inst, 0))]);
                        (*self.build).ldr(
                            inst.reg_a64,
                            mem(
                                self.reg_op((*get_op_mut(inst, 0))),
                                (core::mem::offset_of!(LuaTable, array) as i32),
                            ),
                        );

                        if (*get_op_mut(inst, 1)).kind() == IrOpKind::Inst {
                            (*self.build).add_register_a_64_register_a_64_register_a_64_i32(
                                inst.reg_a64,
                                inst.reg_a64,
                                self.reg_op((*get_op_mut(inst, 1))),
                                kTValueSizeLog2,
                            ); // implicit uxtw
                        } else if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant {
                            if self.int_op((*get_op_mut(inst, 1))) == 0 {
                                // no offset required
                            } else if self.int_op((*get_op_mut(inst, 1)))
                                * (core::mem::size_of::<TValue>() as i32)
                                <= K_MAX_IMMEDIATE as i32
                            {
                                (*self.build).add(
                                    inst.reg_a64,
                                    inst.reg_a64,
                                    ((self.int_op((*get_op_mut(inst, 1)))
                                        * (core::mem::size_of::<TValue>() as i32))
                                        as u16),
                                );
                            } else {
                                let mut temp = self.regs.alloc_temp(KindA64::x);
                                (*self.build).mov(
                                    temp,
                                    self.int_op((*get_op_mut(inst, 1)))
                                        * (core::mem::size_of::<TValue>() as i32),
                                );
                                (*self.build).add(inst.reg_a64, inst.reg_a64, temp);
                            }
                        } else {
                            CODEGEN_ASSERT!(false, "Unsupported instruction form");
                        }
                    }
                }
                IrCmd::GET_SLOT_NODE_ADDR => {
                    {
                        inst.reg_a64 =
                            self.regs
                                .alloc_reuse(KindA64::x, index, &[(*get_op_mut(inst, 0))]);
                        let mut temp1 = self.regs.alloc_temp(KindA64::x);
                        let mut temp1w = cast_reg(KindA64::w, temp1);
                        let mut temp2 = self.regs.alloc_temp(KindA64::w);
                        let mut temp2x = cast_reg(KindA64::x, temp2);

                        // note: since the stride of the load is the same as the destination register size, we can range check the array index, not the byte offset
                        if self.uint_op((*get_op_mut(inst, 1))) <= AddressA64::kMaxOffset as u32 {
                            (*self.build).ldr(
                                temp1w,
                                mem(
                                    rCode,
                                    (self.uint_op((*get_op_mut(inst, 1))) as i32)
                                        * (core::mem::size_of::<Instruction>() as i32),
                                ),
                            );
                        } else {
                            (*self.build).mov(
                                temp1,
                                (self.uint_op((*get_op_mut(inst, 1))) as i32)
                                    * (core::mem::size_of::<Instruction>() as i32),
                            );
                            (*self.build).ldr(temp1w, mem(rCode, temp1));
                        }

                        // C field can be shifted as long as it's at the most significant byte of the instruction word
                        CODEGEN_ASSERT!(kOffsetOfInstructionC == 3);
                        (*self.build).ldrb(
                            temp2,
                            mem(
                                self.reg_op((*get_op_mut(inst, 0))),
                                (core::mem::offset_of!(LuaTable, nodemask8) as i32),
                            ),
                        );
                        (*self.build).and_register_a_64_register_a_64_register_a_64_i32(
                            temp2, temp2, temp1w, -24,
                        );

                        // note: this may clobber (*get_op_mut(inst, 0)), so it's important that we don't use it after this
                        (*self.build).ldr(
                            inst.reg_a64,
                            mem(
                                self.reg_op((*get_op_mut(inst, 0))),
                                (core::mem::offset_of!(LuaTable, node) as i32),
                            ),
                        );
                        (*self.build).add_register_a_64_register_a_64_register_a_64_i32(
                            inst.reg_a64,
                            inst.reg_a64,
                            temp2x,
                            kLuaNodeSizeLog2,
                        ); // "zero extend" temp2 to get a larger shift (top 32 bits are zero)
                    }
                }
                IrCmd::GET_HASH_NODE_ADDR => {
                    {
                        inst.reg_a64 =
                            self.regs
                                .alloc_reuse(KindA64::x, index, &[(*get_op_mut(inst, 0))]);
                        let mut temp1 = self.regs.alloc_temp(KindA64::w);
                        let mut temp2 = self.regs.alloc_temp(KindA64::w);
                        let mut temp2x = cast_reg(KindA64::x, temp2);

                        // hash & ((1 << lsizenode) - 1) == hash & !(-1 << lsizenode)
                        (*self.build).mov(temp1, -1);
                        (*self.build).ldrb(
                            temp2,
                            mem(
                                self.reg_op((*get_op_mut(inst, 0))),
                                (core::mem::offset_of!(LuaTable, lsizenode) as i32),
                            ),
                        );
                        (*self.build).lsl(temp1, temp1, temp2);
                        (*self.build).mov(temp2, self.uint_op((*get_op_mut(inst, 1))));
                        (*self.build).bic(temp2, temp2, temp1, 0);

                        // note: this may clobber (*get_op_mut(inst, 0)), so it's important that we don't use it after this
                        (*self.build).ldr(
                            inst.reg_a64,
                            mem(
                                self.reg_op((*get_op_mut(inst, 0))),
                                (core::mem::offset_of!(LuaTable, node) as i32),
                            ),
                        );
                        (*self.build).add_register_a_64_register_a_64_register_a_64_i32(
                            inst.reg_a64,
                            inst.reg_a64,
                            temp2x,
                            kLuaNodeSizeLog2,
                        ); // "zero extend" temp2 to get a larger shift (top 32 bits are zero)
                    }
                }
                IrCmd::GET_CLOSURE_UPVAL_ADDR => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::x, index, &[(*get_op_mut(inst, 0))]);
                    let mut cl = if (*get_op_mut(inst, 0)).kind() == IrOpKind::Undef {
                        rClosure
                    } else {
                        self.reg_op((*get_op_mut(inst, 0)))
                    };

                    (*self.build).add(
                        inst.reg_a64,
                        cl,
                        (((K_CLOSURE_L_UPREFS_OFFSET as i32)
                            + (core::mem::size_of::<TValue>() as i32)
                                * vm_upvalue_op((*get_op_mut(inst, 1))))
                            as u16),
                    );
                }
                IrCmd::STORE_TAG => {
                    let mut addr = self.temp_addr(
                        (*get_op_mut(inst, 0)),
                        (core::mem::offset_of!(TValue, tt) as i32),
                    );
                    if self.tag_op((*get_op_mut(inst, 1))) == 0 {
                        (*self.build).str(wzr, addr);
                    } else {
                        let mut temp = self.regs.alloc_temp(KindA64::w);
                        (*self.build).mov(temp, self.tag_op((*get_op_mut(inst, 1))));
                        (*self.build).str(temp, addr);
                    }
                }
                IrCmd::STORE_POINTER => {
                    let mut addr = self.temp_addr(
                        (*get_op_mut(inst, 0)),
                        (core::mem::offset_of!(TValue, value) as i32),
                    );
                    if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant {
                        CODEGEN_ASSERT!(self.int_op((*get_op_mut(inst, 1))) == 0);
                        (*self.build).str(xzr, addr);
                    } else {
                        (*self.build).str(self.reg_op((*get_op_mut(inst, 1))), addr);
                    }
                }
                IrCmd::STORE_EXTRA => {
                    let mut addr = self.temp_addr(
                        (*get_op_mut(inst, 0)),
                        (core::mem::offset_of!(TValue, extra) as i32),
                    );
                    if self.int_op((*get_op_mut(inst, 1))) == 0 {
                        (*self.build).str(wzr, addr);
                    } else {
                        let mut temp = self.regs.alloc_temp(KindA64::w);
                        (*self.build).mov(temp, self.int_op((*get_op_mut(inst, 1))));
                        (*self.build).str(temp, addr);
                    }
                }
                IrCmd::STORE_DOUBLE => {
                    let mut addr = self.temp_addr(
                        (*get_op_mut(inst, 0)),
                        (core::mem::offset_of!(TValue, value) as i32),
                    );
                    if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
                        && get_double_bits(self.double_op((*get_op_mut(inst, 1)))) == 0
                    {
                        (*self.build).str(xzr, addr);
                    } else {
                        let mut temp = self.temp_double((*get_op_mut(inst, 1)));
                        (*self.build).str(temp, addr);
                    }
                }
                IrCmd::STORE_INT => {
                    let mut addr = self.temp_addr(
                        (*get_op_mut(inst, 0)),
                        (core::mem::offset_of!(TValue, value) as i32),
                    );
                    if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
                        && self.int_op((*get_op_mut(inst, 1))) == 0
                    {
                        (*self.build).str(wzr, addr);
                    } else {
                        let mut temp = self.temp_int((*get_op_mut(inst, 1)));
                        (*self.build).str(temp, addr);
                    }
                }
                IrCmd::STORE_INT64 => {
                    let mut addr = self.temp_addr(
                        (*get_op_mut(inst, 0)),
                        (core::mem::offset_of!(TValue, value) as i32),
                    );
                    if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
                        && self.int_64_op((*get_op_mut(inst, 1))) == 0
                    {
                        (*self.build).str(xzr, addr);
                    } else {
                        let mut temp = self.temp_int64((*get_op_mut(inst, 1)));
                        (*self.build).str(temp, addr);
                    }
                }
                IrCmd::STORE_VECTOR => {
                    let mut temp1 = self.temp_float((*get_op_mut(inst, 1)));
                    let mut temp2 = self.temp_float((*get_op_mut(inst, 2)));
                    let mut temp3 = self.temp_float((*get_op_mut(inst, 3)));

                    let mut addr = self.temp_addr(
                        (*get_op_mut(inst, 0)),
                        (core::mem::offset_of!(TValue, value) as i32),
                    );
                    CODEGEN_ASSERT!(
                        addr.kind == AddressKindA64::imm
                            && addr.data % 4 == 0
                            && ((addr.data + 8) as u32) / 4 <= AddressA64::kMaxOffset as u32
                    );

                    (*self.build).str(temp1, mem(addr.base, addr.data + 0));
                    (*self.build).str(temp2, mem(addr.base, addr.data + 4));
                    (*self.build).str(temp3, mem(addr.base, addr.data + 8));

                    if has_op_e(inst) {
                        let mut temp = self.regs.alloc_temp(KindA64::w);
                        (*self.build).mov(temp, self.tag_op((*get_op_mut(inst, 4))));
                        (*self.build).str(
                            temp,
                            self.temp_addr(
                                (*get_op_mut(inst, 0)),
                                (core::mem::offset_of!(TValue, tt) as i32),
                            ),
                        );
                    }
                }
                IrCmd::STORE_TVALUE => {
                    let mut addrOffset = if has_op_c(inst) {
                        self.int_op((*get_op_mut(inst, 2)))
                    } else {
                        0
                    };
                    let mut addr = self.temp_addr((*get_op_mut(inst, 0)), addrOffset);
                    (*self.build).str(self.reg_op((*get_op_mut(inst, 1))), addr);
                }
                IrCmd::STORE_SPLIT_TVALUE => {
                    {
                        let mut addrOffset = if has_op_d(inst) {
                            self.int_op((*get_op_mut(inst, 3)))
                        } else {
                            0
                        };

                        let mut tempt = self.regs.alloc_temp(KindA64::w);
                        let mut addrt = self.temp_addr(
                            (*get_op_mut(inst, 0)),
                            (core::mem::offset_of!(TValue, tt) as i32) + addrOffset,
                        );
                        (*self.build).mov(tempt, self.tag_op((*get_op_mut(inst, 1))));
                        (*self.build).str(tempt, addrt);

                        let mut addr = self.temp_addr(
                            (*get_op_mut(inst, 0)),
                            (core::mem::offset_of!(TValue, value) as i32) + addrOffset,
                        );

                        if self.tag_op((*get_op_mut(inst, 1))) == LUA_TBOOLEAN {
                            if (*get_op_mut(inst, 2)).kind() == IrOpKind::Constant {
                                // note: we reuse tag temp register as value for true booleans, and use built-in zero register for false values
                                CODEGEN_ASSERT!(LUA_TBOOLEAN == 1);
                                (*self.build).str(
                                    if self.int_op((*get_op_mut(inst, 2))) != 0 {
                                        tempt
                                    } else {
                                        wzr
                                    },
                                    addr,
                                );
                            } else {
                                (*self.build).str(self.reg_op((*get_op_mut(inst, 2))), addr);
                            }
                        } else if self.tag_op((*get_op_mut(inst, 1))) == LUA_TNUMBER {
                            let mut temp = self.temp_double((*get_op_mut(inst, 2)));
                            (*self.build).str(temp, addr);
                        } else if self.tag_op((*get_op_mut(inst, 1))) == LUA_TINTEGER {
                            let mut temp = self.temp_int64((*get_op_mut(inst, 2)));
                            (*self.build).str(temp, addr);
                        } else if is_gco(self.tag_op((*get_op_mut(inst, 1)))) {
                            (*self.build).str(self.reg_op((*get_op_mut(inst, 2))), addr);
                        } else {
                            CODEGEN_ASSERT!(false, "Unsupported instruction form");
                        }
                    }
                }
                IrCmd::ADD_INT => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::w,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );
                    if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
                        && ((self.int_op((*get_op_mut(inst, 1)))) as u32) <= K_MAX_IMMEDIATE
                    {
                        (*self.build).add(
                            inst.reg_a64,
                            self.reg_op((*get_op_mut(inst, 0))),
                            ((self.int_op((*get_op_mut(inst, 1)))) as u16),
                        );
                    } else if (*get_op_mut(inst, 0)).kind() == IrOpKind::Constant
                        && ((self.int_op((*get_op_mut(inst, 0)))) as u32) <= K_MAX_IMMEDIATE
                    {
                        (*self.build).add(
                            inst.reg_a64,
                            self.reg_op((*get_op_mut(inst, 1))),
                            ((self.int_op((*get_op_mut(inst, 0)))) as u16),
                        );
                    } else {
                        let mut temp1 = self.temp_int((*get_op_mut(inst, 0)));
                        let mut temp2 = self.temp_int((*get_op_mut(inst, 1)));
                        (*self.build).add(inst.reg_a64, temp1, temp2);
                    }
                }
                IrCmd::SUB_INT => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::w,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );
                    if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
                        && ((self.int_op((*get_op_mut(inst, 1)))) as u32) <= K_MAX_IMMEDIATE
                    {
                        (*self.build).sub(
                            inst.reg_a64,
                            self.reg_op((*get_op_mut(inst, 0))),
                            ((self.int_op((*get_op_mut(inst, 1)))) as u16),
                        );
                    } else {
                        let mut temp1 = self.temp_int((*get_op_mut(inst, 0)));
                        let mut temp2 = self.temp_int((*get_op_mut(inst, 1)));
                        (*self.build).sub(inst.reg_a64, temp1, temp2);
                    }
                }
                IrCmd::ADD_INT64 => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::x,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );
                    if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
                        && ((self.int_64_op((*get_op_mut(inst, 1)))) as u64)
                            <= K_MAX_IMMEDIATE as u64
                    {
                        (*self.build).add(
                            inst.reg_a64,
                            self.temp_int64((*get_op_mut(inst, 0))),
                            ((self.int_64_op((*get_op_mut(inst, 1)))) as u16),
                        );
                    } else if (*get_op_mut(inst, 0)).kind() == IrOpKind::Constant
                        && ((self.int_64_op((*get_op_mut(inst, 0)))) as u64)
                            <= K_MAX_IMMEDIATE as u64
                    {
                        (*self.build).add(
                            inst.reg_a64,
                            self.temp_int64((*get_op_mut(inst, 1))),
                            ((self.int_64_op((*get_op_mut(inst, 0)))) as u16),
                        );
                    } else {
                        let mut temp1 = self.temp_int64((*get_op_mut(inst, 0)));
                        let mut temp2 = self.temp_int64((*get_op_mut(inst, 1)));
                        (*self.build).add(inst.reg_a64, temp1, temp2);
                    }
                }
                IrCmd::SUB_INT64 => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::x,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );
                    if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
                        && ((self.int_64_op((*get_op_mut(inst, 1)))) as u64)
                            <= K_MAX_IMMEDIATE as u64
                    {
                        (*self.build).sub(
                            inst.reg_a64,
                            self.temp_int64((*get_op_mut(inst, 0))),
                            ((self.int_64_op((*get_op_mut(inst, 1)))) as u16),
                        );
                    } else {
                        let mut temp1 = self.temp_int64((*get_op_mut(inst, 0)));
                        let mut temp2 = self.temp_int64((*get_op_mut(inst, 1)));
                        (*self.build).sub(inst.reg_a64, temp1, temp2);
                    }
                }
                IrCmd::MUL_INT64 => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::x,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );
                    {
                        let mut temp1 = self.temp_int64((*get_op_mut(inst, 0)));
                        let mut temp2 = self.temp_int64((*get_op_mut(inst, 1)));
                        (*self.build).mul(inst.reg_a64, temp1, temp2);
                    }
                }
                IrCmd::DIV_INT64 => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::x,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );
                    {
                        let mut temp1 = self.temp_int64((*get_op_mut(inst, 0)));
                        let mut temp2 = self.temp_int64((*get_op_mut(inst, 1)));
                        (*self.build).sdiv(inst.reg_a64, temp1, temp2);
                    }
                }
                IrCmd::IDIV_INT64 => {
                    // floored division: q = a / b, then if (q < 0 && a % b != 0) q -= 1
                    inst.reg_a64 = self.regs.alloc_reg(KindA64::x, index); // can't reuse: both operands needed for remainder
                    {
                        let mut temp1 = self.temp_int64((*get_op_mut(inst, 0)));
                        let mut temp2 = self.temp_int64((*get_op_mut(inst, 1)));
                        let mut tempRem = self.regs.alloc_temp(KindA64::x);
                        let mut tempAdj = self.regs.alloc_temp(KindA64::x);

                        (*self.build).sdiv(inst.reg_a64, temp1, temp2); // result = a / b
                        (*self.build).mov(tempRem, inst.reg_a64); // copy quotient; rem requires dst to initially hold quotient
                        (*self.build).rem(tempRem, temp1, temp2);

                        (*self.build).sub(tempAdj, inst.reg_a64, ((1) as u16)); // adjusted = result - 1

                        (*self.build).cmp(tempRem, ((0) as u16));
                        (*self.build).csel(tempAdj, tempAdj, inst.reg_a64, ConditionA64::NotEqual); // (remainder != 0) ? result-1 : result

                        (*self.build).cmp(inst.reg_a64, ((0) as u16));
                        (*self.build).csel(inst.reg_a64, tempAdj, inst.reg_a64, ConditionA64::Less);
                        // (result < 0) ? tempAdj : result
                    }
                }
                IrCmd::CHECK_DIV_INT64 => {
                    {
                        let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
                        let fail = self.ir_lowering_a_64_get_target_label(
                            (*get_op_mut(inst, 2)),
                            index,
                            &mut fresh,
                        ) as *mut Label;

                        // guard against divide by zero
                        let mut regB = self.temp_int64((*get_op_mut(inst, 1)));
                        (*self.build).cbz(regB, &mut *fail);

                        // guard against if a is -2^63 and b is -1
                        let mut regA = self.temp_int64((*get_op_mut(inst, 0)));
                        let mut tempRotate = self.regs.alloc_temp(KindA64::x);

                        // bit trick, if we are integer.minsigned (0x8000000000000000), then if we rotate by 63, we will get 1
                        (*self.build).ror(tempRotate, regA, 63);

                        (*self.build).cmp(tempRotate, ((1) as u16));

                        // nzcv = 0000 EQ
                        // nzcv = 0001 NE
                        (*self.build).ccmn(regB, 1, get_condition_int64(IrCondition::Equal), 1);
                        (*self.build).b_condition_a_64_label(
                            get_condition_int64(IrCondition::Equal),
                            &mut *fail,
                        );

                        self.ir_lowering_a_64_finalize_target_label(
                            (*get_op_mut(inst, 2)),
                            index,
                            &mut fresh,
                        );
                    }
                }
                IrCmd::UDIV_INT64 => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::x,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );
                    {
                        let mut temp1 = self.temp_int64((*get_op_mut(inst, 0)));
                        let mut temp2 = self.temp_int64((*get_op_mut(inst, 1)));
                        (*self.build).udiv(inst.reg_a64, temp1, temp2);
                    }
                }
                IrCmd::REM_INT64 => {
                    inst.reg_a64 = self.regs.alloc_reg(KindA64::x, index);
                    {
                        let mut temp1 = self.temp_int64((*get_op_mut(inst, 0)));
                        let mut temp2 = self.temp_int64((*get_op_mut(inst, 1)));
                        (*self.build).sdiv(inst.reg_a64, temp1, temp2);
                        (*self.build).rem(inst.reg_a64, temp1, temp2);
                    }
                }
                IrCmd::MOD_INT64 => {
                    // floored modulo: rem = a % b (C truncated); if (rem != 0 && sign(rem) != sign(b)) rem += b
                    inst.reg_a64 = self.regs.alloc_reg(KindA64::x, index); // can't reuse: dividend (temp1) needed after sdiv
                    {
                        let mut temp1 = self.temp_int64((*get_op_mut(inst, 0)));
                        let mut temp2 = self.temp_int64((*get_op_mut(inst, 1)));
                        let mut tempRem = self.regs.alloc_temp(KindA64::x);
                        let mut tempAdj = self.regs.alloc_temp(KindA64::x);

                        (*self.build).sdiv(inst.reg_a64, temp1, temp2); // quotient = a / b
                        (*self.build).mov(tempRem, inst.reg_a64); // tempRem = quotient
                        (*self.build).rem(tempRem, temp1, temp2); // tempRem = C-style remainder

                        (*self.build).add(tempAdj, tempRem, temp2); // tempAdj = rem + b (floored candidate)
                        (*self.build).eor(inst.reg_a64, tempRem, temp2); // sign check: negative if signs differ

                        (*self.build).cmp(inst.reg_a64, ((0) as u16));
                        (*self.build).csel(tempAdj, tempAdj, tempRem, ConditionA64::Less); // if signs differ then rem+b else rem

                        (*self.build).cmp(tempRem, ((0) as u16));
                        (*self.build).csel(inst.reg_a64, tempAdj, tempRem, ConditionA64::NotEqual);
                        // if rem != 0 then adjusted else 0
                    }
                }
                IrCmd::UREM_INT64 => {
                    inst.reg_a64 = self.regs.alloc_reg(KindA64::x, index);
                    {
                        let mut temp1 = self.temp_int64((*get_op_mut(inst, 0)));
                        let mut temp2 = self.temp_int64((*get_op_mut(inst, 1)));
                        (*self.build).udiv(inst.reg_a64, temp1, temp2);
                        (*self.build).rem(inst.reg_a64, temp1, temp2);
                    }
                }
                IrCmd::SEXTI8_INT => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::w, index, &[(*get_op_mut(inst, 0))]);

                    (*self.build).sbfx(inst.reg_a64, self.reg_op((*get_op_mut(inst, 0))), 0, 8);
                    // sextb
                }
                IrCmd::SEXTI16_INT => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::w, index, &[(*get_op_mut(inst, 0))]);

                    (*self.build).sbfx(inst.reg_a64, self.reg_op((*get_op_mut(inst, 0))), 0, 16);
                    // sexth
                }
                IrCmd::ADD_NUM => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::d,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );
                    let mut temp1 = self.temp_double((*get_op_mut(inst, 0)));
                    let mut temp2 = self.temp_double((*get_op_mut(inst, 1)));
                    (*self.build).fadd(inst.reg_a64, temp1, temp2);
                }
                IrCmd::SUB_NUM => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::d,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );
                    let mut temp1 = self.temp_double((*get_op_mut(inst, 0)));
                    let mut temp2 = self.temp_double((*get_op_mut(inst, 1)));
                    (*self.build).fsub(inst.reg_a64, temp1, temp2);
                }
                IrCmd::MUL_NUM => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::d,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );
                    let mut temp1 = self.temp_double((*get_op_mut(inst, 0)));
                    let mut temp2 = self.temp_double((*get_op_mut(inst, 1)));
                    (*self.build).fmul(inst.reg_a64, temp1, temp2);
                }
                IrCmd::DIV_NUM => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::d,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );
                    let mut temp1 = self.temp_double((*get_op_mut(inst, 0)));
                    let mut temp2 = self.temp_double((*get_op_mut(inst, 1)));
                    (*self.build).fdiv(inst.reg_a64, temp1, temp2);
                }
                IrCmd::IDIV_NUM => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::d,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );
                    let mut temp1 = self.temp_double((*get_op_mut(inst, 0)));
                    let mut temp2 = self.temp_double((*get_op_mut(inst, 1)));
                    (*self.build).fdiv(inst.reg_a64, temp1, temp2);
                    (*self.build).frintm(inst.reg_a64, inst.reg_a64);
                }
                IrCmd::MOD_NUM => {
                    {
                        inst.reg_a64 = self.regs.alloc_reg(KindA64::d, index); // can't allocReuse because both A and B are used twice
                        let mut temp1 = self.temp_double((*get_op_mut(inst, 0)));
                        let mut temp2 = self.temp_double((*get_op_mut(inst, 1)));
                        (*self.build).fdiv(inst.reg_a64, temp1, temp2);
                        (*self.build).frintm(inst.reg_a64, inst.reg_a64);
                        (*self.build).fmul(inst.reg_a64, inst.reg_a64, temp2);
                        (*self.build).fsub(inst.reg_a64, temp1, inst.reg_a64);
                    }
                }
                IrCmd::MULADD_NUM => {
                    let mut tempA = self.temp_double((*get_op_mut(inst, 0)));
                    let mut tempB = self.temp_double((*get_op_mut(inst, 1)));
                    let mut tempC = self.temp_double((*get_op_mut(inst, 2)));

                    if ((*self.build).features & Feature_AdvSIMD) != 0 {
                        inst.reg_a64 =
                            self.regs
                                .alloc_reuse(KindA64::d, index, &[(*get_op_mut(inst, 2))]);
                        if inst.reg_a64 != tempC {
                            (*self.build).fmov(inst.reg_a64, tempC);
                        }
                        (*self.build).fmla(inst.reg_a64, tempB, tempA);
                    } else {
                        inst.reg_a64 = self.regs.alloc_reg(KindA64::d, index);
                        (*self.build).fmul(inst.reg_a64, tempB, tempA);
                        (*self.build).fadd(inst.reg_a64, inst.reg_a64, tempC);
                    }
                }
                IrCmd::MIN_NUM => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::d,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );
                    let mut temp1 = self.temp_double((*get_op_mut(inst, 0)));
                    let mut temp2 = self.temp_double((*get_op_mut(inst, 1)));
                    (*self.build).fcmp(temp1, temp2);
                    (*self.build).fcsel(
                        inst.reg_a64,
                        temp1,
                        temp2,
                        get_condition_f_p(IrCondition::Less),
                    );
                }
                IrCmd::MAX_NUM => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::d,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );
                    let mut temp1 = self.temp_double((*get_op_mut(inst, 0)));
                    let mut temp2 = self.temp_double((*get_op_mut(inst, 1)));
                    (*self.build).fcmp(temp1, temp2);
                    (*self.build).fcsel(
                        inst.reg_a64,
                        temp1,
                        temp2,
                        get_condition_f_p(IrCondition::Greater),
                    );
                }
                IrCmd::UNM_NUM => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::d, index, &[(*get_op_mut(inst, 0))]);
                    let mut temp = self.temp_double((*get_op_mut(inst, 0)));
                    (*self.build).fneg(inst.reg_a64, temp);
                }
                IrCmd::FLOOR_NUM => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::d, index, &[(*get_op_mut(inst, 0))]);
                    let mut temp = self.temp_double((*get_op_mut(inst, 0)));
                    (*self.build).frintm(inst.reg_a64, temp);
                }
                IrCmd::CEIL_NUM => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::d, index, &[(*get_op_mut(inst, 0))]);
                    let mut temp = self.temp_double((*get_op_mut(inst, 0)));
                    (*self.build).frintp(inst.reg_a64, temp);
                }
                IrCmd::ROUND_NUM => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::d, index, &[(*get_op_mut(inst, 0))]);
                    let mut temp = self.temp_double((*get_op_mut(inst, 0)));
                    (*self.build).frinta(inst.reg_a64, temp);
                }
                IrCmd::SQRT_NUM => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::d, index, &[(*get_op_mut(inst, 0))]);
                    let mut temp = self.temp_double((*get_op_mut(inst, 0)));
                    (*self.build).fsqrt(inst.reg_a64, temp);
                }
                IrCmd::ABS_NUM => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::d, index, &[(*get_op_mut(inst, 0))]);
                    let mut temp = self.temp_double((*get_op_mut(inst, 0)));
                    (*self.build).fabs(inst.reg_a64, temp);
                }
                IrCmd::SIGN_NUM => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::d, index, &[(*get_op_mut(inst, 0))]);

                    let mut temp = self.temp_double((*get_op_mut(inst, 0)));
                    let mut temp0 = self.regs.alloc_temp(KindA64::d);
                    let mut temp1 = self.regs.alloc_temp(KindA64::d);

                    (*self.build).fcmpz(temp);
                    (*self.build).fmov(temp0, 0.0);
                    (*self.build).fmov(temp1, 1.0);
                    (*self.build).fcsel(
                        inst.reg_a64,
                        temp1,
                        temp0,
                        get_condition_f_p(IrCondition::Greater),
                    );
                    (*self.build).fmov(temp1, -1.0);
                    (*self.build).fcsel(
                        inst.reg_a64,
                        temp1,
                        inst.reg_a64,
                        get_condition_f_p(IrCondition::Less),
                    );
                }
                IrCmd::ADD_FLOAT => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::s,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );
                    let mut temp1 = self.temp_float((*get_op_mut(inst, 0)));
                    let mut temp2 = self.temp_float((*get_op_mut(inst, 1)));
                    (*self.build).fadd(inst.reg_a64, temp1, temp2);
                }
                IrCmd::SUB_FLOAT => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::s,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );
                    let mut temp1 = self.temp_float((*get_op_mut(inst, 0)));
                    let mut temp2 = self.temp_float((*get_op_mut(inst, 1)));
                    (*self.build).fsub(inst.reg_a64, temp1, temp2);
                }
                IrCmd::MUL_FLOAT => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::s,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );
                    let mut temp1 = self.temp_float((*get_op_mut(inst, 0)));
                    let mut temp2 = self.temp_float((*get_op_mut(inst, 1)));
                    (*self.build).fmul(inst.reg_a64, temp1, temp2);
                }
                IrCmd::DIV_FLOAT => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::s,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );
                    let mut temp1 = self.temp_float((*get_op_mut(inst, 0)));
                    let mut temp2 = self.temp_float((*get_op_mut(inst, 1)));
                    (*self.build).fdiv(inst.reg_a64, temp1, temp2);
                }
                IrCmd::MIN_FLOAT => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::s,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );
                    let mut temp1 = self.temp_float((*get_op_mut(inst, 0)));
                    let mut temp2 = self.temp_float((*get_op_mut(inst, 1)));
                    (*self.build).fcmp(temp1, temp2);
                    (*self.build).fcsel(
                        inst.reg_a64,
                        temp1,
                        temp2,
                        get_condition_f_p(IrCondition::Less),
                    );
                }
                IrCmd::MAX_FLOAT => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::s,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );
                    let mut temp1 = self.temp_float((*get_op_mut(inst, 0)));
                    let mut temp2 = self.temp_float((*get_op_mut(inst, 1)));
                    (*self.build).fcmp(temp1, temp2);
                    (*self.build).fcsel(
                        inst.reg_a64,
                        temp1,
                        temp2,
                        get_condition_f_p(IrCondition::Greater),
                    );
                }
                IrCmd::UNM_FLOAT => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::s, index, &[(*get_op_mut(inst, 0))]);
                    let mut temp = self.temp_float((*get_op_mut(inst, 0)));
                    (*self.build).fneg(inst.reg_a64, temp);
                }
                IrCmd::FLOOR_FLOAT => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::s, index, &[(*get_op_mut(inst, 0))]);
                    let mut temp = self.temp_float((*get_op_mut(inst, 0)));
                    (*self.build).frintm(inst.reg_a64, temp);
                }
                IrCmd::CEIL_FLOAT => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::s, index, &[(*get_op_mut(inst, 0))]);
                    let mut temp = self.temp_float((*get_op_mut(inst, 0)));
                    (*self.build).frintp(inst.reg_a64, temp);
                }
                IrCmd::SQRT_FLOAT => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::s, index, &[(*get_op_mut(inst, 0))]);
                    let mut temp = self.temp_float((*get_op_mut(inst, 0)));
                    (*self.build).fsqrt(inst.reg_a64, temp);
                }
                IrCmd::ABS_FLOAT => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::s, index, &[(*get_op_mut(inst, 0))]);
                    let mut temp = self.temp_float((*get_op_mut(inst, 0)));
                    (*self.build).fabs(inst.reg_a64, temp);
                }
                IrCmd::SIGN_FLOAT => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::s, index, &[(*get_op_mut(inst, 0))]);

                    let mut temp = self.temp_float((*get_op_mut(inst, 0)));
                    let mut temp0 = self.regs.alloc_temp(KindA64::s);
                    let mut temp1 = self.regs.alloc_temp(KindA64::s);

                    (*self.build).fcmpz(temp);
                    (*self.build).fmov(temp0, 0.0);
                    (*self.build).fmov(temp1, 1.0);
                    (*self.build).fcsel(
                        inst.reg_a64,
                        temp1,
                        temp0,
                        get_condition_f_p(IrCondition::Greater),
                    );
                    (*self.build).fmov(temp1, -1.0);
                    (*self.build).fcsel(
                        inst.reg_a64,
                        temp1,
                        inst.reg_a64,
                        get_condition_f_p(IrCondition::Less),
                    );
                }
                IrCmd::SELECT_NUM => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::d,
                        index,
                        &[
                            (*get_op_mut(inst, 0)),
                            (*get_op_mut(inst, 1)),
                            (*get_op_mut(inst, 2)),
                            (*get_op_mut(inst, 3)),
                        ],
                    );

                    let mut temp1 = self.temp_double((*get_op_mut(inst, 0)));
                    let mut temp2 = self.temp_double((*get_op_mut(inst, 1)));
                    let mut temp3 = self.temp_double((*get_op_mut(inst, 2)));
                    let mut temp4 = self.temp_double((*get_op_mut(inst, 3)));

                    (*self.build).fcmp(temp3, temp4);
                    (*self.build).fcsel(
                        inst.reg_a64,
                        temp2,
                        temp1,
                        get_condition_f_p(IrCondition::Equal),
                    );
                }
                IrCmd::SELECT_INT64 => {
                    let mut cond = condition_op((*get_op_mut(inst, 4)));

                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::x,
                        index,
                        &[
                            (*get_op_mut(inst, 0)),
                            (*get_op_mut(inst, 1)),
                            (*get_op_mut(inst, 2)),
                            (*get_op_mut(inst, 3)),
                        ],
                    );

                    let mut temp1 = self.temp_int64((*get_op_mut(inst, 0)));
                    let mut temp2 = self.temp_int64((*get_op_mut(inst, 1)));
                    let mut temp3 = self.temp_int64((*get_op_mut(inst, 2)));
                    let mut temp4 = self.temp_int64((*get_op_mut(inst, 3)));

                    (*self.build).cmp(temp3, temp4);
                    (*self.build).csel(inst.reg_a64, temp2, temp1, get_condition_int64(cond));
                }
                IrCmd::SELECT_VEC => {
                    {
                        // `(*get_op_mut(inst, 1))` cannot be reused for return value, because it can be overwritten with A before the first usage
                        inst.reg_a64 = self.regs.alloc_reuse(
                            KindA64::q,
                            index,
                            &[
                                (*get_op_mut(inst, 0)),
                                (*get_op_mut(inst, 2)),
                                (*get_op_mut(inst, 3)),
                            ],
                        );

                        let mut temp1 = self.reg_op((*get_op_mut(inst, 0)));
                        let mut temp2 = self.reg_op((*get_op_mut(inst, 1)));
                        let mut temp3 = self.reg_op((*get_op_mut(inst, 2)));
                        let mut temp4 = self.reg_op((*get_op_mut(inst, 3)));

                        let mut mask = self.regs.alloc_temp(KindA64::q);

                        // Evaluate predicate and calculate mask.
                        (*self.build).fcmeq_4s(mask, temp3, temp4);
                        // mov A to res register
                        (*self.build).mov(inst.reg_a64, temp1);
                        // If numbers are equal override A with B in res register.
                        (*self.build).bit(inst.reg_a64, temp2, mask);
                    }
                }
                IrCmd::SELECT_IF_TRUTHY => {
                    {
                        inst.reg_a64 = self.regs.alloc_reg(KindA64::q, index);

                        // Place lhs as the result, we will overwrite it with rhs if 'A' is falsy later
                        (*self.build).mov(inst.reg_a64, self.reg_op((*get_op_mut(inst, 1))));

                        // Get rhs register early, so a potential restore happens on both sides of a conditional control flow
                        let mut c = self.reg_op((*get_op_mut(inst, 2)));

                        let mut temp = self.regs.alloc_temp(KindA64::w);
                        let mut saveRhs = Label::default();
                        let mut exit = Label::default();

                        // Check tag first
                        (*self.build).umov_4s(temp, self.reg_op((*get_op_mut(inst, 0))), 3);
                        (*self.build).cmp(temp, ((LUA_TBOOLEAN) as u16));

                        (*self.build)
                            .b_condition_a_64_label(ConditionA64::UnsignedLess, &mut saveRhs); // rhs if 'A' is nil
                        (*self.build)
                            .b_condition_a_64_label(ConditionA64::UnsignedGreater, &mut exit); // Keep lhs if 'A' is not a boolean

                        // Check the boolean value
                        (*self.build).umov_4s(temp, self.reg_op((*get_op_mut(inst, 0))), 0);
                        (*self.build).cbnz(temp, &mut exit); // Keep lhs if 'A' is true

                        (*self.build).set_label_label(&mut saveRhs);
                        (*self.build).mov(inst.reg_a64, c);

                        (*self.build).set_label_label(&mut exit);
                    }
                }
                IrCmd::MULADD_VEC => {
                    let mut tempA = self.reg_op((*get_op_mut(inst, 0)));
                    let mut tempB = self.reg_op((*get_op_mut(inst, 1)));
                    let mut tempC = self.reg_op((*get_op_mut(inst, 2)));

                    if ((*self.build).features & Feature_AdvSIMD) != 0 {
                        inst.reg_a64 =
                            self.regs
                                .alloc_reuse(KindA64::q, index, &[(*get_op_mut(inst, 2))]);
                        if inst.reg_a64 != tempC {
                            (*self.build).mov(inst.reg_a64, tempC);
                        }
                        (*self.build).fmla(inst.reg_a64, tempB, tempA);
                    } else {
                        inst.reg_a64 = self.regs.alloc_reg(KindA64::q, index);
                        (*self.build).fmul(inst.reg_a64, tempB, tempA);
                        (*self.build).fadd(inst.reg_a64, inst.reg_a64, tempC);
                    }
                }
                IrCmd::ADD_VEC => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::q,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );

                    (*self.build).fadd(
                        inst.reg_a64,
                        self.reg_op((*get_op_mut(inst, 0))),
                        self.reg_op((*get_op_mut(inst, 1))),
                    );
                }
                IrCmd::SUB_VEC => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::q,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );

                    (*self.build).fsub(
                        inst.reg_a64,
                        self.reg_op((*get_op_mut(inst, 0))),
                        self.reg_op((*get_op_mut(inst, 1))),
                    );
                }
                IrCmd::MUL_VEC => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::q,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );

                    (*self.build).fmul(
                        inst.reg_a64,
                        self.reg_op((*get_op_mut(inst, 0))),
                        self.reg_op((*get_op_mut(inst, 1))),
                    );
                }
                IrCmd::DIV_VEC => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::q,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );

                    (*self.build).fdiv(
                        inst.reg_a64,
                        self.reg_op((*get_op_mut(inst, 0))),
                        self.reg_op((*get_op_mut(inst, 1))),
                    );
                }
                IrCmd::IDIV_VEC => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::q,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );

                    (*self.build).fdiv(
                        inst.reg_a64,
                        self.reg_op((*get_op_mut(inst, 0))),
                        self.reg_op((*get_op_mut(inst, 1))),
                    );
                    (*self.build).frintm(inst.reg_a64, inst.reg_a64);
                }
                IrCmd::UNM_VEC => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::q, index, &[(*get_op_mut(inst, 0))]);

                    (*self.build).fneg(inst.reg_a64, self.reg_op((*get_op_mut(inst, 0))));
                }
                IrCmd::MIN_VEC => {
                    {
                        inst.reg_a64 = self.regs.alloc_reuse(
                            KindA64::q,
                            index,
                            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                        );

                        let mut temp1 = self.reg_op((*get_op_mut(inst, 0)));
                        let mut temp2 = self.reg_op((*get_op_mut(inst, 1)));

                        let mut mask = self.regs.alloc_temp(KindA64::q);

                        // b > a == a < b
                        (*self.build).fcmgt_4s(mask, temp2, temp1);

                        // If A is already at the target, select B where mask is 0
                        if inst.reg_a64 == temp1 {
                            (*self.build).bif(inst.reg_a64, temp2, mask);
                        } else {
                            // Store B at the target unless it's there, select A where mask is 1
                            if inst.reg_a64 != temp2 {
                                (*self.build).mov(inst.reg_a64, temp2);
                            }

                            (*self.build).bit(inst.reg_a64, temp1, mask);
                        }
                    }
                }
                IrCmd::MAX_VEC => {
                    {
                        inst.reg_a64 = self.regs.alloc_reuse(
                            KindA64::q,
                            index,
                            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                        );

                        let mut temp1 = self.reg_op((*get_op_mut(inst, 0)));
                        let mut temp2 = self.reg_op((*get_op_mut(inst, 1)));

                        let mut mask = self.regs.alloc_temp(KindA64::q);

                        (*self.build).fcmgt_4s(mask, temp1, temp2);

                        // If A is already at the target, select B where mask is 0
                        if inst.reg_a64 == temp1 {
                            (*self.build).bif(inst.reg_a64, temp2, mask);
                        } else {
                            // Store B at the target unless it's there, select A where mask is 1
                            if inst.reg_a64 != temp2 {
                                (*self.build).mov(inst.reg_a64, temp2);
                            }

                            (*self.build).bit(inst.reg_a64, temp1, mask);
                        }
                    }
                }
                IrCmd::FLOOR_VEC => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::q, index, &[(*get_op_mut(inst, 0))]);

                    (*self.build).frintm(inst.reg_a64, self.reg_op((*get_op_mut(inst, 0))));
                }
                IrCmd::CEIL_VEC => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::q, index, &[(*get_op_mut(inst, 0))]);

                    (*self.build).frintp(inst.reg_a64, self.reg_op((*get_op_mut(inst, 0))));
                }
                IrCmd::ABS_VEC => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::q, index, &[(*get_op_mut(inst, 0))]);
                    (*self.build).fabs(inst.reg_a64, self.reg_op((*get_op_mut(inst, 0))));
                }
                IrCmd::DOT_VEC => {
                    {
                        inst.reg_a64 = self.regs.alloc_reg(KindA64::s, index);

                        let mut temp = self.regs.alloc_temp(KindA64::q);
                        let mut temps = cast_reg(KindA64::s, temp);

                        (*self.build).fmul(
                            temp,
                            self.reg_op((*get_op_mut(inst, 0))),
                            self.reg_op((*get_op_mut(inst, 1))),
                        );
                        (*self.build).faddp(inst.reg_a64, temps); // x+y
                        (*self.build).dup_4s(temp, temp, 2);
                        (*self.build).fadd(inst.reg_a64, inst.reg_a64, temps); // +z
                    }
                }
                IrCmd::EXTRACT_VEC => {
                    {
                        inst.reg_a64 = self.regs.alloc_reg(KindA64::s, index);

                        if self.int_op((*get_op_mut(inst, 1))) == 0 {
                            // Lane vN.s[0] can just be read directly as sN
                            (*self.build).fmov(
                                inst.reg_a64,
                                cast_reg(KindA64::s, self.reg_op((*get_op_mut(inst, 0)))),
                            );
                        } else {
                            (*self.build).dup_4s(
                                inst.reg_a64,
                                self.reg_op((*get_op_mut(inst, 0))),
                                self.int_op((*get_op_mut(inst, 1))) as u8,
                            );
                        }
                    }
                }
                IrCmd::NOT_ANY => {
                    {
                        inst.reg_a64 = self.regs.alloc_reuse(
                            KindA64::w,
                            index,
                            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                        );

                        if (*get_op_mut(inst, 0)).kind() == IrOpKind::Constant {
                            // other cases should've been constant folded
                            CODEGEN_ASSERT!(self.tag_op((*get_op_mut(inst, 0))) == LUA_TBOOLEAN);
                            (*self.build).eor(inst.reg_a64, self.reg_op((*get_op_mut(inst, 1))), 1);
                        } else {
                            let mut notBool = Label::default();
                            let mut exit = Label::default();

                            // use the fact that NIL is the only value less than BOOLEAN to do two tag comparisons at once
                            CODEGEN_ASSERT!(LUA_TNIL == 0 && LUA_TBOOLEAN == 1);
                            (*self.build)
                                .cmp(self.reg_op((*get_op_mut(inst, 0))), ((LUA_TBOOLEAN) as u16));
                            (*self.build)
                                .b_condition_a_64_label(ConditionA64::NotEqual, &mut notBool);

                            if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant {
                                (*self.build).mov(
                                    inst.reg_a64,
                                    if self.int_op((*get_op_mut(inst, 1))) == 0 {
                                        1
                                    } else {
                                        0
                                    },
                                );
                            } else {
                                (*self.build).eor(
                                    inst.reg_a64,
                                    self.reg_op((*get_op_mut(inst, 1))),
                                    1,
                                );
                            } // boolean => invert value

                            (*self.build).b(&mut exit);

                            // not boolean => result is true iff tag was nil
                            (*self.build).set_label_label(&mut notBool);
                            (*self.build).cset(inst.reg_a64, ConditionA64::Less);

                            (*self.build).set_label_label(&mut exit);
                        }
                    }
                }
                IrCmd::CMP_INT => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::w,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );

                    let mut cond = condition_op((*get_op_mut(inst, 2)));

                    if (*get_op_mut(inst, 0)).kind() == IrOpKind::Constant {
                        if ((self.int_op((*get_op_mut(inst, 0)))) as u32) <= K_MAX_IMMEDIATE {
                            (*self.build).cmp(
                                self.reg_op((*get_op_mut(inst, 1))),
                                ((self.int_op((*get_op_mut(inst, 0)))) as u16),
                            );
                        } else {
                            (*self.build).cmp(
                                self.reg_op((*get_op_mut(inst, 1))),
                                self.temp_int((*get_op_mut(inst, 0))),
                            );
                        }

                        (*self.build)
                            .cset(inst.reg_a64, get_inverse_condition(get_condition_int(cond)));
                    } else if (*get_op_mut(inst, 0)).kind() == IrOpKind::Inst {
                        if ((self.int_op((*get_op_mut(inst, 1)))) as u32) <= K_MAX_IMMEDIATE {
                            (*self.build).cmp(
                                self.reg_op((*get_op_mut(inst, 0))),
                                ((self.int_op((*get_op_mut(inst, 1)))) as u16),
                            );
                        } else {
                            (*self.build).cmp(
                                self.reg_op((*get_op_mut(inst, 0))),
                                self.temp_int((*get_op_mut(inst, 1))),
                            );
                        }

                        (*self.build).cset(inst.reg_a64, get_condition_int(cond));
                    } else {
                        CODEGEN_ASSERT!(false, "Unsupported instruction form");
                    }
                }
                IrCmd::CMP_INT64 => {
                    inst.reg_a64 = self.regs.alloc_reg(KindA64::w, index);

                    let mut cond = condition_op((*get_op_mut(inst, 2)));

                    if (*get_op_mut(inst, 0)).kind() == IrOpKind::Constant {
                        if ((self.int_64_op((*get_op_mut(inst, 0)))) as u64)
                            <= K_MAX_IMMEDIATE as u64
                        {
                            (*self.build).cmp(
                                self.reg_op((*get_op_mut(inst, 1))),
                                ((self.int_64_op((*get_op_mut(inst, 0)))) as u16),
                            );
                        } else {
                            (*self.build).cmp(
                                self.reg_op((*get_op_mut(inst, 1))),
                                self.temp_int64((*get_op_mut(inst, 0))),
                            );
                        }

                        (*self.build).cset(
                            inst.reg_a64,
                            get_inverse_condition(get_condition_int64(cond)),
                        );
                    } else if (*get_op_mut(inst, 0)).kind() == IrOpKind::Inst {
                        if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
                            && ((self.int_64_op((*get_op_mut(inst, 1)))) as u64)
                                <= K_MAX_IMMEDIATE as u64
                        {
                            (*self.build).cmp(
                                self.reg_op((*get_op_mut(inst, 0))),
                                ((self.int_64_op((*get_op_mut(inst, 1)))) as u16),
                            );
                        } else {
                            (*self.build).cmp(
                                self.reg_op((*get_op_mut(inst, 0))),
                                self.temp_int64((*get_op_mut(inst, 1))),
                            );
                        }

                        (*self.build).cset(inst.reg_a64, get_condition_int64(cond));
                    } else {
                        CODEGEN_ASSERT!(false, "Unsupported instruction form");
                    }
                }
                IrCmd::CMP_ANY => {
                    {
                        CODEGEN_ASSERT!(
                            (*get_op_mut(inst, 0)).kind() == IrOpKind::VmReg
                                && (*get_op_mut(inst, 1)).kind() == IrOpKind::VmReg
                        );
                        let mut cond = condition_op((*get_op_mut(inst, 2)));

                        inst.reg_a64 = self.regs.alloc_reg(KindA64::w, index);

                        let mut skip = Label::default();
                        let mut exit = Label::default();

                        // For equality comparison, 'luaV_equalval' expects tag to be equal before the call
                        if cond == IrCondition::Equal {
                            let mut tempa = self.regs.alloc_temp(KindA64::w);
                            let mut tempb = self.regs.alloc_temp(KindA64::w);

                            (*self.build).ldr(
                                tempa,
                                self.temp_addr(
                                    (*get_op_mut(inst, 0)),
                                    (core::mem::offset_of!(TValue, tt) as i32),
                                ),
                            );
                            (*self.build).ldr(
                                tempb,
                                self.temp_addr(
                                    (*get_op_mut(inst, 1)),
                                    (core::mem::offset_of!(TValue, tt) as i32),
                                ),
                            );
                            (*self.build).cmp(tempa, tempb);

                            // If the tags are not equal, skip the call and set result to 0
                            (*self.build).b_condition_a_64_label(ConditionA64::NotEqual, &mut skip);
                        }

                        // We have reserved the result register, so we can free it now so it is not recorded in the spill sequence
                        self.regs.free_reg(inst.reg_a64);

                        let mut spills = self
                            .regs
                            .spill_u32_initializer_list_register_a_64(index, &[]);

                        (*self.build).mov(x0, rState);
                        (*self.build).add(
                            x1,
                            r_base(),
                            ((vm_reg_op((*get_op_mut(inst, 0)))
                                * (core::mem::size_of::<TValue>() as i32))
                                as u16),
                        );
                        (*self.build).add(
                            x2,
                            r_base(),
                            ((vm_reg_op((*get_op_mut(inst, 1)))
                                * (core::mem::size_of::<TValue>() as i32))
                                as u16),
                        );

                        if cond == IrCondition::LessEqual {
                            (*self.build).ldr(
                                x3,
                                mem(
                                    rNativeContext,
                                    (core::mem::offset_of!(NativeContext, luaV_lessequal) as i32),
                                ),
                            );
                        } else if cond == IrCondition::Less {
                            (*self.build).ldr(
                                x3,
                                mem(
                                    rNativeContext,
                                    (core::mem::offset_of!(NativeContext, luaV_lessthan) as i32),
                                ),
                            );
                        } else if cond == IrCondition::Equal {
                            (*self.build).ldr(
                                x3,
                                mem(
                                    rNativeContext,
                                    (core::mem::offset_of!(NativeContext, luaV_equalval) as i32),
                                ),
                            );
                        } else {
                            CODEGEN_ASSERT!(false, "Unsupported condition");
                        }

                        (*self.build).blr(x3);

                        if inst.reg_a64 != w0 {
                            (*self.build).mov(inst.reg_a64, w0);
                        }

                        inst.reg_a64 = self.regs.take_reg(inst.reg_a64, index);

                        emit_update_base(&mut *self.build);

                        self.regs.restore_usize(spills);

                        if cond == IrCondition::Equal {
                            (*self.build).b(&mut exit);
                            (*self.build).set_label_label(&mut skip);

                            (*self.build).mov(inst.reg_a64, 0);
                            (*self.build).set_label_label(&mut exit);
                        }

                        // In case we made a call, skip high register bits clear, only consumer is JUMP_CMP_INT which doesn't read them
                    }
                }
                IrCmd::CMP_TAG => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::w,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );

                    let mut cond = condition_op((*get_op_mut(inst, 2)));
                    CODEGEN_ASSERT!(cond == IrCondition::Equal || cond == IrCondition::NotEqual);
                    let mut aReg = noreg;
                    let mut bReg = noreg;

                    if (*get_op_mut(inst, 0)).kind() == IrOpKind::Inst {
                        aReg = self.reg_op((*get_op_mut(inst, 0)));
                    } else if (*get_op_mut(inst, 0)).kind() == IrOpKind::VmReg {
                        aReg = self.regs.alloc_temp(KindA64::w);
                        let mut addr = self.temp_addr(
                            (*get_op_mut(inst, 0)),
                            (core::mem::offset_of!(TValue, tt) as i32),
                        );
                        (*self.build).ldr(aReg, addr);
                    } else {
                        CODEGEN_ASSERT!((*get_op_mut(inst, 0)).kind() == IrOpKind::Constant);
                    }

                    if (*get_op_mut(inst, 1)).kind() == IrOpKind::Inst {
                        bReg = self.reg_op((*get_op_mut(inst, 1)));
                    } else if (*get_op_mut(inst, 1)).kind() == IrOpKind::VmReg {
                        bReg = self.regs.alloc_temp(KindA64::w);
                        let mut addr = self.temp_addr(
                            (*get_op_mut(inst, 1)),
                            (core::mem::offset_of!(TValue, tt) as i32),
                        );
                        (*self.build).ldr(bReg, addr);
                    } else {
                        CODEGEN_ASSERT!((*get_op_mut(inst, 1)).kind() == IrOpKind::Constant);
                    }

                    if (*get_op_mut(inst, 0)).kind() == IrOpKind::Constant {
                        (*self.build).cmp(bReg, ((self.tag_op((*get_op_mut(inst, 0)))) as u16));
                        (*self.build)
                            .cset(inst.reg_a64, get_inverse_condition(get_condition_int(cond)));
                    } else if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant {
                        (*self.build).cmp(aReg, ((self.tag_op((*get_op_mut(inst, 1)))) as u16));
                        (*self.build).cset(inst.reg_a64, get_condition_int(cond));
                    } else {
                        (*self.build).cmp(aReg, bReg);
                        (*self.build).cset(inst.reg_a64, get_condition_int(cond));
                    }
                }
                IrCmd::CMP_SPLIT_TVALUE => {
                    {
                        inst.reg_a64 = self.regs.alloc_reuse(
                            KindA64::w,
                            index,
                            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                        );

                        // Second operand of this instruction must be a constant
                        // Without a constant type, we wouldn't know the correct way to compare the values at lowering time
                        CODEGEN_ASSERT!((*get_op_mut(inst, 1)).kind() == IrOpKind::Constant);

                        let mut cond = condition_op((*get_op_mut(inst, 4)));
                        CODEGEN_ASSERT!(
                            cond == IrCondition::Equal || cond == IrCondition::NotEqual
                        );

                        // Check tag equality first
                        let mut temp = self.regs.alloc_temp(KindA64::w);

                        if (*get_op_mut(inst, 0)).kind() != IrOpKind::Constant {
                            (*self.build).cmp(
                                self.reg_op((*get_op_mut(inst, 0))),
                                ((self.tag_op((*get_op_mut(inst, 1)))) as u16),
                            );
                            (*self.build).cset(temp, get_condition_int(cond));
                        } else {
                            // Constant folding had to handle different constant tags
                            CODEGEN_ASSERT!(
                                self.tag_op((*get_op_mut(inst, 0)))
                                    == self.tag_op((*get_op_mut(inst, 1)))
                            );
                        }

                        if self.tag_op((*get_op_mut(inst, 1))) == LUA_TBOOLEAN {
                            if (*get_op_mut(inst, 2)).kind() == IrOpKind::Constant {
                                CODEGEN_ASSERT!(
                                    self.int_op((*get_op_mut(inst, 2))) == 0
                                        || self.int_op((*get_op_mut(inst, 2))) == 1
                                );
                                (*self.build).cmp(
                                    self.reg_op((*get_op_mut(inst, 3))),
                                    ((self.int_op((*get_op_mut(inst, 2)))) as u16),
                                ); // swapped arguments
                            } else if (*get_op_mut(inst, 3)).kind() == IrOpKind::Constant {
                                CODEGEN_ASSERT!(
                                    self.int_op((*get_op_mut(inst, 3))) == 0
                                        || self.int_op((*get_op_mut(inst, 3))) == 1
                                );
                                (*self.build).cmp(
                                    self.reg_op((*get_op_mut(inst, 2))),
                                    ((self.int_op((*get_op_mut(inst, 3)))) as u16),
                                );
                            } else {
                                (*self.build).cmp(
                                    self.reg_op((*get_op_mut(inst, 2))),
                                    self.reg_op((*get_op_mut(inst, 3))),
                                );
                            }

                            (*self.build).cset(inst.reg_a64, get_condition_int(cond));
                        } else if self.tag_op((*get_op_mut(inst, 1))) == LUA_TSTRING {
                            (*self.build).cmp(
                                self.reg_op((*get_op_mut(inst, 2))),
                                self.reg_op((*get_op_mut(inst, 3))),
                            );
                            (*self.build).cset(inst.reg_a64, get_condition_int(cond));
                        } else if self.tag_op((*get_op_mut(inst, 1))) == LUA_TNUMBER {
                            let mut temp1 = self.temp_double((*get_op_mut(inst, 2)));
                            let mut temp2 = self.temp_double((*get_op_mut(inst, 3)));

                            (*self.build).fcmp(temp1, temp2);
                            (*self.build).cset(inst.reg_a64, get_condition_f_p(cond));
                        } else if self.tag_op((*get_op_mut(inst, 1))) == LUA_TINTEGER {
                            let mut temp1 = self.temp_int64((*get_op_mut(inst, 2)));
                            let mut temp2 = self.temp_int64((*get_op_mut(inst, 3)));

                            (*self.build).cmp(temp1, temp2);
                            (*self.build).cset(inst.reg_a64, get_condition_int64(cond));
                        } else {
                            CODEGEN_ASSERT!(false, "unsupported type tag in CMP_SPLIT_TVALUE");
                        }

                        if (*get_op_mut(inst, 0)).kind() != IrOpKind::Constant {
                            if cond == IrCondition::Equal {
                                (*self.build).and_(inst.reg_a64, inst.reg_a64, temp);
                            } else {
                                (*self.build).orr(inst.reg_a64, inst.reg_a64, temp);
                            }
                        }
                    }
                }
                IrCmd::JUMP => {
                    if (*get_op_mut(inst, 0)).kind() == IrOpKind::Undef
                        || (*get_op_mut(inst, 0)).kind() == IrOpKind::VmExit
                    {
                        let mut fresh = Label::default();
                        let target = self.ir_lowering_a_64_get_target_label(
                            (*get_op_mut(inst, 0)),
                            index,
                            &mut fresh,
                        ) as *mut Label;
                        (*self.build).b(&mut *target);
                        self.ir_lowering_a_64_finalize_target_label(
                            (*get_op_mut(inst, 0)),
                            index,
                            &mut fresh,
                        );
                    } else {
                        self.jump_or_fallthrough_op((*get_op_mut(inst, 0)), next);
                    }
                }
                IrCmd::JUMP_IF_TRUTHY => {
                    {
                        let mut temp = self.regs.alloc_temp(KindA64::w);
                        (*self.build).ldr(
                            temp,
                            mem(
                                r_base(),
                                vm_reg_op((*get_op_mut(inst, 0)))
                                    * (core::mem::size_of::<TValue>() as i32)
                                    + (core::mem::offset_of!(TValue, tt) as i32),
                            ),
                        );
                        // nil => falsy
                        CODEGEN_ASSERT!(LUA_TNIL == 0);
                        (*self.build).cbz(temp, self.label_op((*get_op_mut(inst, 2))));
                        // not boolean => truthy
                        (*self.build).cmp(temp, ((LUA_TBOOLEAN) as u16));
                        (*self.build).b_condition_a_64_label(
                            ConditionA64::NotEqual,
                            self.label_op((*get_op_mut(inst, 1))),
                        );
                        // compare boolean value
                        (*self.build).ldr(
                            temp,
                            mem(
                                r_base(),
                                vm_reg_op((*get_op_mut(inst, 0)))
                                    * (core::mem::size_of::<TValue>() as i32)
                                    + (core::mem::offset_of!(TValue, value) as i32),
                            ),
                        );
                        (*self.build).cbnz(temp, self.label_op((*get_op_mut(inst, 1))));
                        self.jump_or_fallthrough_op((*get_op_mut(inst, 2)), next);
                    }
                }
                IrCmd::JUMP_IF_FALSY => {
                    {
                        let mut temp = self.regs.alloc_temp(KindA64::w);
                        (*self.build).ldr(
                            temp,
                            mem(
                                r_base(),
                                vm_reg_op((*get_op_mut(inst, 0)))
                                    * (core::mem::size_of::<TValue>() as i32)
                                    + (core::mem::offset_of!(TValue, tt) as i32),
                            ),
                        );
                        // nil => falsy
                        CODEGEN_ASSERT!(LUA_TNIL == 0);
                        (*self.build).cbz(temp, self.label_op((*get_op_mut(inst, 1))));
                        // not boolean => truthy
                        (*self.build).cmp(temp, ((LUA_TBOOLEAN) as u16));
                        (*self.build).b_condition_a_64_label(
                            ConditionA64::NotEqual,
                            self.label_op((*get_op_mut(inst, 2))),
                        );
                        // compare boolean value
                        (*self.build).ldr(
                            temp,
                            mem(
                                r_base(),
                                vm_reg_op((*get_op_mut(inst, 0)))
                                    * (core::mem::size_of::<TValue>() as i32)
                                    + (core::mem::offset_of!(TValue, value) as i32),
                            ),
                        );
                        (*self.build).cbz(temp, self.label_op((*get_op_mut(inst, 1))));
                        self.jump_or_fallthrough_op((*get_op_mut(inst, 2)), next);
                    }
                }
                IrCmd::JUMP_EQ_TAG => {
                    let mut zr = noreg;
                    let mut aReg = noreg;
                    let mut bReg = noreg;

                    if (*get_op_mut(inst, 0)).kind() == IrOpKind::Inst {
                        aReg = self.reg_op((*get_op_mut(inst, 0)));
                    } else if (*get_op_mut(inst, 0)).kind() == IrOpKind::VmReg {
                        aReg = self.regs.alloc_temp(KindA64::w);
                        let mut addr = self.temp_addr(
                            (*get_op_mut(inst, 0)),
                            (core::mem::offset_of!(TValue, tt) as i32),
                        );
                        (*self.build).ldr(aReg, addr);
                    } else {
                        CODEGEN_ASSERT!((*get_op_mut(inst, 0)).kind() == IrOpKind::Constant);
                    }

                    if (*get_op_mut(inst, 1)).kind() == IrOpKind::Inst {
                        bReg = self.reg_op((*get_op_mut(inst, 1)));
                    } else if (*get_op_mut(inst, 1)).kind() == IrOpKind::VmReg {
                        bReg = self.regs.alloc_temp(KindA64::w);
                        let mut addr = self.temp_addr(
                            (*get_op_mut(inst, 1)),
                            (core::mem::offset_of!(TValue, tt) as i32),
                        );
                        (*self.build).ldr(bReg, addr);
                    } else {
                        CODEGEN_ASSERT!((*get_op_mut(inst, 1)).kind() == IrOpKind::Constant);
                    }

                    if (*get_op_mut(inst, 0)).kind() == IrOpKind::Constant
                        && self.tag_op((*get_op_mut(inst, 0))) == 0
                    {
                        zr = bReg;
                    } else if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
                        && self.tag_op((*get_op_mut(inst, 1))) == 0
                    {
                        zr = aReg;
                    } else if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant {
                        (*self.build).cmp(aReg, ((self.tag_op((*get_op_mut(inst, 1)))) as u16));
                    } else if (*get_op_mut(inst, 0)).kind() == IrOpKind::Constant {
                        (*self.build).cmp(bReg, ((self.tag_op((*get_op_mut(inst, 0)))) as u16));
                    } else {
                        (*self.build).cmp(aReg, bReg);
                    }

                    if self.is_fallthrough_block(self.block_op((*get_op_mut(inst, 3))), next) {
                        if zr != noreg {
                            (*self.build).cbz(zr, self.label_op((*get_op_mut(inst, 2))));
                        } else {
                            (*self.build).b_condition_a_64_label(
                                ConditionA64::Equal,
                                self.label_op((*get_op_mut(inst, 2))),
                            );
                        }
                        self.jump_or_fallthrough_op((*get_op_mut(inst, 3)), next);
                    } else {
                        if zr != noreg {
                            (*self.build).cbnz(zr, self.label_op((*get_op_mut(inst, 3))));
                        } else {
                            (*self.build).b_condition_a_64_label(
                                ConditionA64::NotEqual,
                                self.label_op((*get_op_mut(inst, 3))),
                            );
                        }
                        self.jump_or_fallthrough_op((*get_op_mut(inst, 2)), next);
                    }
                }
                IrCmd::JUMP_CMP_INT => {
                    let mut cond = condition_op((*get_op_mut(inst, 2)));

                    if cond == IrCondition::Equal && self.int_op((*get_op_mut(inst, 1))) == 0 {
                        (*self.build).cbz(
                            self.reg_op((*get_op_mut(inst, 0))),
                            self.label_op((*get_op_mut(inst, 3))),
                        );
                    } else if cond == IrCondition::NotEqual
                        && self.int_op((*get_op_mut(inst, 1))) == 0
                    {
                        (*self.build).cbnz(
                            self.reg_op((*get_op_mut(inst, 0))),
                            self.label_op((*get_op_mut(inst, 3))),
                        );
                    } else {
                        CODEGEN_ASSERT!(
                            ((self.int_op((*get_op_mut(inst, 1)))) as u32) <= K_MAX_IMMEDIATE
                        );
                        (*self.build).cmp(
                            self.reg_op((*get_op_mut(inst, 0))),
                            ((self.int_op((*get_op_mut(inst, 1)))) as u16),
                        );
                        (*self.build).b_condition_a_64_label(
                            get_condition_int(cond),
                            self.label_op((*get_op_mut(inst, 3))),
                        );
                    }
                    self.jump_or_fallthrough_op((*get_op_mut(inst, 4)), next);
                }
                IrCmd::JUMP_EQ_POINTER => {
                    (*self.build).cmp(
                        self.reg_op((*get_op_mut(inst, 0))),
                        self.reg_op((*get_op_mut(inst, 1))),
                    );
                    (*self.build).b_condition_a_64_label(
                        ConditionA64::Equal,
                        self.label_op((*get_op_mut(inst, 2))),
                    );
                    self.jump_or_fallthrough_op((*get_op_mut(inst, 3)), next);
                }
                IrCmd::JUMP_CMP_NUM => {
                    let mut cond = condition_op((*get_op_mut(inst, 2)));

                    if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
                        && self.double_op((*get_op_mut(inst, 1))) == 0.0
                    {
                        let mut temp = self.temp_double((*get_op_mut(inst, 0)));

                        (*self.build).fcmpz(temp);
                    } else {
                        let mut temp1 = self.temp_double((*get_op_mut(inst, 0)));
                        let mut temp2 = self.temp_double((*get_op_mut(inst, 1)));

                        (*self.build).fcmp(temp1, temp2);
                    }

                    (*self.build).b_condition_a_64_label(
                        get_condition_f_p(cond),
                        self.label_op((*get_op_mut(inst, 3))),
                    );
                    self.jump_or_fallthrough_op((*get_op_mut(inst, 4)), next);
                }
                IrCmd::JUMP_CMP_FLOAT => {
                    let mut cond = condition_op((*get_op_mut(inst, 2)));

                    if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
                        && ((self.double_op((*get_op_mut(inst, 1)))) as f32) == 0.0
                    {
                        let mut temp = self.temp_float((*get_op_mut(inst, 0)));

                        (*self.build).fcmpz(temp);
                    } else {
                        let mut temp1 = self.temp_float((*get_op_mut(inst, 0)));
                        let mut temp2 = self.temp_float((*get_op_mut(inst, 1)));

                        (*self.build).fcmp(temp1, temp2);
                    }

                    (*self.build).b_condition_a_64_label(
                        get_condition_f_p(cond),
                        self.label_op((*get_op_mut(inst, 3))),
                    );
                    self.jump_or_fallthrough_op((*get_op_mut(inst, 4)), next);
                }
                IrCmd::JUMP_FORN_LOOP_COND => {
                    {
                        let mut index = self.temp_double((*get_op_mut(inst, 0)));
                        let mut limit = self.temp_double((*get_op_mut(inst, 1)));
                        let mut step = self.temp_double((*get_op_mut(inst, 2)));

                        let mut direct = Label::default();

                        // step > 0
                        (*self.build).fcmpz(step);
                        (*self.build).b_condition_a_64_label(
                            get_condition_f_p(IrCondition::Greater),
                            &mut direct,
                        );

                        // !(limit <= index)
                        (*self.build).fcmp(limit, index);
                        (*self.build).b_condition_a_64_label(
                            get_condition_f_p(IrCondition::NotLessEqual),
                            self.label_op((*get_op_mut(inst, 4))),
                        );
                        (*self.build).b(self.label_op((*get_op_mut(inst, 3))));

                        // !(index <= limit)
                        (*self.build).set_label_label(&mut direct);

                        (*self.build).fcmp(index, limit);
                        (*self.build).b_condition_a_64_label(
                            get_condition_f_p(IrCondition::NotLessEqual),
                            self.label_op((*get_op_mut(inst, 4))),
                        );
                        self.jump_or_fallthrough_op((*get_op_mut(inst, 3)), next);
                    }
                    // IrCmd::JUMP_SLOT_MATCH implemented below
                }
                IrCmd::TABLE_LEN => {
                    {
                        let mut reg = self.reg_op((*get_op_mut(inst, 0))); // note: we need to call regOp before spill so that we don't do redundant reloads
                        self.regs
                            .spill_u32_initializer_list_register_a_64(index, &[reg]);
                        (*self.build).mov(x0, reg);
                        (*self.build).ldr(
                            x1,
                            mem(
                                rNativeContext,
                                (core::mem::offset_of!(NativeContext, luaH_getn) as i32),
                            ),
                        );
                        (*self.build).blr(x1);

                        inst.reg_a64 = self.regs.take_reg(w0, index);

                        (*self.build).ubfx(inst.reg_a64, inst.reg_a64, 0, 32); // Ensure high register bits are cleared
                    }
                }
                IrCmd::STRING_LEN => {
                    inst.reg_a64 = self.regs.alloc_reg(KindA64::w, index);

                    (*self.build).ldr(
                        inst.reg_a64,
                        mem(
                            self.reg_op((*get_op_mut(inst, 0))),
                            (K_TSTRING_LEN_OFFSET as i32),
                        ),
                    );
                }
                IrCmd::TABLE_SETNUM => {
                    {
                        // note: we need to call regOp before spill so that we don't do redundant reloads
                        let mut table = self.reg_op((*get_op_mut(inst, 0)));
                        let mut key = self.reg_op((*get_op_mut(inst, 1)));
                        let mut temp = self.regs.alloc_temp(KindA64::w);

                        self.regs
                            .spill_u32_initializer_list_register_a_64(index, &[table, key]);

                        if w1 != key {
                            (*self.build).mov(x1, table);
                            (*self.build).mov(w2, key);
                        } else {
                            (*self.build).mov(temp, w1);
                            (*self.build).mov(x1, table);
                            (*self.build).mov(w2, temp);
                        }

                        (*self.build).mov(x0, rState);
                        (*self.build).ldr(
                            x3,
                            mem(
                                rNativeContext,
                                (core::mem::offset_of!(NativeContext, luaH_setnum) as i32),
                            ),
                        );
                        (*self.build).blr(x3);
                        inst.reg_a64 = self.regs.take_reg(x0, index);
                    }
                }
                IrCmd::NEW_TABLE => {
                    self.regs
                        .spill_u32_initializer_list_register_a_64(index, &[]);
                    (*self.build).mov(x0, rState);
                    (*self.build).mov(x1, self.uint_op((*get_op_mut(inst, 0))));
                    (*self.build).mov(x2, self.uint_op((*get_op_mut(inst, 1))));
                    (*self.build).ldr(
                        x3,
                        mem(
                            rNativeContext,
                            (core::mem::offset_of!(NativeContext, luaH_new) as i32),
                        ),
                    );
                    (*self.build).blr(x3);
                    inst.reg_a64 = self.regs.take_reg(x0, index);
                }
                IrCmd::DUP_TABLE => {
                    {
                        let mut reg = self.reg_op((*get_op_mut(inst, 0))); // note: we need to call regOp before spill so that we don't do redundant reloads
                        self.regs
                            .spill_u32_initializer_list_register_a_64(index, &[reg]);
                        (*self.build).mov(x1, reg);
                        (*self.build).mov(x0, rState);
                        (*self.build).ldr(
                            x2,
                            mem(
                                rNativeContext,
                                (core::mem::offset_of!(NativeContext, luaH_clone) as i32),
                            ),
                        );
                        (*self.build).blr(x2);
                        inst.reg_a64 = self.regs.take_reg(x0, index);
                    }
                }
                IrCmd::TRY_NUM_TO_INDEX => {
                    {
                        inst.reg_a64 = self.regs.alloc_reg(KindA64::w, index);
                        let mut temp1 = self.temp_double((*get_op_mut(inst, 0)));

                        if ((*self.build).features & Feature_JSCVT) != 0 {
                            (*self.build).fjcvtzs(inst.reg_a64, temp1); // fjcvtzs sets PSTATE.Z (equal) iff conversion is exact
                            (*self.build).b_condition_a_64_label(
                                ConditionA64::NotEqual,
                                self.label_op((*get_op_mut(inst, 1))),
                            );
                        } else {
                            let mut temp2 = self.regs.alloc_temp(KindA64::d);

                            (*self.build).fcvtzs(inst.reg_a64, temp1);
                            (*self.build).scvtf(temp2, inst.reg_a64);
                            (*self.build).fcmp(temp1, temp2);
                            (*self.build).b_condition_a_64_label(
                                ConditionA64::NotEqual,
                                self.label_op((*get_op_mut(inst, 1))),
                            );
                        }
                    }
                }
                IrCmd::TRY_CALL_FASTGETTM => {
                    {
                        let mut temp1 = self.regs.alloc_temp(KindA64::x);
                        let mut temp2 = self.regs.alloc_temp(KindA64::w);

                        (*self.build).ldr(
                            temp1,
                            mem(
                                self.reg_op((*get_op_mut(inst, 0))),
                                (core::mem::offset_of!(LuaTable, metatable) as i32),
                            ),
                        );
                        (*self.build).cbz(temp1, self.label_op((*get_op_mut(inst, 2)))); // no metatable

                        (*self.build).ldrb(
                            temp2,
                            mem(temp1, (core::mem::offset_of!(LuaTable, tmcache) as i32)),
                        );
                        (*self.build).tst(temp2, 1 << self.int_op((*get_op_mut(inst, 1)))); // can't use tbz/tbnz because their jump offsets are too short
                        (*self.build).b_condition_a_64_label(
                            ConditionA64::NotEqual,
                            self.label_op((*get_op_mut(inst, 2))),
                        ); // Equal = Zero after tst; tmcache caches *absence* of metamethods

                        self.regs
                            .spill_u32_initializer_list_register_a_64(index, &[temp1]);
                        (*self.build).mov(x0, temp1);
                        (*self.build).mov(w1, self.int_op((*get_op_mut(inst, 1))));
                        (*self.build).ldr(
                            x2,
                            mem(
                                rGlobalState,
                                (core::mem::offset_of!(global_State, tmname) as i32)
                                    + self.int_op((*get_op_mut(inst, 1)))
                                        * (core::mem::size_of::<*mut TString>() as i32),
                            ),
                        );
                        (*self.build).ldr(
                            x3,
                            mem(
                                rNativeContext,
                                (core::mem::offset_of!(NativeContext, luaT_gettm) as i32),
                            ),
                        );
                        (*self.build).blr(x3);

                        (*self.build).cbz(x0, self.label_op((*get_op_mut(inst, 2)))); // no tag method

                        inst.reg_a64 = self.regs.take_reg(x0, index);
                    }
                }
                IrCmd::NEW_USERDATA => {
                    self.regs
                        .spill_u32_initializer_list_register_a_64(index, &[]);
                    (*self.build).mov(x0, rState);
                    (*self.build).mov(x1, self.int_op((*get_op_mut(inst, 0))));
                    (*self.build).mov(x2, self.int_op((*get_op_mut(inst, 1))));
                    (*self.build).ldr(
                        x3,
                        mem(
                            rNativeContext,
                            (core::mem::offset_of!(NativeContext, newUserdata) as i32),
                        ),
                    );
                    (*self.build).blr(x3);
                    inst.reg_a64 = self.regs.take_reg(x0, index);
                }
                IrCmd::INT64_TO_NUM => {
                    inst.reg_a64 = self.regs.alloc_reg(KindA64::d, index);
                    let mut temp = self.temp_int64((*get_op_mut(inst, 0)));
                    (*self.build).scvtf(inst.reg_a64, temp);
                }
                IrCmd::INT_TO_NUM => {
                    inst.reg_a64 = self.regs.alloc_reg(KindA64::d, index);
                    let mut temp = self.temp_int((*get_op_mut(inst, 0)));
                    (*self.build).scvtf(inst.reg_a64, temp);
                }
                IrCmd::UINT_TO_NUM => {
                    inst.reg_a64 = self.regs.alloc_reg(KindA64::d, index);
                    let mut temp = self.temp_int((*get_op_mut(inst, 0)));
                    (*self.build).ucvtf(inst.reg_a64, temp);
                }
                IrCmd::UINT_TO_FLOAT => {
                    inst.reg_a64 = self.regs.alloc_reg(KindA64::s, index);
                    let mut temp = self.temp_int((*get_op_mut(inst, 0)));
                    (*self.build).ucvtf(inst.reg_a64, temp);
                }
                IrCmd::NUM_TO_INT => {
                    inst.reg_a64 = self.regs.alloc_reg(KindA64::w, index);
                    let mut temp = self.temp_double((*get_op_mut(inst, 0)));
                    (*self.build).fcvtzs(inst.reg_a64, temp);
                }
                IrCmd::NUM_TO_INT64 => {
                    inst.reg_a64 = self.regs.alloc_reg(KindA64::x, index);
                    let mut temp = self.temp_double((*get_op_mut(inst, 0)));
                    (*self.build).fcvtzs(inst.reg_a64, temp);
                }
                IrCmd::NUM_TO_UINT => {
                    {
                        inst.reg_a64 = self.regs.alloc_reg(KindA64::w, index);
                        let mut temp = self.temp_double((*get_op_mut(inst, 0)));
                        // note: we don't use fcvtzu for consistency with C++ code
                        (*self.build).fcvtzs(cast_reg(KindA64::x, inst.reg_a64), temp);
                    }
                }
                IrCmd::FLOAT_TO_NUM => {
                    inst.reg_a64 = self.regs.alloc_reg(KindA64::d, index);

                    (*self.build).fcvt(inst.reg_a64, self.reg_op((*get_op_mut(inst, 0))));
                }
                IrCmd::NUM_TO_FLOAT => {
                    inst.reg_a64 = self.regs.alloc_reg(KindA64::s, index);

                    (*self.build).fcvt(inst.reg_a64, self.reg_op((*get_op_mut(inst, 0))));
                }
                IrCmd::FLOAT_TO_VEC => {
                    inst.reg_a64 = self.regs.alloc_reg(KindA64::q, index);

                    if (*get_op_mut(inst, 0)).kind() == IrOpKind::Constant {
                        let mut value = ((self.double_op((*get_op_mut(inst, 0)))) as f32);
                        let mut asU32 = value.to_bits();

                        if (*self.build).is_fmov_supported_fp_32(value) {
                            (*self.build).fmov(inst.reg_a64, value);
                        } else {
                            let mut temp = self.regs.alloc_temp(KindA64::x);

                            let mut vec = [asU32, asU32, asU32, 0u32];
                            (*self.build).adr_register_a_64_void_usize(
                                temp,
                                vec.as_ptr() as *const core::ffi::c_void,
                                core::mem::size_of_val(&vec),
                            );
                            (*self.build).ldr(inst.reg_a64, mem(temp, 0));
                        }
                    } else {
                        let mut temp = self.temp_float((*get_op_mut(inst, 0)));

                        (*self.build).dup_4s(inst.reg_a64, cast_reg(KindA64::q, temp), 0);
                    }
                }
                IrCmd::TAG_VECTOR => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::q, index, &[(*get_op_mut(inst, 0))]);

                    let mut reg = self.reg_op((*get_op_mut(inst, 0)));
                    let mut tempw = self.regs.alloc_temp(KindA64::w);

                    if inst.reg_a64 != reg {
                        (*self.build).mov(inst.reg_a64, reg);
                    }

                    (*self.build).mov(tempw, LUA_TVECTOR);
                    (*self.build).ins_4_s_register_a_64_register_a_64_u8(inst.reg_a64, tempw, 3);
                }
                IrCmd::TRUNCATE_UINT => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::w, index, &[(*get_op_mut(inst, 0))]);

                    (*self.build).ubfx(
                        cast_reg(KindA64::x, inst.reg_a64),
                        cast_reg(KindA64::x, self.reg_op((*get_op_mut(inst, 0)))),
                        0,
                        32,
                    ); // explicit uxtw
                }
                IrCmd::ADJUST_STACK_TO_REG => {
                    {
                        let mut temp = self.regs.alloc_temp(KindA64::x);

                        if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant {
                            (*self.build).add(
                                temp,
                                r_base(),
                                (((vm_reg_op((*get_op_mut(inst, 0)))
                                    + self.int_op((*get_op_mut(inst, 1))))
                                    * (core::mem::size_of::<TValue>() as i32))
                                    as u16),
                            );
                            (*self.build).str(
                                temp,
                                mem(rState, (core::mem::offset_of!(lua_State, top) as i32)),
                            );
                        } else if (*get_op_mut(inst, 1)).kind() == IrOpKind::Inst {
                            (*self.build).add(
                                temp,
                                r_base(),
                                ((vm_reg_op((*get_op_mut(inst, 0)))
                                    * (core::mem::size_of::<TValue>() as i32))
                                    as u16),
                            );
                            (*self.build).add_register_a_64_register_a_64_register_a_64_i32(
                                temp,
                                temp,
                                self.reg_op((*get_op_mut(inst, 1))),
                                kTValueSizeLog2,
                            ); // implicit uxtw
                            (*self.build).str(
                                temp,
                                mem(rState, (core::mem::offset_of!(lua_State, top) as i32)),
                            );
                        } else {
                            CODEGEN_ASSERT!(false, "Unsupported instruction form");
                        }
                    }
                }
                IrCmd::ADJUST_STACK_TO_TOP => {
                    let mut temp = self.regs.alloc_temp(KindA64::x);
                    (*self.build).ldr(
                        temp,
                        mem(rState, (core::mem::offset_of!(lua_State, ci) as i32)),
                    );
                    (*self.build).ldr(
                        temp,
                        mem(temp, (core::mem::offset_of!(CallInfo, top) as i32)),
                    );
                    (*self.build).str(
                        temp,
                        mem(rState, (core::mem::offset_of!(lua_State, top) as i32)),
                    );
                }
                IrCmd::FASTCALL => {
                    self.regs
                        .spill_u32_initializer_list_register_a_64(index, &[]);

                    let bfid = self.uint_op((*get_op_mut(inst, 0))) as i32;
                    let res = vm_reg_op((*get_op_mut(inst, 1)));
                    let arg = vm_reg_op((*get_op_mut(inst, 2)));
                    let nresults = self.int_op((*get_op_mut(inst, 3)));
                    self.error |= !emit_builtin(
                        &mut *self.build,
                        &mut *self.function,
                        &mut self.regs,
                        bfid,
                        res,
                        arg,
                        nresults,
                    );
                }
                IrCmd::INVOKE_FASTCALL => {
                    {
                        // We might need a temporary and we have to preserve it over the spill
                        let mut temp = self.regs.alloc_temp(KindA64::q);
                        self.regs
                            .spill_u32_initializer_list_register_a_64(index, &[temp]);

                        (*self.build).mov(x0, rState);
                        (*self.build).add(
                            x1,
                            r_base(),
                            ((vm_reg_op((*get_op_mut(inst, 1)))
                                * (core::mem::size_of::<TValue>() as i32))
                                as u16),
                        );
                        (*self.build).add(
                            x2,
                            r_base(),
                            ((vm_reg_op((*get_op_mut(inst, 2)))
                                * (core::mem::size_of::<TValue>() as i32))
                                as u16),
                        );
                        (*self.build).mov(w3, self.int_op((*get_op_mut(inst, 6)))); // nresults

                        // 'E' argument can only be produced by LOP_FASTCALL3 lowering
                        if (*get_op_mut(inst, 4)).kind() != IrOpKind::Undef {
                            CODEGEN_ASSERT!(self.int_op((*get_op_mut(inst, 5))) == 3);

                            (*self.build).ldr(
                                x4,
                                mem(rState, (core::mem::offset_of!(lua_State, top) as i32)),
                            );

                            (*self.build).ldr(
                                temp,
                                mem(
                                    r_base(),
                                    vm_reg_op((*get_op_mut(inst, 3)))
                                        * (core::mem::size_of::<TValue>() as i32),
                                ),
                            );
                            (*self.build).str(temp, mem(x4, 0));

                            (*self.build).ldr(
                                temp,
                                mem(
                                    r_base(),
                                    vm_reg_op((*get_op_mut(inst, 4)))
                                        * (core::mem::size_of::<TValue>() as i32),
                                ),
                            );
                            (*self.build)
                                .str(temp, mem(x4, (core::mem::size_of::<TValue>() as i32)));
                        } else {
                            if (*get_op_mut(inst, 3)).kind() == IrOpKind::VmReg {
                                (*self.build).add(
                                    x4,
                                    r_base(),
                                    ((vm_reg_op((*get_op_mut(inst, 3)))
                                        * (core::mem::size_of::<TValue>() as i32))
                                        as u16),
                                );
                            } else if (*get_op_mut(inst, 3)).kind() == IrOpKind::VmConst {
                                emit_add_offset(
                                    &mut *self.build,
                                    x4,
                                    r_constants(),
                                    vm_const_op((*get_op_mut(inst, 3)))
                                        * (core::mem::size_of::<TValue>() as i32),
                                );
                            } else {
                                CODEGEN_ASSERT!((*get_op_mut(inst, 3)).kind() == IrOpKind::Undef);
                            }
                        }

                        // nparams
                        if self.int_op((*get_op_mut(inst, 5))) == LUA_MULTRET {
                            // L->top - (ra + 1)
                            (*self.build).ldr(
                                x5,
                                mem(rState, (core::mem::offset_of!(lua_State, top) as i32)),
                            );
                            (*self.build).sub(x5, x5, r_base());
                            (*self.build).sub(
                                x5,
                                x5,
                                (((vm_reg_op((*get_op_mut(inst, 1))) + 1)
                                    * (core::mem::size_of::<TValue>() as i32))
                                    as u16),
                            );
                            (*self.build).lsr(x5, x5, kTValueSizeLog2);
                        } else {
                            (*self.build).mov(w5, self.int_op((*get_op_mut(inst, 5))));
                        }

                        (*self.build).ldr(
                            x6,
                            mem(
                                rNativeContext,
                                (core::mem::offset_of!(NativeContext, luauF_table) as i32)
                                    + (self.uint_op((*get_op_mut(inst, 0))) as i32)
                                        * (core::mem::size_of::<luau_FastFunction>() as i32),
                            ),
                        );
                        (*self.build).blr(x6);

                        inst.reg_a64 = self.regs.take_reg(w0, index);
                        // Skipping high register bits clear, only consumer is CHECK_FASTCALL_RES which doesn't read them
                    }
                }
                IrCmd::CHECK_FASTCALL_RES => {
                    (*self.build).cmp(self.reg_op((*get_op_mut(inst, 0))), ((0) as u16));
                    (*self.build).b_condition_a_64_label(
                        ConditionA64::Less,
                        self.label_op((*get_op_mut(inst, 1))),
                    );
                }
                IrCmd::DO_ARITH => {
                    self.regs
                        .spill_u32_initializer_list_register_a_64(index, &[]);
                    (*self.build).mov(x0, rState);
                    (*self.build).add(
                        x1,
                        r_base(),
                        ((vm_reg_op((*get_op_mut(inst, 0)))
                            * (core::mem::size_of::<TValue>() as i32))
                            as u16),
                    );

                    if (*get_op_mut(inst, 1)).kind() == IrOpKind::VmConst {
                        emit_add_offset(
                            &mut *self.build,
                            x2,
                            r_constants(),
                            vm_const_op((*get_op_mut(inst, 1)))
                                * (core::mem::size_of::<TValue>() as i32),
                        );
                    } else {
                        (*self.build).add(
                            x2,
                            r_base(),
                            ((vm_reg_op((*get_op_mut(inst, 1)))
                                * (core::mem::size_of::<TValue>() as i32))
                                as u16),
                        );
                    }

                    if (*get_op_mut(inst, 2)).kind() == IrOpKind::VmConst {
                        emit_add_offset(
                            &mut *self.build,
                            x3,
                            r_constants(),
                            vm_const_op((*get_op_mut(inst, 2)))
                                * (core::mem::size_of::<TValue>() as i32),
                        );
                    } else {
                        (*self.build).add(
                            x3,
                            r_base(),
                            ((vm_reg_op((*get_op_mut(inst, 2)))
                                * (core::mem::size_of::<TValue>() as i32))
                                as u16),
                        );
                    }

                    match self.int_op((*get_op_mut(inst, 3))) as u32 {
                        x if x == TMS::TM_ADD as u32 => (*self.build).ldr(
                            x4,
                            mem(
                                rNativeContext,
                                (core::mem::offset_of!(NativeContext, luaV_doarithadd) as i32),
                            ),
                        ),
                        x if x == TMS::TM_SUB as u32 => (*self.build).ldr(
                            x4,
                            mem(
                                rNativeContext,
                                (core::mem::offset_of!(NativeContext, luaV_doarithsub) as i32),
                            ),
                        ),
                        x if x == TMS::TM_MUL as u32 => (*self.build).ldr(
                            x4,
                            mem(
                                rNativeContext,
                                (core::mem::offset_of!(NativeContext, luaV_doarithmul) as i32),
                            ),
                        ),
                        x if x == TMS::TM_DIV as u32 => (*self.build).ldr(
                            x4,
                            mem(
                                rNativeContext,
                                (core::mem::offset_of!(NativeContext, luaV_doarithdiv) as i32),
                            ),
                        ),
                        x if x == TMS::TM_IDIV as u32 => (*self.build).ldr(
                            x4,
                            mem(
                                rNativeContext,
                                (core::mem::offset_of!(NativeContext, luaV_doarithidiv) as i32),
                            ),
                        ),
                        x if x == TMS::TM_MOD as u32 => (*self.build).ldr(
                            x4,
                            mem(
                                rNativeContext,
                                (core::mem::offset_of!(NativeContext, luaV_doarithmod) as i32),
                            ),
                        ),
                        x if x == TMS::TM_POW as u32 => (*self.build).ldr(
                            x4,
                            mem(
                                rNativeContext,
                                (core::mem::offset_of!(NativeContext, luaV_doarithpow) as i32),
                            ),
                        ),
                        x if x == TMS::TM_UNM as u32 => (*self.build).ldr(
                            x4,
                            mem(
                                rNativeContext,
                                (core::mem::offset_of!(NativeContext, luaV_doarithunm) as i32),
                            ),
                        ),
                        _ => CODEGEN_ASSERT!(false, "Invalid doarith helper operation tag"),
                    }

                    (*self.build).blr(x4);

                    emit_update_base(&mut *self.build);
                }
                IrCmd::DO_LEN => {
                    self.regs
                        .spill_u32_initializer_list_register_a_64(index, &[]);
                    (*self.build).mov(x0, rState);
                    (*self.build).add(
                        x1,
                        r_base(),
                        ((vm_reg_op((*get_op_mut(inst, 0)))
                            * (core::mem::size_of::<TValue>() as i32))
                            as u16),
                    );
                    (*self.build).add(
                        x2,
                        r_base(),
                        ((vm_reg_op((*get_op_mut(inst, 1)))
                            * (core::mem::size_of::<TValue>() as i32))
                            as u16),
                    );
                    (*self.build).ldr(
                        x3,
                        mem(
                            rNativeContext,
                            (core::mem::offset_of!(NativeContext, luaV_dolen) as i32),
                        ),
                    );
                    (*self.build).blr(x3);

                    emit_update_base(&mut *self.build);
                }
                IrCmd::GET_TABLE => {
                    self.regs
                        .spill_u32_initializer_list_register_a_64(index, &[]);
                    (*self.build).mov(x0, rState);
                    (*self.build).add(
                        x1,
                        r_base(),
                        ((vm_reg_op((*get_op_mut(inst, 1)))
                            * (core::mem::size_of::<TValue>() as i32))
                            as u16),
                    );

                    if (*get_op_mut(inst, 2)).kind() == IrOpKind::VmReg {
                        (*self.build).add(
                            x2,
                            r_base(),
                            ((vm_reg_op((*get_op_mut(inst, 2)))
                                * (core::mem::size_of::<TValue>() as i32))
                                as u16),
                        );
                    } else if (*get_op_mut(inst, 2)).kind() == IrOpKind::Constant {
                        let mut n = TValue::default();
                        setnvalue!(
                            &mut n as *mut TValue,
                            self.uint_op((*get_op_mut(inst, 2))) as f64
                        );
                        (*self.build).adr_register_a_64_void_usize(
                            x2,
                            &n as *const TValue as *const core::ffi::c_void,
                            core::mem::size_of::<TValue>(),
                        );
                    } else {
                        CODEGEN_ASSERT!(false, "Unsupported instruction form");
                    }

                    (*self.build).add(
                        x3,
                        r_base(),
                        ((vm_reg_op((*get_op_mut(inst, 0)))
                            * (core::mem::size_of::<TValue>() as i32))
                            as u16),
                    );
                    (*self.build).ldr(
                        x4,
                        mem(
                            rNativeContext,
                            (core::mem::offset_of!(NativeContext, luaV_gettable) as i32),
                        ),
                    );
                    (*self.build).blr(x4);

                    emit_update_base(&mut *self.build);
                }
                IrCmd::SET_TABLE => {
                    self.regs
                        .spill_u32_initializer_list_register_a_64(index, &[]);
                    (*self.build).mov(x0, rState);
                    (*self.build).add(
                        x1,
                        r_base(),
                        ((vm_reg_op((*get_op_mut(inst, 1)))
                            * (core::mem::size_of::<TValue>() as i32))
                            as u16),
                    );

                    if (*get_op_mut(inst, 2)).kind() == IrOpKind::VmReg {
                        (*self.build).add(
                            x2,
                            r_base(),
                            ((vm_reg_op((*get_op_mut(inst, 2)))
                                * (core::mem::size_of::<TValue>() as i32))
                                as u16),
                        );
                    } else if (*get_op_mut(inst, 2)).kind() == IrOpKind::Constant {
                        let mut n = TValue::default();
                        setnvalue!(
                            &mut n as *mut TValue,
                            self.uint_op((*get_op_mut(inst, 2))) as f64
                        );
                        (*self.build).adr_register_a_64_void_usize(
                            x2,
                            &n as *const TValue as *const core::ffi::c_void,
                            core::mem::size_of::<TValue>(),
                        );
                    } else {
                        CODEGEN_ASSERT!(false, "Unsupported instruction form");
                    }

                    (*self.build).add(
                        x3,
                        r_base(),
                        ((vm_reg_op((*get_op_mut(inst, 0)))
                            * (core::mem::size_of::<TValue>() as i32))
                            as u16),
                    );
                    (*self.build).ldr(
                        x4,
                        mem(
                            rNativeContext,
                            (core::mem::offset_of!(NativeContext, luaV_settable) as i32),
                        ),
                    );
                    (*self.build).blr(x4);

                    emit_update_base(&mut *self.build);
                }
                IrCmd::GET_CACHED_IMPORT => {
                    {
                        self.regs
                            .spill_u32_initializer_list_register_a_64(index, &[]);

                        let mut skip = Label::default();
                        let mut exit = Label::default();

                        let mut tempTag = self.regs.alloc_temp(KindA64::w);

                        let mut addrConstTag = self.temp_addr(
                            (*get_op_mut(inst, 1)),
                            (core::mem::offset_of!(TValue, tt) as i32),
                        );
                        (*self.build).ldr(tempTag, addrConstTag);

                        // If the constant for the import is set, we will use it directly, otherwise we have to call an import path lookup function
                        CODEGEN_ASSERT!(LUA_TNIL == 0);
                        (*self.build).cbnz(tempTag, &mut skip);

                        {
                            (*self.build).mov(x0, rState);
                            (*self.build).add(
                                x1,
                                r_base(),
                                ((vm_reg_op((*get_op_mut(inst, 0)))
                                    * (core::mem::size_of::<TValue>() as i32))
                                    as u16),
                            );
                            (*self.build).mov(w2, self.import_op((*get_op_mut(inst, 2))));
                            (*self.build).mov(w3, self.uint_op((*get_op_mut(inst, 3))));
                            (*self.build).ldr(
                                x4,
                                mem(
                                    rNativeContext,
                                    (core::mem::offset_of!(NativeContext, getImport) as i32),
                                ),
                            );
                            (*self.build).blr(x4);

                            emit_update_base(&mut *self.build);
                            (*self.build).b(&mut exit);
                        }

                        (*self.build).set_label_label(&mut skip);

                        let mut tempTv = self.regs.alloc_temp(KindA64::q);

                        let mut addrConst = self.temp_addr((*get_op_mut(inst, 1)), 0);
                        (*self.build).ldr(tempTv, addrConst);

                        let mut addrReg = self.temp_addr((*get_op_mut(inst, 0)), 0);
                        (*self.build).str(tempTv, addrReg);

                        (*self.build).set_label_label(&mut exit);
                    }
                }
                IrCmd::CONCAT => {
                    self.regs
                        .spill_u32_initializer_list_register_a_64(index, &[]);
                    (*self.build).mov(x0, rState);
                    (*self.build).mov(w1, self.uint_op((*get_op_mut(inst, 1))));
                    (*self.build).mov(
                        w2,
                        vm_reg_op((*get_op_mut(inst, 0)))
                            + self.uint_op((*get_op_mut(inst, 1))) as i32
                            - 1,
                    );
                    (*self.build).ldr(
                        x3,
                        mem(
                            rNativeContext,
                            (core::mem::offset_of!(NativeContext, luaV_concat) as i32),
                        ),
                    );
                    (*self.build).blr(x3);

                    emit_update_base(&mut *self.build);
                }
                IrCmd::GET_UPVALUE => {
                    {
                        inst.reg_a64 = self.regs.alloc_reg(KindA64::q, index);

                        let mut temp1 = self.regs.alloc_temp(KindA64::x);
                        let mut temp2 = self.regs.alloc_temp(KindA64::w);

                        (*self.build).add(
                            temp1,
                            rClosure,
                            (((K_CLOSURE_L_UPREFS_OFFSET as i32)
                                + (core::mem::size_of::<TValue>() as i32)
                                    * vm_upvalue_op((*get_op_mut(inst, 0))))
                                as u16),
                        );

                        // uprefs[] is either an actual value, or it points to UpVal object which has a pointer to value
                        let mut skip = Label::default();
                        (*self.build).ldr(
                            temp2,
                            mem(temp1, (core::mem::offset_of!(TValue, tt) as i32)),
                        );
                        (*self.build).cmp(temp2, ((LUA_TUPVAL) as u16));
                        (*self.build).b_condition_a_64_label(ConditionA64::NotEqual, &mut skip);

                        // UpVal.v points to the value (either on stack, or on heap inside each UpVal, but we can deref it unconditionally)
                        (*self.build).ldr(temp1, mem(temp1, (K_TVALUE_VALUE_GC_OFFSET as i32)));
                        (*self.build)
                            .ldr(temp1, mem(temp1, (core::mem::offset_of!(UpVal, v) as i32)));

                        (*self.build).set_label_label(&mut skip);

                        (*self.build).ldr(inst.reg_a64, mem(temp1, 0));
                    }
                }
                IrCmd::SET_UPVALUE => {
                    {
                        let mut temp1 = self.regs.alloc_temp(KindA64::x);
                        let mut temp2 = self.regs.alloc_temp(KindA64::x);

                        // UpVal*
                        (*self.build).ldr(
                            temp1,
                            mem(
                                rClosure,
                                (K_CLOSURE_L_UPREFS_OFFSET as i32)
                                    + (core::mem::size_of::<TValue>() as i32)
                                        * vm_upvalue_op((*get_op_mut(inst, 0)))
                                    + (K_TVALUE_VALUE_GC_OFFSET as i32),
                            ),
                        );

                        (*self.build)
                            .ldr(temp2, mem(temp1, (core::mem::offset_of!(UpVal, v) as i32)));
                        (*self.build).str(self.reg_op((*get_op_mut(inst, 1))), mem(temp2, 0));

                        if (*get_op_mut(inst, 2)).kind() == IrOpKind::Undef
                            || is_gco(self.tag_op((*get_op_mut(inst, 2))))
                        {
                            let mut value = self.reg_op((*get_op_mut(inst, 1)));

                            let mut skip = Label::default();
                            self.check_object_barrier_conditions(
                                temp1,
                                temp2,
                                value,
                                (*get_op_mut(inst, 1)),
                                if (*get_op_mut(inst, 2)).kind() == IrOpKind::Undef {
                                    -1
                                } else {
                                    self.tag_op((*get_op_mut(inst, 2))) as i32
                                },
                                &mut skip,
                            );

                            let mut spills = self
                                .regs
                                .spill_u32_initializer_list_register_a_64(index, &[temp1, value]);

                            (*self.build).mov(x1, temp1);
                            (*self.build).mov(x0, rState);
                            (*self.build).fmov(x2, cast_reg(KindA64::d, value));
                            (*self.build).ldr(
                                x3,
                                mem(
                                    rNativeContext,
                                    (core::mem::offset_of!(NativeContext, luaC_barrierf) as i32),
                                ),
                            );
                            (*self.build).blr(x3);

                            self.regs.restore_usize(spills); // need to restore before skip so that registers are in a consistent state

                            // note: no emitUpdateBase necessary because luaC_ barriers do not reallocate stack
                            (*self.build).set_label_label(&mut skip);
                        }
                    }
                }
                IrCmd::CHECK_TAG => {
                    {
                        let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
                        let fail = self.ir_lowering_a_64_get_target_label(
                            (*get_op_mut(inst, 2)),
                            index,
                            &mut fresh,
                        ) as *mut Label;

                        if self.tag_op((*get_op_mut(inst, 1))) == 0 {
                            let reg = self.reg_op((*get_op_mut(inst, 0)));
                            (*self.build).cbnz(reg, &mut *fail);
                        } else {
                            let reg = self.reg_op((*get_op_mut(inst, 0)));
                            (*self.build).cmp(reg, ((self.tag_op((*get_op_mut(inst, 1)))) as u16));
                            (*self.build)
                                .b_condition_a_64_label(ConditionA64::NotEqual, &mut *fail);
                        }

                        self.ir_lowering_a_64_finalize_target_label(
                            (*get_op_mut(inst, 2)),
                            index,
                            &mut fresh,
                        );
                    }
                }
                IrCmd::CHECK_TRUTHY => {
                    {
                        // Constant tags which don't require boolean value check should've been removed in constant folding
                        CODEGEN_ASSERT!(
                            (*get_op_mut(inst, 0)).kind() != IrOpKind::Constant
                                || self.tag_op((*get_op_mut(inst, 0))) == LUA_TBOOLEAN
                        );

                        let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
                        let target = self.ir_lowering_a_64_get_target_label(
                            (*get_op_mut(inst, 2)),
                            index,
                            &mut fresh,
                        ) as *mut Label;

                        let mut skip = Label::default();

                        if (*get_op_mut(inst, 0)).kind() != IrOpKind::Constant {
                            // fail to fallback on 'nil' (falsy)
                            CODEGEN_ASSERT!(LUA_TNIL == 0);
                            let tag = self.reg_op((*get_op_mut(inst, 0)));
                            (*self.build).cbz(tag, &mut *target);

                            // skip value test if it's not a boolean (truthy)
                            let tag = self.reg_op((*get_op_mut(inst, 0)));
                            (*self.build).cmp(tag, ((LUA_TBOOLEAN) as u16));
                            (*self.build).b_condition_a_64_label(ConditionA64::NotEqual, &mut skip);
                        }

                        // fail to fallback on 'false' boolean value (falsy)
                        if (*get_op_mut(inst, 1)).kind() != IrOpKind::Constant {
                            let value = self.reg_op((*get_op_mut(inst, 1)));
                            (*self.build).cbz(value, &mut *target);
                        } else {
                            if self.int_op((*get_op_mut(inst, 1))) == 0 {
                                (*self.build).b(&mut *target);
                            }
                        }

                        if (*get_op_mut(inst, 0)).kind() != IrOpKind::Constant {
                            (*self.build).set_label_label(&mut skip);
                        }

                        self.ir_lowering_a_64_finalize_target_label(
                            (*get_op_mut(inst, 2)),
                            index,
                            &mut fresh,
                        );
                    }
                }
                IrCmd::CHECK_READONLY => {
                    {
                        let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
                        let mut temp = self.regs.alloc_temp(KindA64::w);
                        (*self.build).ldrb(
                            temp,
                            mem(
                                self.reg_op((*get_op_mut(inst, 0))),
                                (core::mem::offset_of!(LuaTable, readonly) as i32),
                            ),
                        );
                        let target = self.ir_lowering_a_64_get_target_label(
                            (*get_op_mut(inst, 1)),
                            index,
                            &mut fresh,
                        ) as *mut Label;
                        (*self.build).cbnz(temp, &mut *target);
                        self.ir_lowering_a_64_finalize_target_label(
                            (*get_op_mut(inst, 1)),
                            index,
                            &mut fresh,
                        );
                    }
                }
                IrCmd::CHECK_NO_METATABLE => {
                    {
                        let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
                        let mut temp = self.regs.alloc_temp(KindA64::x);
                        (*self.build).ldr(
                            temp,
                            mem(
                                self.reg_op((*get_op_mut(inst, 0))),
                                (core::mem::offset_of!(LuaTable, metatable) as i32),
                            ),
                        );
                        let target = self.ir_lowering_a_64_get_target_label(
                            (*get_op_mut(inst, 1)),
                            index,
                            &mut fresh,
                        ) as *mut Label;
                        (*self.build).cbnz(temp, &mut *target);
                        self.ir_lowering_a_64_finalize_target_label(
                            (*get_op_mut(inst, 1)),
                            index,
                            &mut fresh,
                        );
                    }
                }
                IrCmd::CHECK_SAFE_ENV => {
                    self.check_safe_env((*get_op_mut(inst, 0)), index, next);
                }
                IrCmd::CHECK_ARRAY_SIZE => {
                    {
                        let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
                        let fail = self.ir_lowering_a_64_get_target_label(
                            (*get_op_mut(inst, 2)),
                            index,
                            &mut fresh,
                        ) as *mut Label;

                        let mut temp = self.regs.alloc_temp(KindA64::w);
                        (*self.build).ldr(
                            temp,
                            mem(
                                self.reg_op((*get_op_mut(inst, 0))),
                                (core::mem::offset_of!(LuaTable, sizearray) as i32),
                            ),
                        );

                        if (*get_op_mut(inst, 1)).kind() == IrOpKind::Inst {
                            (*self.build).cmp(temp, self.reg_op((*get_op_mut(inst, 1))));
                            (*self.build).b_condition_a_64_label(
                                ConditionA64::UnsignedLessEqual,
                                &mut *fail,
                            );
                        } else if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant {
                            if self.int_op((*get_op_mut(inst, 1))) == 0 {
                                (*self.build).cbz(temp, &mut *fail);
                            } else if ((self.int_op((*get_op_mut(inst, 1)))) as usize)
                                <= K_MAX_IMMEDIATE as usize
                            {
                                (*self.build)
                                    .cmp(temp, ((self.int_op((*get_op_mut(inst, 1)))) as u16));
                                (*self.build).b_condition_a_64_label(
                                    ConditionA64::UnsignedLessEqual,
                                    &mut *fail,
                                );
                            } else {
                                let mut temp2 = self.regs.alloc_temp(KindA64::w);
                                (*self.build).mov(temp2, self.int_op((*get_op_mut(inst, 1))));
                                (*self.build).cmp(temp, temp2);
                                (*self.build).b_condition_a_64_label(
                                    ConditionA64::UnsignedLessEqual,
                                    &mut *fail,
                                );
                            }
                        } else {
                            CODEGEN_ASSERT!(false, "Unsupported instruction form");
                        }

                        self.ir_lowering_a_64_finalize_target_label(
                            (*get_op_mut(inst, 2)),
                            index,
                            &mut fresh,
                        );
                    }
                }
                IrCmd::JUMP_SLOT_MATCH | IrCmd::CHECK_SLOT_MATCH => {
                    {
                        let mut abort = Label::default(); // used when guard aborts execution
                        let mut mismatchOp = if inst.cmd == IrCmd::JUMP_SLOT_MATCH {
                            (*get_op_mut(inst, 3))
                        } else {
                            (*get_op_mut(inst, 2))
                        };
                        let mismatch = if mismatchOp.kind() == IrOpKind::Undef {
                            &mut abort as *mut Label
                        } else {
                            self.label_op(mismatchOp) as *mut Label
                        };

                        let mut temp1 = self.regs.alloc_temp(KindA64::x);
                        let mut temp1w = cast_reg(KindA64::w, temp1);
                        let mut temp2 = self.regs.alloc_temp(KindA64::x);

                        CODEGEN_ASSERT!(
                            (core::mem::offset_of!(LuaNode, key) as i32)
                                == (core::mem::offset_of!(LuaNode, key) as i32)
                                && kOffsetOfTKeyTagNext >= 8
                                && kOffsetOfTKeyTagNext < 16
                        );
                        (*self.build).ldp(
                            temp1,
                            temp2,
                            mem(
                                self.reg_op((*get_op_mut(inst, 0))),
                                (core::mem::offset_of!(LuaNode, key) as i32),
                            ),
                        ); // load key.value into temp1 and key.tt (alongside other bits) into temp2
                        (*self.build).ubfx(
                            temp2,
                            temp2,
                            ((kOffsetOfTKeyTagNext - 8) * 8) as u8,
                            kTKeyTagBits as u8,
                        ); // .tt is right before .next, and 8 bytes are skipped by ldp
                        (*self.build).cmp(temp2, ((LUA_TSTRING) as u16));
                        (*self.build)
                            .b_condition_a_64_label(ConditionA64::NotEqual, &mut *mismatch);

                        let mut addr = self.temp_addr(
                            (*get_op_mut(inst, 1)),
                            (core::mem::offset_of!(TValue, value) as i32),
                        );
                        (*self.build).ldr(temp2, addr);
                        (*self.build).cmp(temp1, temp2);
                        (*self.build)
                            .b_condition_a_64_label(ConditionA64::NotEqual, &mut *mismatch);

                        (*self.build).ldr(
                            temp1w,
                            mem(
                                self.reg_op((*get_op_mut(inst, 0))),
                                ((core::mem::offset_of!(LuaNode, val)
                                    + core::mem::offset_of!(TValue, tt))
                                    as i32),
                            ),
                        );
                        CODEGEN_ASSERT!(LUA_TNIL == 0);
                        (*self.build).cbz(temp1w, &mut *mismatch);

                        if inst.cmd == IrCmd::JUMP_SLOT_MATCH {
                            self.jump_or_fallthrough_op((*get_op_mut(inst, 2)), next);
                        } else if abort.id != 0 {
                            emit_abort(&mut *self.build, &mut abort);
                        }
                    }
                }
                IrCmd::CHECK_NODE_NO_NEXT => {
                    {
                        let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
                        let mut temp = self.regs.alloc_temp(KindA64::w);

                        (*self.build).ldr(
                            temp,
                            mem(
                                self.reg_op((*get_op_mut(inst, 0))),
                                (core::mem::offset_of!(LuaNode, key) as i32) + kOffsetOfTKeyTagNext,
                            ),
                        );
                        (*self.build).lsr(temp, temp, kTKeyTagBits);
                        let target = self.ir_lowering_a_64_get_target_label(
                            (*get_op_mut(inst, 1)),
                            index,
                            &mut fresh,
                        ) as *mut Label;
                        (*self.build).cbnz(temp, &mut *target);
                        self.ir_lowering_a_64_finalize_target_label(
                            (*get_op_mut(inst, 1)),
                            index,
                            &mut fresh,
                        );
                    }
                }
                IrCmd::CHECK_NODE_VALUE => {
                    {
                        let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
                        let mut temp = self.regs.alloc_temp(KindA64::w);

                        (*self.build).ldr(
                            temp,
                            mem(
                                self.reg_op((*get_op_mut(inst, 0))),
                                ((core::mem::offset_of!(LuaNode, val)
                                    + core::mem::offset_of!(TValue, tt))
                                    as i32),
                            ),
                        );
                        CODEGEN_ASSERT!(LUA_TNIL == 0);
                        let target = self.ir_lowering_a_64_get_target_label(
                            (*get_op_mut(inst, 1)),
                            index,
                            &mut fresh,
                        ) as *mut Label;
                        (*self.build).cbz(temp, &mut *target);
                        self.ir_lowering_a_64_finalize_target_label(
                            (*get_op_mut(inst, 1)),
                            index,
                            &mut fresh,
                        );
                    }
                }
                IrCmd::CHECK_BUFFER_LEN => {
                    {
                        let mut minOffset = self.int_op((*get_op_mut(inst, 2)));
                        let mut maxOffset = self.int_op((*get_op_mut(inst, 3)));
                        CODEGEN_ASSERT!(minOffset < maxOffset);
                        CODEGEN_ASSERT!(
                            minOffset >= -((K_MAX_IMMEDIATE) as i32)
                                && minOffset <= ((K_MAX_IMMEDIATE) as i32)
                        );

                        let mut accessSize = maxOffset - minOffset;
                        CODEGEN_ASSERT!(accessSize > 0 && accessSize <= ((K_MAX_IMMEDIATE) as i32));

                        // For jumps to exit sync blocks to work, we need the same register allocation state at each potential taken branch
                        let mut regA = if FFlag::LuauCodegenVmExitSync.get()
                            && (*get_op_mut(inst, 0)).kind() == IrOpKind::Inst
                        {
                            self.reg_op((*get_op_mut(inst, 0)))
                        } else {
                            noreg
                        };
                        let mut regB = if FFlag::LuauCodegenVmExitSync.get()
                            && (*get_op_mut(inst, 1)).kind() == IrOpKind::Inst
                        {
                            self.reg_op((*get_op_mut(inst, 1)))
                        } else {
                            noreg
                        };
                        let mut regE = if FFlag::LuauCodegenVmExitSync.get()
                            && (*get_op_mut(inst, 4)).kind() != IrOpKind::Undef
                        {
                            self.reg_op((*get_op_mut(inst, 4)))
                        } else {
                            noreg
                        };
                        let mut tempW1 = if FFlag::LuauCodegenVmExitSync.get() {
                            self.regs.alloc_temp(KindA64::w)
                        } else {
                            noreg
                        };
                        let mut tempW2 = if FFlag::LuauCodegenVmExitSync.get() {
                            self.regs.alloc_temp(KindA64::w)
                        } else {
                            noreg
                        };
                        let mut tempD = if FFlag::LuauCodegenVmExitSync.get() {
                            self.regs.alloc_temp(KindA64::d)
                        } else {
                            noreg
                        };

                        // Validate that we don't allocate anything else in this multi-branch instruction lowering
                        if FFlag::LuauCodegenVmExitSync.get() {
                            self.exit_sync_inst_idx = index;
                            self.exit_sync_alloc_token = self.regs.get_alloc_token();
                        }

                        let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
                        let target = self.ir_lowering_a_64_get_target_label(
                            (*get_op_mut(inst, 5)),
                            index,
                            &mut fresh,
                        ) as *mut Label;

                        // Check if we are acting not only as a guard for the size, but as a guard that offset represents an exact integer
                        if (*get_op_mut(inst, 4)).kind() != IrOpKind::Undef {
                            CODEGEN_ASSERT!(
                                get_cmd_value_kind(
                                    (*self.function).inst_op((*get_op_mut(inst, 1))).cmd
                                ) == IrValueKind::Int
                            );
                            CODEGEN_ASSERT!(!produces_dirty_high_register_bits(
                                (*self.function).inst_op((*get_op_mut(inst, 1))).cmd
                            )); // Ensure that high register bits are cleared

                            if ((*self.build).features & Feature_JSCVT) != 0 {
                                let mut temp = if FFlag::LuauCodegenVmExitSync.get() {
                                    tempW1
                                } else {
                                    self.regs.alloc_temp(KindA64::w)
                                };

                                (*self.build).fjcvtzs(
                                    temp,
                                    if FFlag::LuauCodegenVmExitSync.get() {
                                        regE
                                    } else {
                                        self.reg_op((*get_op_mut(inst, 4)))
                                    },
                                ); // fjcvtzs sets PSTATE.Z (equal) iff conversion is exact
                                (*self.build)
                                    .b_condition_a_64_label(ConditionA64::NotEqual, &mut *target);
                            } else {
                                let mut temp = if FFlag::LuauCodegenVmExitSync.get() {
                                    tempD
                                } else {
                                    self.regs.alloc_temp(KindA64::d)
                                };

                                (*self.build).scvtf(
                                    temp,
                                    if FFlag::LuauCodegenVmExitSync.get() {
                                        regB
                                    } else {
                                        self.reg_op((*get_op_mut(inst, 1)))
                                    },
                                );
                                (*self.build).fcmp(
                                    if FFlag::LuauCodegenVmExitSync.get() {
                                        regE
                                    } else {
                                        self.reg_op((*get_op_mut(inst, 4)))
                                    },
                                    temp,
                                );
                                (*self.build)
                                    .b_condition_a_64_label(ConditionA64::NotEqual, &mut *target);
                            }
                        }

                        let mut temp = if FFlag::LuauCodegenVmExitSync.get() {
                            tempW1
                        } else {
                            self.regs.alloc_temp(KindA64::w)
                        };
                        (*self.build).ldr(
                            temp,
                            mem(
                                if FFlag::LuauCodegenVmExitSync.get() {
                                    regA
                                } else {
                                    self.reg_op((*get_op_mut(inst, 0)))
                                },
                                K_BUFFER_LEN_OFFSET,
                            ),
                        );

                        if (*get_op_mut(inst, 1)).kind() == IrOpKind::Inst {
                            CODEGEN_ASSERT!(!produces_dirty_high_register_bits(
                                (*self.function).inst_op((*get_op_mut(inst, 1))).cmd
                            )); // Ensure that high register bits are cleared

                            if accessSize == 1 && minOffset == 0 {
                                // fails if offset >= len
                                (*self.build).cmp(
                                    temp,
                                    if FFlag::LuauCodegenVmExitSync.get() {
                                        regB
                                    } else {
                                        self.reg_op((*get_op_mut(inst, 1)))
                                    },
                                );
                                (*self.build).b_condition_a_64_label(
                                    ConditionA64::UnsignedLessEqual,
                                    &mut *target,
                                );
                            } else if minOffset >= 0 && maxOffset <= ((K_MAX_IMMEDIATE) as i32) {
                                // fails if offset + size > len; we compute it as len - offset < size
                                let mut tempx = cast_reg(KindA64::x, temp);
                                (*self.build).sub(
                                    tempx,
                                    tempx,
                                    if FFlag::LuauCodegenVmExitSync.get() {
                                        regB
                                    } else {
                                        self.reg_op((*get_op_mut(inst, 1)))
                                    },
                                ); // implicit uxtw
                                (*self.build).cmp(tempx, ((maxOffset) as u16));
                                (*self.build)
                                    .b_condition_a_64_label(ConditionA64::Less, &mut *target);
                            // note: this is a signed 64-bit comparison so that out of bounds offset fails
                            } else {
                                let mut tempx = cast_reg(KindA64::x, temp);
                                let mut temp2 = if FFlag::LuauCodegenVmExitSync.get() {
                                    cast_reg(KindA64::x, tempW2)
                                } else {
                                    self.regs.alloc_temp(KindA64::x)
                                };

                                // Get the base offset in 32 bits
                                if minOffset >= 0 {
                                    (*self.build).add(
                                        cast_reg(KindA64::w, temp2),
                                        if FFlag::LuauCodegenVmExitSync.get() {
                                            regB
                                        } else {
                                            self.reg_op((*get_op_mut(inst, 1)))
                                        },
                                        ((minOffset) as u16),
                                    );
                                } else {
                                    (*self.build).sub(
                                        cast_reg(KindA64::w, temp2),
                                        if FFlag::LuauCodegenVmExitSync.get() {
                                            regB
                                        } else {
                                            self.reg_op((*get_op_mut(inst, 1)))
                                        },
                                        ((-minOffset) as u16),
                                    );
                                }

                                // fail if (((offset + minOffset) as u32)) as u64 { + accessSize > length
                                (*self.build).add(temp2, temp2, ((accessSize) as u16));
                                (*self.build).cmp(temp2, tempx);
                                (*self.build).b_condition_a_64_label(
                                    ConditionA64::UnsignedGreater,
                                    &mut *target,
                                );
                            }
                        } else if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant {
                            let mut offset = self.int_op((*get_op_mut(inst, 1)));
                            let mut endOffset = if FFlag::LuauCodegenFixBufferLenCheck.get() {
                                maxOffset
                            } else {
                                accessSize
                            };
                            let mut failCond = if FFlag::LuauCodegenFixBufferLenCheck.get() {
                                ConditionA64::UnsignedLess
                            } else {
                                ConditionA64::UnsignedLessEqual
                            };

                            // Constant folding can take care of it, but for safety we avoid overflow/underflow cases here
                            if offset < 0
                                || ((offset) as u32) + ((endOffset) as u32) >= ((INT_MAX) as u32)
                            {
                                (*self.build).b(&mut *target);
                            } else if offset + endOffset <= ((K_MAX_IMMEDIATE) as i32) {
                                (*self.build).cmp(temp, ((offset + endOffset) as u16));
                                (*self.build).b_condition_a_64_label(failCond, &mut *target);
                            } else {
                                let mut temp2 = if FFlag::LuauCodegenVmExitSync.get() {
                                    tempW2
                                } else {
                                    self.regs.alloc_temp(KindA64::w)
                                };
                                (*self.build).mov(temp2, offset + endOffset);
                                (*self.build).cmp(temp, temp2);
                                (*self.build).b_condition_a_64_label(failCond, &mut *target);
                            }
                        } else {
                            CODEGEN_ASSERT!(false, "Unsupported instruction form");
                        }
                        self.ir_lowering_a_64_finalize_target_label(
                            (*get_op_mut(inst, 5)),
                            index,
                            &mut fresh,
                        );
                    }
                }
                IrCmd::CHECK_USERDATA_TAG => {
                    {
                        CODEGEN_ASSERT!(
                            ((self.int_op((*get_op_mut(inst, 1)))) as u32) <= K_MAX_IMMEDIATE
                        );

                        let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
                        let fail = self.ir_lowering_a_64_get_target_label(
                            (*get_op_mut(inst, 2)),
                            index,
                            &mut fresh,
                        ) as *mut Label;
                        let mut temp = self.regs.alloc_temp(KindA64::w);
                        (*self.build).ldrb(
                            temp,
                            mem(
                                self.reg_op((*get_op_mut(inst, 0))),
                                (core::mem::offset_of!(Udata, tag) as i32),
                            ),
                        );
                        (*self.build).cmp(temp, ((self.int_op((*get_op_mut(inst, 1)))) as u16));
                        (*self.build).b_condition_a_64_label(ConditionA64::NotEqual, &mut *fail);
                        self.ir_lowering_a_64_finalize_target_label(
                            (*get_op_mut(inst, 2)),
                            index,
                            &mut fresh,
                        );
                    }
                }
                IrCmd::CHECK_CMP_NUM => {
                    {
                        let mut cond = condition_op((*get_op_mut(inst, 2)));
                        let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
                        let fail = self.ir_lowering_a_64_get_target_label(
                            (*get_op_mut(inst, 3)),
                            index,
                            &mut fresh,
                        ) as *mut Label;

                        let mut tempA = self.temp_double((*get_op_mut(inst, 0)));

                        (*self.build).fcmp(tempA, self.temp_double((*get_op_mut(inst, 1))));
                        (*self.build).b_condition_a_64_label(
                            get_condition_f_p(get_negated_condition(cond)),
                            &mut *fail,
                        );

                        self.ir_lowering_a_64_finalize_target_label(
                            (*get_op_mut(inst, 3)),
                            index,
                            &mut fresh,
                        );
                    }
                }
                IrCmd::CHECK_CMP_INT => {
                    {
                        let mut cond = condition_op((*get_op_mut(inst, 2)));

                        let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
                        let fail = self.ir_lowering_a_64_get_target_label(
                            (*get_op_mut(inst, 3)),
                            index,
                            &mut fresh,
                        ) as *mut Label;

                        if cond == IrCondition::Equal && self.int_op((*get_op_mut(inst, 1))) == 0 {
                            let reg = self.reg_op((*get_op_mut(inst, 0)));
                            (*self.build).cbnz(reg, &mut *fail);
                        } else if cond == IrCondition::NotEqual
                            && self.int_op((*get_op_mut(inst, 1))) == 0
                        {
                            let reg = self.reg_op((*get_op_mut(inst, 0)));
                            (*self.build).cbz(reg, &mut *fail);
                        } else {
                            let mut tempA = self.temp_int((*get_op_mut(inst, 0)));

                            if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
                                && ((self.int_op((*get_op_mut(inst, 1)))) as u32) <= K_MAX_IMMEDIATE
                            {
                                (*self.build)
                                    .cmp(tempA, ((self.int_op((*get_op_mut(inst, 1)))) as u16));
                            } else {
                                (*self.build).cmp(tempA, self.temp_int((*get_op_mut(inst, 1))));
                            }

                            (*self.build).b_condition_a_64_label(
                                get_condition_int(get_negated_condition(cond)),
                                &mut *fail,
                            );
                        }
                        self.ir_lowering_a_64_finalize_target_label(
                            (*get_op_mut(inst, 3)),
                            index,
                            &mut fresh,
                        );
                    }
                }
                IrCmd::CHECK_CMP_INT64 => {
                    {
                        let mut cond = condition_op((*get_op_mut(inst, 2)));

                        let mut fresh = Label::default(); // used when guard aborts execution or jumps to a VM exit
                        let fail = self.ir_lowering_a_64_get_target_label(
                            (*get_op_mut(inst, 3)),
                            index,
                            &mut fresh,
                        ) as *mut Label;

                        if cond == IrCondition::Equal
                            && (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
                            && self.int_64_op((*get_op_mut(inst, 1))) == 0
                        {
                            let reg = self.reg_op((*get_op_mut(inst, 0)));
                            (*self.build).cbnz(reg, &mut *fail);
                        } else if cond == IrCondition::NotEqual
                            && (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
                            && self.int_64_op((*get_op_mut(inst, 1))) == 0
                        {
                            let reg = self.reg_op((*get_op_mut(inst, 0)));
                            (*self.build).cbz(reg, &mut *fail);
                        } else {
                            let mut tempA = self.temp_int64((*get_op_mut(inst, 0)));

                            if (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
                                && ((self.int_64_op((*get_op_mut(inst, 1)))) as u64)
                                    <= K_MAX_IMMEDIATE as u64
                            {
                                (*self.build)
                                    .cmp(tempA, ((self.int_64_op((*get_op_mut(inst, 1)))) as u16));
                            } else {
                                (*self.build).cmp(tempA, self.temp_int64((*get_op_mut(inst, 1))));
                            }

                            (*self.build).b_condition_a_64_label(
                                get_condition_int64(get_negated_condition(cond)),
                                &mut *fail,
                            );
                        }
                        self.ir_lowering_a_64_finalize_target_label(
                            (*get_op_mut(inst, 3)),
                            index,
                            &mut fresh,
                        );
                    }
                }
                IrCmd::INTERRUPT => {
                    self.regs
                        .spill_u32_initializer_list_register_a_64(index, &[]);

                    let mut self_ = Label::default();

                    (*self.build).ldr(
                        x0,
                        mem(
                            rGlobalState,
                            (core::mem::offset_of!(global_State, cb.interrupt) as i32),
                        ),
                    );
                    (*self.build).cbnz(x0, &mut self_);

                    let mut next = (*self.build).set_label();

                    self.interrupt_handlers.push(InterruptHandler {
                        self_,
                        pcpos: self.uint_op((*get_op_mut(inst, 0))),
                        next,
                    });
                }
                IrCmd::CHECK_GC => {
                    {
                        let mut temp1 = self.regs.alloc_temp(KindA64::x);
                        let mut temp2 = self.regs.alloc_temp(KindA64::x);

                        CODEGEN_ASSERT!(
                            (core::mem::offset_of!(global_State, totalbytes) as i32)
                                == (core::mem::offset_of!(global_State, GCthreshold) as i32)
                                    + (core::mem::size_of::<usize>() as i32)
                        );
                        let mut skip = Label::default();
                        (*self.build).ldp(
                            temp1,
                            temp2,
                            mem(
                                rGlobalState,
                                (core::mem::offset_of!(global_State, GCthreshold) as i32),
                            ),
                        );
                        (*self.build).cmp(temp1, temp2);
                        (*self.build)
                            .b_condition_a_64_label(ConditionA64::UnsignedGreater, &mut skip);

                        let mut spills = self
                            .regs
                            .spill_u32_initializer_list_register_a_64(index, &[]);

                        (*self.build).mov(x0, rState);
                        (*self.build).mov(w1, 1);
                        (*self.build).ldr(
                            x2,
                            mem(
                                rNativeContext,
                                (core::mem::offset_of!(NativeContext, luaC_step) as i32),
                            ),
                        );
                        (*self.build).blr(x2);

                        emit_update_base(&mut *self.build);

                        self.regs.restore_usize(spills); // need to restore before skip so that registers are in a consistent state

                        (*self.build).set_label_label(&mut skip);
                    }
                }
                IrCmd::BARRIER_OBJ => {
                    {
                        let mut temp = self.regs.alloc_temp(KindA64::x);

                        let mut skip = Label::default();
                        let object = self.reg_op((*get_op_mut(inst, 0)));
                        let ratag = if (*get_op_mut(inst, 2)).kind() == IrOpKind::Undef {
                            -1
                        } else {
                            self.tag_op((*get_op_mut(inst, 2))) as i32
                        };
                        self.check_object_barrier_conditions(
                            object,
                            temp,
                            noreg,
                            (*get_op_mut(inst, 1)),
                            ratag,
                            &mut skip,
                        );

                        let mut reg = self.reg_op((*get_op_mut(inst, 0))); // note: we need to call regOp before spill so that we don't do redundant reloads
                        let mut spills = self
                            .regs
                            .spill_u32_initializer_list_register_a_64(index, &[reg]);
                        (*self.build).mov(x1, reg);
                        (*self.build).mov(x0, rState);
                        (*self.build).ldr(
                            x2,
                            mem(
                                r_base(),
                                vm_reg_op((*get_op_mut(inst, 1)))
                                    * (core::mem::size_of::<TValue>() as i32)
                                    + (core::mem::offset_of!(TValue, value) as i32),
                            ),
                        );
                        (*self.build).ldr(
                            x3,
                            mem(
                                rNativeContext,
                                (core::mem::offset_of!(NativeContext, luaC_barrierf) as i32),
                            ),
                        );
                        (*self.build).blr(x3);

                        self.regs.restore_usize(spills); // need to restore before skip so that registers are in a consistent state

                        // note: no emitUpdateBase necessary because luaC_ barriers do not reallocate stack
                        (*self.build).set_label_label(&mut skip);
                    }
                }
                IrCmd::BARRIER_TABLE_BACK => {
                    {
                        let mut skip = Label::default();
                        let mut temp = self.regs.alloc_temp(KindA64::w);

                        // isblack(obj2gco(t))
                        (*self.build).ldrb(
                            temp,
                            mem(
                                self.reg_op((*get_op_mut(inst, 0))),
                                (core::mem::offset_of!(GCheader, marked) as i32),
                            ),
                        );
                        (*self.build).tbz(temp, BLACKBIT, &mut skip);

                        let mut reg = self.reg_op((*get_op_mut(inst, 0))); // note: we need to call regOp before spill so that we don't do redundant reloads
                        let mut spills = self
                            .regs
                            .spill_u32_initializer_list_register_a_64(index, &[reg]);
                        (*self.build).mov(x1, reg);
                        (*self.build).mov(x0, rState);
                        (*self.build).add(
                            x2,
                            x1,
                            ((core::mem::offset_of!(LuaTable, gclist) as i32) as u16),
                        );
                        (*self.build).ldr(
                            x3,
                            mem(
                                rNativeContext,
                                (core::mem::offset_of!(NativeContext, luaC_barrierback) as i32),
                            ),
                        );
                        (*self.build).blr(x3);

                        self.regs.restore_usize(spills); // need to restore before skip so that registers are in a consistent state

                        // note: no emitUpdateBase necessary because luaC_ barriers do not reallocate stack
                        (*self.build).set_label_label(&mut skip);
                    }
                }
                IrCmd::BARRIER_TABLE_FORWARD => {
                    {
                        let mut temp = self.regs.alloc_temp(KindA64::x);

                        let mut skip = Label::default();
                        let object = self.reg_op((*get_op_mut(inst, 0)));
                        let ratag = if (*get_op_mut(inst, 2)).kind() == IrOpKind::Undef {
                            -1
                        } else {
                            self.tag_op((*get_op_mut(inst, 2))) as i32
                        };
                        self.check_object_barrier_conditions(
                            object,
                            temp,
                            noreg,
                            (*get_op_mut(inst, 1)),
                            ratag,
                            &mut skip,
                        );

                        let mut reg = self.reg_op((*get_op_mut(inst, 0))); // note: we need to call regOp before spill so that we don't do redundant reloads
                        let mut addr = self.temp_addr(
                            (*get_op_mut(inst, 1)),
                            (core::mem::offset_of!(TValue, value) as i32),
                        );
                        let mut spills = self
                            .regs
                            .spill_u32_initializer_list_register_a_64(index, &[reg]);
                        (*self.build).mov(x1, reg);
                        (*self.build).mov(x0, rState);
                        (*self.build).ldr(x2, addr);
                        (*self.build).ldr(
                            x3,
                            mem(
                                rNativeContext,
                                (core::mem::offset_of!(NativeContext, luaC_barriertable) as i32),
                            ),
                        );
                        (*self.build).blr(x3);

                        self.regs.restore_usize(spills); // need to restore before skip so that registers are in a consistent state

                        // note: no emitUpdateBase necessary because luaC_ barriers do not reallocate stack
                        (*self.build).set_label_label(&mut skip);
                    }
                }
                IrCmd::SET_SAVEDPC => {
                    let mut temp1 = self.regs.alloc_temp(KindA64::x);
                    let mut temp2 = self.regs.alloc_temp(KindA64::x);

                    emit_add_offset(
                        &mut *self.build,
                        temp1,
                        rCode,
                        (self.uint_op((*get_op_mut(inst, 0))) as i32)
                            * (core::mem::size_of::<Instruction>() as i32),
                    );
                    (*self.build).ldr(
                        temp2,
                        mem(rState, (core::mem::offset_of!(lua_State, ci) as i32)),
                    );
                    (*self.build).str(
                        temp1,
                        mem(temp2, (core::mem::offset_of!(CallInfo, savedpc) as i32)),
                    );
                }
                IrCmd::CLOSE_UPVALS => {
                    {
                        let mut skip = Label::default();
                        let mut temp1 = self.regs.alloc_temp(KindA64::x);
                        let mut temp2 = self.regs.alloc_temp(KindA64::x);

                        // L->openupval != 0
                        (*self.build).ldr(
                            temp1,
                            mem(rState, (core::mem::offset_of!(lua_State, openupval) as i32)),
                        );
                        (*self.build).cbz(temp1, &mut skip);

                        // ra <= L->openupval->v
                        (*self.build)
                            .ldr(temp1, mem(temp1, (core::mem::offset_of!(UpVal, v) as i32)));
                        (*self.build).add(
                            temp2,
                            r_base(),
                            ((vm_reg_op((*get_op_mut(inst, 0)))
                                * (core::mem::size_of::<TValue>() as i32))
                                as u16),
                        );
                        (*self.build).cmp(temp2, temp1);
                        (*self.build)
                            .b_condition_a_64_label(ConditionA64::UnsignedGreater, &mut skip);

                        let mut spills = self
                            .regs
                            .spill_u32_initializer_list_register_a_64(index, &[temp2]);
                        (*self.build).mov(x1, temp2);
                        (*self.build).mov(x0, rState);
                        (*self.build).ldr(
                            x2,
                            mem(
                                rNativeContext,
                                (core::mem::offset_of!(NativeContext, luaF_close) as i32),
                            ),
                        );
                        (*self.build).blr(x2);

                        self.regs.restore_usize(spills); // need to restore before skip so that registers are in a consistent state

                        (*self.build).set_label_label(&mut skip);
                    }
                }
                IrCmd::CAPTURE => {
                    // no-op
                }
                IrCmd::SETLIST => {
                    self.regs
                        .spill_u32_initializer_list_register_a_64(index, &[]);
                    emit_fallback(
                        &mut *self.build,
                        (core::mem::offset_of!(NativeContext, executeSETLIST) as i32),
                        self.uint_op((*get_op_mut(inst, 0))),
                    );
                }
                IrCmd::CALL => {
                    self.regs
                        .spill_u32_initializer_list_register_a_64(index, &[]);
                    // argtop = if (nparams == LUA_MULTRET) { L->top } else { ra + 1 + nparams };
                    if self.int_op((*get_op_mut(inst, 1))) == LUA_MULTRET {
                        (*self.build).ldr(
                            x2,
                            mem(rState, (core::mem::offset_of!(lua_State, top) as i32)),
                        );
                    } else {
                        (*self.build).add(
                            x2,
                            r_base(),
                            (((vm_reg_op((*get_op_mut(inst, 0)))
                                + 1
                                + self.int_op((*get_op_mut(inst, 1))))
                                * (core::mem::size_of::<TValue>() as i32))
                                as u16),
                        );
                    }

                    // call_fallback(L, ra, argtop, nresults)
                    (*self.build).mov(x0, rState);
                    (*self.build).add(
                        x1,
                        r_base(),
                        ((vm_reg_op((*get_op_mut(inst, 0)))
                            * (core::mem::size_of::<TValue>() as i32))
                            as u16),
                    );
                    (*self.build).mov(w3, self.int_op((*get_op_mut(inst, 2))));
                    (*self.build).ldr(
                        x4,
                        mem(
                            rNativeContext,
                            (core::mem::offset_of!(NativeContext, callFallback) as i32),
                        ),
                    );
                    (*self.build).blr(x4);

                    emit_update_base(&mut *self.build);

                    // reentry with x0=closure (NULL implies C function; CALL_FALLBACK_YIELD will trigger exit)
                    (*self.build).cbnz(x0, &mut (*self.helpers).continueCall);
                }
                IrCmd::RETURN => {
                    self.regs
                        .spill_u32_initializer_list_register_a_64(index, &[]);

                    if (*self.function).variadic {
                        (*self.build).ldr(
                            x1,
                            mem(rState, (core::mem::offset_of!(lua_State, ci) as i32)),
                        );
                        (*self.build)
                            .ldr(x1, mem(x1, (core::mem::offset_of!(CallInfo, func) as i32)));
                    } else if self.int_op((*get_op_mut(inst, 1))) != 1 {
                        (*self.build).sub(
                            x1,
                            r_base(),
                            ((core::mem::size_of::<TValue>() as i32) as u16),
                        );
                    } // invariant: ci->func + 1 == ci->base for non-variadic frames

                    if self.int_op((*get_op_mut(inst, 1))) == 0 {
                        (*self.build).mov(w2, 0);
                        (*self.build).b(&mut (*self.helpers).return_);
                    } else if self.int_op((*get_op_mut(inst, 1))) == 1 && !(*self.function).variadic
                    {
                        // fast path: minimizes x1 adjustments
                        // note that we skipped x1 computation for this specific case above
                        (*self.build).ldr(
                            q0,
                            mem(
                                r_base(),
                                vm_reg_op((*get_op_mut(inst, 0)))
                                    * (core::mem::size_of::<TValue>() as i32),
                            ),
                        );
                        (*self.build).str(
                            q0,
                            mem(r_base(), -((core::mem::size_of::<TValue>() as i32) as i32)),
                        );
                        (*self.build).mov(x1, r_base());
                        (*self.build).mov(w2, 1);
                        (*self.build).b(&mut (*self.helpers).return_);
                    } else if self.int_op((*get_op_mut(inst, 1))) >= 1
                        && self.int_op((*get_op_mut(inst, 1))) <= 3
                    {
                        for r in 0..self.int_op((*get_op_mut(inst, 1))) {
                            (*self.build).ldr(
                                q0,
                                mem(
                                    r_base(),
                                    (vm_reg_op((*get_op_mut(inst, 0))) + r)
                                        * (core::mem::size_of::<TValue>() as i32),
                                ),
                            );
                            (*self.build).str(
                                q0,
                                mem_kind(
                                    x1,
                                    (core::mem::size_of::<TValue>() as i32),
                                    AddressKindA64::post,
                                ),
                            );
                        }
                        (*self.build).mov(w2, self.int_op((*get_op_mut(inst, 1))));
                        (*self.build).b(&mut (*self.helpers).return_);
                    } else {
                        (*self.build).mov(w2, 0);

                        // vali = ra
                        (*self.build).add(
                            x3,
                            r_base(),
                            ((vm_reg_op((*get_op_mut(inst, 0)))
                                * (core::mem::size_of::<TValue>() as i32))
                                as u16),
                        );

                        // valend = if (n == LUA_MULTRET) { L->top } else { ra + n
                        if self.int_op((*get_op_mut(inst, 1))) == LUA_MULTRET {
                            (*self.build).ldr(
                                x4,
                                mem(rState, (core::mem::offset_of!(lua_State, top) as i32)),
                            );
                        } else {
                            (*self.build).add(
                                x4,
                                r_base(),
                                (((vm_reg_op((*get_op_mut(inst, 0)))
                                    + self.int_op((*get_op_mut(inst, 1))))
                                    * (core::mem::size_of::<TValue>() as i32))
                                    as u16),
                            );
                        }

                        let mut repeatValueLoop = Label::default();
                        let mut exitValueLoop = Label::default();

                        if self.int_op((*get_op_mut(inst, 1))) == LUA_MULTRET {
                            (*self.build).cmp(x3, x4);
                            (*self.build)
                                .b_condition_a_64_label(ConditionA64::CarrySet, &mut exitValueLoop);
                            // CarrySet == UnsignedGreaterEqual
                        }

                        (*self.build).set_label_label(&mut repeatValueLoop);
                        (*self.build).ldr(
                            q0,
                            mem_kind(
                                x3,
                                (core::mem::size_of::<TValue>() as i32),
                                AddressKindA64::post,
                            ),
                        );
                        (*self.build).str(
                            q0,
                            mem_kind(
                                x1,
                                (core::mem::size_of::<TValue>() as i32),
                                AddressKindA64::post,
                            ),
                        );
                        (*self.build).add(w2, w2, ((1) as u16));
                        (*self.build).cmp(x3, x4);
                        (*self.build)
                            .b_condition_a_64_label(ConditionA64::CarryClear, &mut repeatValueLoop); // CarryClear == UnsignedLess

                        (*self.build).set_label_label(&mut exitValueLoop);
                        (*self.build).b(&mut (*self.helpers).return_);
                    }
                }
                IrCmd::FORGLOOP => {
                    // register layout: ra + 1 = table, ra + 2 = internal index, ra + 3 .. ra + aux = iteration variables
                    self.regs
                        .spill_u32_initializer_list_register_a_64(index, &[]);
                    // clear extra variables since we might have more than two
                    if self.int_op((*get_op_mut(inst, 1))) > 2 {
                        CODEGEN_ASSERT!(LUA_TNIL == 0);
                        for i in 2..self.int_op((*get_op_mut(inst, 1))) {
                            (*self.build).str(
                                wzr,
                                mem(
                                    r_base(),
                                    (vm_reg_op((*get_op_mut(inst, 0))) + 3 + i)
                                        * (core::mem::size_of::<TValue>() as i32)
                                        + (core::mem::offset_of!(TValue, tt) as i32),
                                ),
                            );
                        }
                    }
                    // we use full iter fallback for now; in the future it could be worthwhile to accelerate array iteration here
                    (*self.build).mov(x0, rState);
                    (*self.build).ldr(
                        x1,
                        mem(
                            r_base(),
                            (vm_reg_op((*get_op_mut(inst, 0))) + 1)
                                * (core::mem::size_of::<TValue>() as i32)
                                + (K_TVALUE_VALUE_GC_OFFSET as i32),
                        ),
                    );
                    (*self.build).ldr(
                        w2,
                        mem(
                            r_base(),
                            (vm_reg_op((*get_op_mut(inst, 0))) + 2)
                                * (core::mem::size_of::<TValue>() as i32)
                                + (K_TVALUE_VALUE_P_OFFSET as i32),
                        ),
                    );
                    (*self.build).add(
                        x3,
                        r_base(),
                        ((vm_reg_op((*get_op_mut(inst, 0)))
                            * (core::mem::size_of::<TValue>() as i32))
                            as u16),
                    );
                    (*self.build).ldr(
                        x4,
                        mem(
                            rNativeContext,
                            (core::mem::offset_of!(NativeContext, forgLoopTableIter) as i32),
                        ),
                    );
                    (*self.build).blr(x4);
                    // note: no emitUpdateBase necessary because forgLoopTableIter does not reallocate stack
                    (*self.build).cbnz(w0, self.label_op((*get_op_mut(inst, 2))));
                    self.jump_or_fallthrough_op((*get_op_mut(inst, 3)), next);
                }
                IrCmd::FORGLOOP_FALLBACK => {
                    self.regs
                        .spill_u32_initializer_list_register_a_64(index, &[]);
                    (*self.build).mov(x0, rState);
                    (*self.build).mov(w1, vm_reg_op((*get_op_mut(inst, 0))));
                    (*self.build).mov(w2, self.int_op((*get_op_mut(inst, 1))));

                    if FFlag::LuauYieldIter2.get() {
                        (*self.build).ldr(
                            x3,
                            mem(
                                rNativeContext,
                                (core::mem::offset_of!(NativeContext, forgLoopNonTableFallback)
                                    as i32),
                            ),
                        );
                        (*self.build).blr(x3);
                        emit_update_base(&mut *self.build);
                        (*self.build).cmp(w0, ((0) as u16));
                        (*self.build).b_condition_a_64_label(
                            ConditionA64::Less,
                            &mut (*self.helpers).exitNoContinueVm,
                        );
                        (*self.build).b_condition_a_64_label(
                            ConditionA64::Greater,
                            self.label_op((*get_op_mut(inst, 2))),
                        );
                    } else {
                        (*self.build).ldr(
                            x3,
                            mem(
                                rNativeContext,
                                (core::mem::offset_of!(
                                    NativeContext,
                                    forgLoopNonTableFallback_DEPRECATED
                                ) as i32),
                            ),
                        );
                        (*self.build).blr(x3);
                        emit_update_base(&mut *self.build);
                        (*self.build).cbnz(w0, self.label_op((*get_op_mut(inst, 2))));
                    }

                    self.jump_or_fallthrough_op((*get_op_mut(inst, 3)), next);
                }
                IrCmd::FORGPREP_XNEXT_FALLBACK => {
                    self.regs
                        .spill_u32_initializer_list_register_a_64(index, &[]);
                    (*self.build).mov(x0, rState);
                    (*self.build).add(
                        x1,
                        r_base(),
                        ((vm_reg_op((*get_op_mut(inst, 1)))
                            * (core::mem::size_of::<TValue>() as i32))
                            as u16),
                    );
                    (*self.build).mov(w2, self.uint_op((*get_op_mut(inst, 0))) + 1);
                    (*self.build).ldr(
                        x3,
                        mem(
                            rNativeContext,
                            (core::mem::offset_of!(NativeContext, forgPrepXnextFallback) as i32),
                        ),
                    );
                    (*self.build).blr(x3);
                    // note: no emitUpdateBase necessary because forgPrepXnextFallback does not reallocate stack
                    self.jump_or_fallthrough_op((*get_op_mut(inst, 2)), next);
                }
                IrCmd::COVERAGE => {
                    {
                        let mut temp1 = self.regs.alloc_temp(KindA64::x);
                        let mut temp2 = self.regs.alloc_temp(KindA64::w);
                        let mut temp3 = self.regs.alloc_temp(KindA64::w);

                        (*self.build).mov(
                            temp1,
                            (self.uint_op((*get_op_mut(inst, 0))) as i32)
                                * (core::mem::size_of::<Instruction>() as i32),
                        );
                        (*self.build).ldr(temp2, mem(rCode, temp1));

                        // increments E (high 24 bits); if the result overflows a 23-bit counter, high bit becomes 1
                        // note: cmp can be eliminated with adds but we aren't concerned with code size for coverage
                        (*self.build).add(temp3, temp2, ((256) as u16));
                        (*self.build).cmp(temp3, ((0) as u16));
                        (*self.build).csel(temp2, temp2, temp3, ConditionA64::Less);

                        (*self.build).str(temp2, mem(rCode, temp1));
                    }

                    // Full instruction fallbacks
                }
                IrCmd::FALLBACK_GETGLOBAL => {
                    CODEGEN_ASSERT!((*get_op_mut(inst, 1)).kind() == IrOpKind::VmReg);
                    CODEGEN_ASSERT!((*get_op_mut(inst, 2)).kind() == IrOpKind::VmConst);

                    self.regs
                        .spill_u32_initializer_list_register_a_64(index, &[]);
                    emit_fallback(
                        &mut *self.build,
                        (core::mem::offset_of!(NativeContext, executeGETGLOBAL) as i32),
                        self.uint_op((*get_op_mut(inst, 0))),
                    );
                }
                IrCmd::FALLBACK_SETGLOBAL => {
                    CODEGEN_ASSERT!((*get_op_mut(inst, 1)).kind() == IrOpKind::VmReg);
                    CODEGEN_ASSERT!((*get_op_mut(inst, 2)).kind() == IrOpKind::VmConst);

                    self.regs
                        .spill_u32_initializer_list_register_a_64(index, &[]);
                    emit_fallback(
                        &mut *self.build,
                        (core::mem::offset_of!(NativeContext, executeSETGLOBAL) as i32),
                        self.uint_op((*get_op_mut(inst, 0))),
                    );
                }
                IrCmd::FALLBACK_GETTABLEKS => {
                    CODEGEN_ASSERT!((*get_op_mut(inst, 1)).kind() == IrOpKind::VmReg);
                    CODEGEN_ASSERT!((*get_op_mut(inst, 2)).kind() == IrOpKind::VmReg);
                    CODEGEN_ASSERT!((*get_op_mut(inst, 3)).kind() == IrOpKind::VmConst);

                    self.regs
                        .spill_u32_initializer_list_register_a_64(index, &[]);
                    emit_fallback(
                        &mut *self.build,
                        (core::mem::offset_of!(NativeContext, executeGETTABLEKS) as i32),
                        self.uint_op((*get_op_mut(inst, 0))),
                    );
                }
                IrCmd::FALLBACK_SETTABLEKS => {
                    CODEGEN_ASSERT!((*get_op_mut(inst, 1)).kind() == IrOpKind::VmReg);
                    CODEGEN_ASSERT!((*get_op_mut(inst, 2)).kind() == IrOpKind::VmReg);
                    CODEGEN_ASSERT!((*get_op_mut(inst, 3)).kind() == IrOpKind::VmConst);

                    self.regs
                        .spill_u32_initializer_list_register_a_64(index, &[]);
                    emit_fallback(
                        &mut *self.build,
                        (core::mem::offset_of!(NativeContext, executeSETTABLEKS) as i32),
                        self.uint_op((*get_op_mut(inst, 0))),
                    );
                }
                IrCmd::FALLBACK_NAMECALL => {
                    CODEGEN_ASSERT!((*get_op_mut(inst, 1)).kind() == IrOpKind::VmReg);
                    CODEGEN_ASSERT!((*get_op_mut(inst, 2)).kind() == IrOpKind::VmReg);
                    CODEGEN_ASSERT!((*get_op_mut(inst, 3)).kind() == IrOpKind::VmConst);

                    self.regs
                        .spill_u32_initializer_list_register_a_64(index, &[]);
                    emit_fallback(
                        &mut *self.build,
                        (core::mem::offset_of!(NativeContext, executeNAMECALL) as i32),
                        self.uint_op((*get_op_mut(inst, 0))),
                    );
                }
                IrCmd::FALLBACK_PREPVARARGS => {
                    CODEGEN_ASSERT!((*get_op_mut(inst, 1)).kind() == IrOpKind::Constant);

                    self.regs
                        .spill_u32_initializer_list_register_a_64(index, &[]);
                    emit_fallback(
                        &mut *self.build,
                        (core::mem::offset_of!(NativeContext, executePREPVARARGS) as i32),
                        self.uint_op((*get_op_mut(inst, 0))),
                    );
                }
                IrCmd::FALLBACK_GETVARARGS => {
                    CODEGEN_ASSERT!((*get_op_mut(inst, 1)).kind() == IrOpKind::VmReg);
                    CODEGEN_ASSERT!((*get_op_mut(inst, 2)).kind() == IrOpKind::Constant);

                    self.regs
                        .spill_u32_initializer_list_register_a_64(index, &[]);
                    (*self.build).mov(x0, rState);

                    if self.int_op((*get_op_mut(inst, 2))) == LUA_MULTRET {
                        emit_add_offset(
                            &mut *self.build,
                            x1,
                            rCode,
                            (self.uint_op((*get_op_mut(inst, 0))) as i32)
                                * (core::mem::size_of::<Instruction>() as i32),
                        );
                        (*self.build).mov(x2, r_base());
                        (*self.build).mov(w3, vm_reg_op((*get_op_mut(inst, 1))));
                        (*self.build).ldr(
                            x4,
                            mem(
                                rNativeContext,
                                (core::mem::offset_of!(NativeContext, executeGETVARARGSMultRet)
                                    as i32),
                            ),
                        );
                        (*self.build).blr(x4);

                        emit_update_base(&mut *self.build);
                    } else {
                        (*self.build).mov(x1, r_base());
                        (*self.build).mov(w2, vm_reg_op((*get_op_mut(inst, 1))));
                        (*self.build).mov(w3, self.int_op((*get_op_mut(inst, 2))));
                        (*self.build).ldr(
                            x4,
                            mem(
                                rNativeContext,
                                (core::mem::offset_of!(NativeContext, executeGETVARARGSConst)
                                    as i32),
                            ),
                        );
                        (*self.build).blr(x4);

                        // note: no emitUpdateBase necessary because executeGETVARARGSConst does not reallocate stack
                    }
                }
                IrCmd::NEWCLOSURE => {
                    {
                        let mut reg = self.reg_op((*get_op_mut(inst, 1))); // note: we need to call regOp before spill so that we don't do redundant reloads

                        self.regs
                            .spill_u32_initializer_list_register_a_64(index, &[reg]);
                        (*self.build).mov(x2, reg);

                        (*self.build).mov(x0, rState);
                        (*self.build).mov(w1, self.uint_op((*get_op_mut(inst, 0))));

                        (*self.build).ldr(x3, mem(rClosure, (K_CLOSURE_L_P_OFFSET as i32)));
                        (*self.build).ldr(x3, mem(x3, (core::mem::offset_of!(Proto, p) as i32)));

                        let mut protoIndex = self.uint_op((*get_op_mut(inst, 2))); // 0..32767
                        let mut protoOffset = (((core::mem::size_of::<*mut Proto>() as i32)
                            * protoIndex as i32)
                            as i32);

                        if protoIndex <= AddressA64::kMaxOffset as u32 {
                            (*self.build).ldr(x3, mem(x3, protoOffset));
                        } else {
                            (*self.build).mov(x4, protoOffset);
                            (*self.build).ldr(x3, mem(x3, x4));
                        }

                        (*self.build).ldr(
                            x4,
                            mem(
                                rNativeContext,
                                (core::mem::offset_of!(NativeContext, luaF_newLclosure) as i32),
                            ),
                        );
                        (*self.build).blr(x4);

                        inst.reg_a64 = self.regs.take_reg(x0, index);
                    }
                }
                IrCmd::FALLBACK_DUPCLOSURE => {
                    CODEGEN_ASSERT!((*get_op_mut(inst, 1)).kind() == IrOpKind::VmReg);
                    CODEGEN_ASSERT!((*get_op_mut(inst, 2)).kind() == IrOpKind::VmConst);

                    self.regs
                        .spill_u32_initializer_list_register_a_64(index, &[]);
                    emit_fallback(
                        &mut *self.build,
                        (core::mem::offset_of!(NativeContext, executeDUPCLOSURE) as i32),
                        self.uint_op((*get_op_mut(inst, 0))),
                    );
                }
                IrCmd::FALLBACK_FORGPREP => {
                    self.regs
                        .spill_u32_initializer_list_register_a_64(index, &[]);
                    emit_fallback(
                        &mut *self.build,
                        (core::mem::offset_of!(NativeContext, executeFORGPREP) as i32),
                        self.uint_op((*get_op_mut(inst, 0))),
                    );
                    self.jump_or_fallthrough_op((*get_op_mut(inst, 2)), next);

                    // Pseudo instructions
                }
                IrCmd::NOP | IrCmd::SUBSTITUTE | IrCmd::MARK_USED | IrCmd::MARK_DEAD => {
                    CODEGEN_ASSERT!(false, "Pseudo instructions should not be lowered");
                }
                IrCmd::BITAND_INT64 => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::x,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );
                    let mut temp1 = self.temp_int64((*get_op_mut(inst, 0)));
                    let mut temp2 = self.temp_int64((*get_op_mut(inst, 1)));
                    (*self.build).and_(inst.reg_a64, temp1, temp2);
                }
                IrCmd::BITXOR_INT64 => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::x,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );
                    let mut temp1 = self.temp_int64((*get_op_mut(inst, 0)));
                    let mut temp2 = self.temp_int64((*get_op_mut(inst, 1)));
                    (*self.build).eor(inst.reg_a64, temp1, temp2);
                }
                IrCmd::BITOR_INT64 => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::x,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );
                    let mut temp1 = self.temp_int64((*get_op_mut(inst, 0)));
                    let mut temp2 = self.temp_int64((*get_op_mut(inst, 1)));
                    (*self.build).orr(inst.reg_a64, temp1, temp2);
                }
                IrCmd::BITNOT_INT64 => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::x, index, &[(*get_op_mut(inst, 0))]);
                    let mut temp = self.temp_int64((*get_op_mut(inst, 0)));
                    (*self.build).mvn_(inst.reg_a64, temp);
                }
                IrCmd::BITLSHIFT_INT64 => {
                    {
                        inst.reg_a64 = self.regs.alloc_reuse(
                            KindA64::x,
                            index,
                            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                        );
                        let mut source = self.temp_int64((*get_op_mut(inst, 0)));
                        let mut amount = self.temp_int64((*get_op_mut(inst, 1)));
                        let mut temp = self.regs.alloc_temp(KindA64::x);

                        let mut done = Label::default();
                        let mut negative = Label::default();
                        let mut outOfRange = Label::default();

                        // (amount + 63) > 126 = |amount| > 63
                        (*self.build).add(temp, amount, ((63) as u16));
                        (*self.build).cmp(temp, ((126) as u16));
                        (*self.build)
                            .b_condition_a_64_label(ConditionA64::UnsignedGreater, &mut outOfRange);

                        // check sign of amount
                        (*self.build).cmp(amount, ((0) as u16));
                        (*self.build).b_condition_a_64_label(ConditionA64::Less, &mut negative);

                        // left shift
                        (*self.build).lsl(inst.reg_a64, source, amount);
                        (*self.build).b(&mut done);

                        // right shift by -amount
                        (*self.build).set_label_label(&mut negative);
                        (*self.build).neg(temp, amount);
                        (*self.build).lsr(inst.reg_a64, source, temp);
                        (*self.build).b(&mut done);

                        (*self.build).set_label_label(&mut outOfRange);
                        (*self.build).mov(inst.reg_a64, 0);

                        (*self.build).set_label_label(&mut done);
                    }
                }
                IrCmd::BITRSHIFT_INT64 => {
                    {
                        inst.reg_a64 = self.regs.alloc_reuse(
                            KindA64::x,
                            index,
                            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                        );
                        let mut source = self.temp_int64((*get_op_mut(inst, 0)));
                        let mut amount = self.temp_int64((*get_op_mut(inst, 1)));
                        let mut temp = self.regs.alloc_temp(KindA64::x);

                        let mut done = Label::default();
                        let mut negative = Label::default();
                        let mut outOfRange = Label::default();

                        // (amount + 63) > 126 = |amount| > 63
                        (*self.build).add(temp, amount, ((63) as u16));
                        (*self.build).cmp(temp, ((126) as u16));
                        (*self.build)
                            .b_condition_a_64_label(ConditionA64::UnsignedGreater, &mut outOfRange);

                        // check sign of amount
                        (*self.build).cmp(amount, ((0) as u16));
                        (*self.build).b_condition_a_64_label(ConditionA64::Less, &mut negative);

                        // unsigned right shift
                        (*self.build).lsr(inst.reg_a64, source, amount);
                        (*self.build).b(&mut done);

                        // left shift by -amount
                        (*self.build).set_label_label(&mut negative);
                        (*self.build).neg(temp, amount);
                        (*self.build).lsl(inst.reg_a64, source, temp);
                        (*self.build).b(&mut done);

                        (*self.build).set_label_label(&mut outOfRange);
                        (*self.build).mov(inst.reg_a64, 0);

                        (*self.build).set_label_label(&mut done);
                    }
                }
                IrCmd::BITARSHIFT_INT64 => {
                    {
                        inst.reg_a64 = self.regs.alloc_reuse(
                            KindA64::x,
                            index,
                            &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                        );
                        let mut source = self.temp_int64((*get_op_mut(inst, 0)));
                        let mut amount = self.temp_int64((*get_op_mut(inst, 1)));
                        let mut temp = self.regs.alloc_temp(KindA64::x);

                        let mut done = Label::default();
                        let mut negative = Label::default();
                        let mut outOfRangePositive = Label::default();
                        let mut outOfRangeNegative = Label::default();

                        // amount > 63 (arithmetic right shift fills with sign)
                        (*self.build).cmp(amount, ((63) as u16));
                        (*self.build)
                            .b_condition_a_64_label(ConditionA64::Greater, &mut outOfRangePositive);

                        // add 63, if < 0 then amount < -63
                        (*self.build).add(temp, amount, ((63) as u16));
                        (*self.build).cmp(temp, ((0) as u16));
                        (*self.build)
                            .b_condition_a_64_label(ConditionA64::Less, &mut outOfRangeNegative);

                        // check sign of amount
                        (*self.build).cmp(amount, ((0) as u16));
                        (*self.build).b_condition_a_64_label(ConditionA64::Less, &mut negative);

                        // arithmetic right shift that sign extends
                        (*self.build).asr(inst.reg_a64, source, amount);
                        (*self.build).b(&mut done);

                        // left shift by -amount (((*self.build).set_label_label(&mut negative)) as u32);
                        (*self.build).neg(temp, amount);
                        (*self.build).lsl(inst.reg_a64, source, temp);
                        (*self.build).b(&mut done);

                        // amount > 63 = sign-fill ( if n < 0 { -1 } else { 0 })
                        (*self.build).set_label_label(&mut outOfRangePositive);
                        (*self.build).asr(inst.reg_a64, source, ((63) as u8));
                        (*self.build).b(&mut done);

                        // amount < -63 = result is 0
                        (*self.build).set_label_label(&mut outOfRangeNegative);
                        (*self.build).mov(inst.reg_a64, 0);

                        (*self.build).set_label_label(&mut done);
                    }
                }
                IrCmd::BITLROTATE_INT64 => {
                    {
                        inst.reg_a64 =
                            self.regs
                                .alloc_reuse(KindA64::x, index, &[(*get_op_mut(inst, 1))]); // can't reuse A because it would be clobbered by neg
                        let mut source = self.temp_int64((*get_op_mut(inst, 0)));
                        let mut amount = self.temp_int64((*get_op_mut(inst, 1)));
                        // left rotate = rotate by negative
                        (*self.build).neg(inst.reg_a64, amount);
                        (*self.build).ror(inst.reg_a64, source, inst.reg_a64);
                    }
                }
                IrCmd::BITRROTATE_INT64 => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::x,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );
                    let mut source = self.temp_int64((*get_op_mut(inst, 0)));
                    let mut amount = self.temp_int64((*get_op_mut(inst, 1)));
                    (*self.build).ror(inst.reg_a64, source, amount);
                }
                IrCmd::BITCOUNTLZ_INT64 => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::x, index, &[(*get_op_mut(inst, 0))]);
                    let mut temp = self.temp_int64((*get_op_mut(inst, 0)));
                    (*self.build).clz(inst.reg_a64, temp);
                }
                IrCmd::BITCOUNTRZ_INT64 => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::x, index, &[(*get_op_mut(inst, 0))]);
                    let mut temp = self.temp_int64((*get_op_mut(inst, 0)));
                    (*self.build).rbit(inst.reg_a64, temp);
                    (*self.build).clz(inst.reg_a64, inst.reg_a64);
                }
                IrCmd::BYTESWAP_INT64 => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::x, index, &[(*get_op_mut(inst, 0))]);
                    let mut temp = self.temp_int64((*get_op_mut(inst, 0)));
                    (*self.build).rev(inst.reg_a64, temp);
                }
                IrCmd::BITAND_UINT => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::w,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );
                    if (*get_op_mut(inst, 0)).kind() == IrOpKind::Inst
                        && (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
                        && (*self.build)
                            .is_mask_supported(((self.int_op((*get_op_mut(inst, 1)))) as u32))
                    {
                        (*self.build).and_(
                            inst.reg_a64,
                            self.reg_op((*get_op_mut(inst, 0))),
                            ((self.int_op((*get_op_mut(inst, 1)))) as u32),
                        );
                    } else {
                        let mut temp1 = self.temp_uint((*get_op_mut(inst, 0)));
                        let mut temp2 = self.temp_uint((*get_op_mut(inst, 1)));
                        (*self.build).and_(inst.reg_a64, temp1, temp2);
                    }
                }
                IrCmd::BITXOR_UINT => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::w,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );
                    if (*get_op_mut(inst, 0)).kind() == IrOpKind::Inst
                        && (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
                        && (*self.build)
                            .is_mask_supported(((self.int_op((*get_op_mut(inst, 1)))) as u32))
                    {
                        (*self.build).eor(
                            inst.reg_a64,
                            self.reg_op((*get_op_mut(inst, 0))),
                            ((self.int_op((*get_op_mut(inst, 1)))) as u32),
                        );
                    } else {
                        let mut temp1 = self.temp_uint((*get_op_mut(inst, 0)));
                        let mut temp2 = self.temp_uint((*get_op_mut(inst, 1)));
                        (*self.build).eor(inst.reg_a64, temp1, temp2);
                    }
                }
                IrCmd::BITOR_UINT => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::w,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );
                    if (*get_op_mut(inst, 0)).kind() == IrOpKind::Inst
                        && (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
                        && (*self.build)
                            .is_mask_supported(((self.int_op((*get_op_mut(inst, 1)))) as u32))
                    {
                        (*self.build).orr(
                            inst.reg_a64,
                            self.reg_op((*get_op_mut(inst, 0))),
                            ((self.int_op((*get_op_mut(inst, 1)))) as u32),
                        );
                    } else {
                        let mut temp1 = self.temp_uint((*get_op_mut(inst, 0)));
                        let mut temp2 = self.temp_uint((*get_op_mut(inst, 1)));
                        (*self.build).orr(inst.reg_a64, temp1, temp2);
                    }
                }
                IrCmd::BITNOT_UINT => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::w, index, &[(*get_op_mut(inst, 0))]);
                    let mut temp = self.temp_uint((*get_op_mut(inst, 0)));
                    (*self.build).mvn_(inst.reg_a64, temp);
                }
                IrCmd::BITLSHIFT_UINT => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::w,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );
                    if (*get_op_mut(inst, 0)).kind() == IrOpKind::Inst
                        && (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
                    {
                        (*self.build).lsl(
                            inst.reg_a64,
                            self.reg_op((*get_op_mut(inst, 0))),
                            ((((self.int_op((*get_op_mut(inst, 1)))) as u32) & 31) as u8),
                        );
                    } else {
                        let mut temp1 = self.temp_uint((*get_op_mut(inst, 0)));
                        let mut temp2 = self.temp_uint((*get_op_mut(inst, 1)));
                        (*self.build).lsl(inst.reg_a64, temp1, temp2);
                    }
                }
                IrCmd::BITRSHIFT_UINT => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::w,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );
                    if (*get_op_mut(inst, 0)).kind() == IrOpKind::Inst
                        && (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
                    {
                        (*self.build).lsr(
                            inst.reg_a64,
                            self.reg_op((*get_op_mut(inst, 0))),
                            ((((self.int_op((*get_op_mut(inst, 1)))) as u32) & 31) as u8),
                        );
                    } else {
                        let mut temp1 = self.temp_uint((*get_op_mut(inst, 0)));
                        let mut temp2 = self.temp_uint((*get_op_mut(inst, 1)));
                        (*self.build).lsr(inst.reg_a64, temp1, temp2);
                    }
                }
                IrCmd::BITARSHIFT_UINT => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::w,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );
                    if (*get_op_mut(inst, 0)).kind() == IrOpKind::Inst
                        && (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
                    {
                        (*self.build).asr(
                            inst.reg_a64,
                            self.reg_op((*get_op_mut(inst, 0))),
                            ((((self.int_op((*get_op_mut(inst, 1)))) as u32) & 31) as u8),
                        );
                    } else {
                        let mut temp1 = self.temp_uint((*get_op_mut(inst, 0)));
                        let mut temp2 = self.temp_uint((*get_op_mut(inst, 1)));
                        (*self.build).asr(inst.reg_a64, temp1, temp2);
                    }
                }
                IrCmd::BITLROTATE_UINT => {
                    {
                        if (*get_op_mut(inst, 0)).kind() == IrOpKind::Inst
                            && (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
                        {
                            inst.reg_a64 =
                                self.regs
                                    .alloc_reuse(KindA64::w, index, &[(*get_op_mut(inst, 0))]);
                            (*self.build).ror(
                                inst.reg_a64,
                                self.reg_op((*get_op_mut(inst, 0))),
                                (((32 - ((self.int_op((*get_op_mut(inst, 1)))) as u32)) & 31)
                                    as u8),
                            );
                        } else {
                            inst.reg_a64 =
                                self.regs
                                    .alloc_reuse(KindA64::w, index, &[(*get_op_mut(inst, 1))]); // can't reuse a because it would be clobbered by neg
                            let mut temp1 = self.temp_uint((*get_op_mut(inst, 0)));
                            let mut temp2 = self.temp_uint((*get_op_mut(inst, 1)));
                            (*self.build).neg(inst.reg_a64, temp2);
                            (*self.build).ror(inst.reg_a64, temp1, inst.reg_a64);
                        }
                    }
                }
                IrCmd::BITRROTATE_UINT => {
                    inst.reg_a64 = self.regs.alloc_reuse(
                        KindA64::w,
                        index,
                        &[(*get_op_mut(inst, 0)), (*get_op_mut(inst, 1))],
                    );
                    if (*get_op_mut(inst, 0)).kind() == IrOpKind::Inst
                        && (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
                    {
                        (*self.build).ror(
                            inst.reg_a64,
                            self.reg_op((*get_op_mut(inst, 0))),
                            ((((self.int_op((*get_op_mut(inst, 1)))) as u32) & 31) as u8),
                        );
                    } else {
                        let mut temp1 = self.temp_uint((*get_op_mut(inst, 0)));
                        let mut temp2 = self.temp_uint((*get_op_mut(inst, 1)));
                        (*self.build).ror(inst.reg_a64, temp1, temp2);
                    }
                }
                IrCmd::BITCOUNTLZ_UINT => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::w, index, &[(*get_op_mut(inst, 0))]);
                    let mut temp = self.temp_uint((*get_op_mut(inst, 0)));
                    (*self.build).clz(inst.reg_a64, temp);
                }
                IrCmd::BITCOUNTRZ_UINT => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::w, index, &[(*get_op_mut(inst, 0))]);
                    let mut temp = self.temp_uint((*get_op_mut(inst, 0)));
                    (*self.build).rbit(inst.reg_a64, temp);
                    (*self.build).clz(inst.reg_a64, inst.reg_a64);
                }
                IrCmd::BYTESWAP_UINT => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::w, index, &[(*get_op_mut(inst, 0))]);
                    let mut temp = self.temp_uint((*get_op_mut(inst, 0)));
                    (*self.build).rev(inst.reg_a64, temp);
                }
                IrCmd::INVOKE_LIBM => {
                    {
                        if has_op_c(inst) {
                            let mut isInt = if ((*get_op_mut(inst, 2)).kind() == IrOpKind::Constant)
                            {
                                self.const_op((*get_op_mut(inst, 2))).kind == IrConstKind::Int
                            } else {
                                get_cmd_value_kind(
                                    (*self.function).inst_op((*get_op_mut(inst, 2))).cmd,
                                ) == IrValueKind::Int
                            };

                            let mut temp1 = self.temp_double((*get_op_mut(inst, 1)));
                            let mut temp2 = if isInt {
                                self.temp_int((*get_op_mut(inst, 2)))
                            } else {
                                self.temp_double((*get_op_mut(inst, 2)))
                            };
                            let mut temp3 = if isInt {
                                noreg
                            } else {
                                self.regs.alloc_temp(KindA64::d)
                            }; // note: spill() frees all registers so we need to avoid alloc after spill
                            self.regs
                                .spill_u32_initializer_list_register_a_64(index, &[temp1, temp2]);

                            if isInt {
                                (*self.build).fmov(d0, temp1);
                                (*self.build).mov(w0, temp2);
                            } else if d0 != temp2 {
                                (*self.build).fmov(d0, temp1);
                                (*self.build).fmov(d1, temp2);
                            } else {
                                (*self.build).fmov(temp3, d0);
                                (*self.build).fmov(d0, temp1);
                                (*self.build).fmov(d1, temp3);
                            }
                        } else {
                            let mut temp1 = self.temp_double((*get_op_mut(inst, 1)));
                            self.regs
                                .spill_u32_initializer_list_register_a_64(index, &[temp1]);
                            (*self.build).fmov(d0, temp1);
                        }

                        (*self.build).ldr(
                            x1,
                            mem(
                                rNativeContext,
                                get_native_context_offset(
                                    self.uint_op((*get_op_mut(inst, 0))) as i32
                                ) as i32,
                            ),
                        );
                        (*self.build).blr(x1);
                        inst.reg_a64 = self.regs.take_reg(d0, index);
                    }
                }
                IrCmd::GET_TYPE => {
                    {
                        inst.reg_a64 = self.regs.alloc_reg(KindA64::x, index);

                        CODEGEN_ASSERT!((core::mem::size_of::<*mut TString>() as i32) == 8);

                        if (*get_op_mut(inst, 0)).kind() == IrOpKind::Inst {
                            (*self.build).add_register_a_64_register_a_64_register_a_64_i32(
                                inst.reg_a64,
                                rGlobalState,
                                self.reg_op((*get_op_mut(inst, 0))),
                                3,
                            );
                        }
                        // implicit uxtw
                        else if (*get_op_mut(inst, 0)).kind() == IrOpKind::Constant {
                            (*self.build).add(
                                inst.reg_a64,
                                rGlobalState,
                                ((self.tag_op((*get_op_mut(inst, 0))) * 8) as u16),
                            );
                        } else {
                            CODEGEN_ASSERT!(false, "Unsupported instruction form");
                        }

                        (*self.build).ldr(
                            inst.reg_a64,
                            mem(
                                inst.reg_a64,
                                (core::mem::offset_of!(global_State, ttname) as i32),
                            ),
                        );
                    }
                }
                IrCmd::GET_TYPEOF => {
                    self.regs
                        .spill_u32_initializer_list_register_a_64(index, &[]);
                    (*self.build).mov(x0, rState);
                    (*self.build).add(
                        x1,
                        r_base(),
                        ((vm_reg_op((*get_op_mut(inst, 0)))
                            * (core::mem::size_of::<TValue>() as i32))
                            as u16),
                    );
                    (*self.build).ldr(
                        x2,
                        mem(
                            rNativeContext,
                            (core::mem::offset_of!(NativeContext, luaT_objtypenamestr) as i32),
                        ),
                    );
                    (*self.build).blr(x2);

                    inst.reg_a64 = self.regs.take_reg(x0, index);
                }
                IrCmd::FINDUPVAL => {
                    self.regs
                        .spill_u32_initializer_list_register_a_64(index, &[]);
                    (*self.build).mov(x0, rState);
                    (*self.build).add(
                        x1,
                        r_base(),
                        ((vm_reg_op((*get_op_mut(inst, 0)))
                            * (core::mem::size_of::<TValue>() as i32))
                            as u16),
                    );
                    (*self.build).ldr(
                        x2,
                        mem(
                            rNativeContext,
                            (core::mem::offset_of!(NativeContext, luaF_findupval) as i32),
                        ),
                    );
                    (*self.build).blr(x2);

                    inst.reg_a64 = self.regs.take_reg(x0, index);
                }
                IrCmd::BUFFER_READI8 => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::w, index, &[(*get_op_mut(inst, 1))]);
                    let mut addr = self.temp_addr_buffer(
                        (*get_op_mut(inst, 0)),
                        (*get_op_mut(inst, 1)),
                        self.tag_op((*get_op_mut(inst, 2))),
                    );

                    (*self.build).ldrsb(inst.reg_a64, addr);
                }
                IrCmd::BUFFER_READU8 => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::w, index, &[(*get_op_mut(inst, 1))]);
                    let mut addr = self.temp_addr_buffer(
                        (*get_op_mut(inst, 0)),
                        (*get_op_mut(inst, 1)),
                        self.tag_op((*get_op_mut(inst, 2))),
                    );

                    (*self.build).ldrb(inst.reg_a64, addr);
                }
                IrCmd::BUFFER_WRITEI8 => {
                    let mut temp = self.temp_int((*get_op_mut(inst, 2)));
                    let mut addr = self.temp_addr_buffer(
                        (*get_op_mut(inst, 0)),
                        (*get_op_mut(inst, 1)),
                        self.tag_op((*get_op_mut(inst, 3))),
                    );

                    (*self.build).strb(temp, addr);
                }
                IrCmd::BUFFER_READI16 => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::w, index, &[(*get_op_mut(inst, 1))]);
                    let mut addr = self.temp_addr_buffer(
                        (*get_op_mut(inst, 0)),
                        (*get_op_mut(inst, 1)),
                        self.tag_op((*get_op_mut(inst, 2))),
                    );

                    (*self.build).ldrsh(inst.reg_a64, addr);
                }
                IrCmd::BUFFER_READU16 => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::w, index, &[(*get_op_mut(inst, 1))]);
                    let mut addr = self.temp_addr_buffer(
                        (*get_op_mut(inst, 0)),
                        (*get_op_mut(inst, 1)),
                        self.tag_op((*get_op_mut(inst, 2))),
                    );

                    (*self.build).ldrh(inst.reg_a64, addr);
                }
                IrCmd::BUFFER_WRITEI16 => {
                    let mut temp = self.temp_int((*get_op_mut(inst, 2)));
                    let mut addr = self.temp_addr_buffer(
                        (*get_op_mut(inst, 0)),
                        (*get_op_mut(inst, 1)),
                        self.tag_op((*get_op_mut(inst, 3))),
                    );

                    (*self.build).strh(temp, addr);
                }
                IrCmd::BUFFER_READI32 => {
                    inst.reg_a64 =
                        self.regs
                            .alloc_reuse(KindA64::w, index, &[(*get_op_mut(inst, 1))]);
                    let mut addr = self.temp_addr_buffer(
                        (*get_op_mut(inst, 0)),
                        (*get_op_mut(inst, 1)),
                        self.tag_op((*get_op_mut(inst, 2))),
                    );

                    (*self.build).ldr(inst.reg_a64, addr);
                }
                IrCmd::BUFFER_WRITEI32 => {
                    let mut temp = self.temp_int((*get_op_mut(inst, 2)));
                    let mut addr = self.temp_addr_buffer(
                        (*get_op_mut(inst, 0)),
                        (*get_op_mut(inst, 1)),
                        self.tag_op((*get_op_mut(inst, 3))),
                    );

                    (*self.build).str(temp, addr);
                }
                IrCmd::BUFFER_READF32 => {
                    inst.reg_a64 = self.regs.alloc_reg(KindA64::s, index);
                    let mut addr = self.temp_addr_buffer(
                        (*get_op_mut(inst, 0)),
                        (*get_op_mut(inst, 1)),
                        self.tag_op((*get_op_mut(inst, 2))),
                    );

                    (*self.build).ldr(inst.reg_a64, addr);
                }
                IrCmd::BUFFER_WRITEF32 => {
                    let mut temp = self.temp_float((*get_op_mut(inst, 2)));
                    let mut addr = self.temp_addr_buffer(
                        (*get_op_mut(inst, 0)),
                        (*get_op_mut(inst, 1)),
                        self.tag_op((*get_op_mut(inst, 3))),
                    );

                    (*self.build).str(temp, addr);
                }
                IrCmd::BUFFER_READF64 => {
                    inst.reg_a64 = self.regs.alloc_reg(KindA64::d, index);
                    let mut addr = self.temp_addr_buffer(
                        (*get_op_mut(inst, 0)),
                        (*get_op_mut(inst, 1)),
                        self.tag_op((*get_op_mut(inst, 2))),
                    );

                    (*self.build).ldr(inst.reg_a64, addr);
                }
                IrCmd::BUFFER_WRITEF64 => {
                    let mut temp = self.temp_double((*get_op_mut(inst, 2)));
                    let mut addr = self.temp_addr_buffer(
                        (*get_op_mut(inst, 0)),
                        (*get_op_mut(inst, 1)),
                        self.tag_op((*get_op_mut(inst, 3))),
                    );

                    (*self.build).str(temp, addr);
                }
                IrCmd::BUFFER_READI64 => {
                    inst.reg_a64 = self.regs.alloc_reg(KindA64::x, index);
                    let mut addr = self.temp_addr_buffer(
                        (*get_op_mut(inst, 0)),
                        (*get_op_mut(inst, 1)),
                        self.tag_op((*get_op_mut(inst, 2))),
                    );

                    (*self.build).ldr(inst.reg_a64, addr);
                }
                IrCmd::BUFFER_WRITEI64 => {
                    let mut temp = self.temp_int64((*get_op_mut(inst, 2)));
                    let mut addr = self.temp_addr_buffer(
                        (*get_op_mut(inst, 0)),
                        (*get_op_mut(inst, 1)),
                        self.tag_op((*get_op_mut(inst, 3))),
                    );

                    (*self.build).str(temp, addr);
                }
                IrCmd::JUMP_CMP_PROTOID => {
                    {
                        CODEGEN_ASSERT!(
                            (*get_op_mut(inst, 0)).kind() == IrOpKind::Inst
                                && (*get_op_mut(inst, 1)).kind() == IrOpKind::Constant
                        );
                        let mut temp = self.regs.alloc_temp(KindA64::x);
                        let mut tempw = cast_reg(KindA64::w, temp);

                        // Is it a C closure?
                        (*self.build).ldrb(
                            tempw,
                            mem(
                                self.reg_op((*get_op_mut(inst, 0))),
                                (core::mem::offset_of!(Closure, isC) as i32),
                            ),
                        );
                        (*self.build).cbnz(tempw, self.label_op((*get_op_mut(inst, 3))));

                        // Load Proto and compare funid
                        (*self.build).ldr(
                            temp,
                            mem(
                                self.reg_op((*get_op_mut(inst, 0))),
                                (K_CLOSURE_L_P_OFFSET as i32),
                            ),
                        );
                        (*self.build).ldr(
                            tempw,
                            mem(temp, (core::mem::offset_of!(Proto, funid) as i32)),
                        );
                        let mut protoId = self.uint_op((*get_op_mut(inst, 1)));
                        if protoId <= K_MAX_IMMEDIATE {
                            (*self.build).cmp(tempw, (protoId as u16));
                        } else {
                            let mut temp2 = self.regs.alloc_temp(KindA64::w);
                            (*self.build).mov(temp2, protoId);
                            (*self.build).cmp(tempw, temp2);
                        }

                        (*self.build).b_condition_a_64_label(
                            ConditionA64::NotEqual,
                            self.label_op((*get_op_mut(inst, 3))),
                        );

                        self.jump_or_fallthrough_op((*get_op_mut(inst, 2)), next);
                    }
                }

                // To handle unsupported instructions, add "case IrCmd::OP" and make sure to set error = true!
                _ => {
                    CODEGEN_ASSERT!(false, "unhandled IrCmd");
                }
            }
            self.value_tracker.after_inst_lowering(inst, index);

            self.regs.curr_inst_idx = kInvalidInstIdx;

            self.regs.free_last_use_regs(inst, index);
            self.regs.free_temp_regs();
        }
    }
}
