//! Node: `cxx:Method:Luau.CodeGen:CodeGen/src/IrLoweringX64.cpp:50:lowerInst`
//! Mechanically transpiled (translation/scripts/lowerinst_rewrite.py) + compiler-driven repair.
#![allow(
    unused_parens,
    unused_braces,
    unused_variables,
    unused_unsafe,
    unused_imports,
    unused_mut
)]
use crate::enums::condition_x_64::ConditionX64;
use crate::enums::features_x_64::FeaturesX64;
use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_condition::IrCondition;
use crate::enums::ir_const_kind::IrConstKind;
use crate::enums::ir_op_kind::IrOpKind;
use crate::enums::ir_value_kind::IrValueKind;
use crate::enums::rounding_mode_x_64::RoundingModeX64;
use crate::enums::size_x_64::SizeX64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::macros::has_op_b::HAS_OP_B;
use crate::macros::has_op_c::HAS_OP_C;
use crate::macros::has_op_d::HAS_OP_D;
use crate::macros::has_op_e::HAS_OP_E;
use crate::records::interrupt_handler_ir_lowering_x_64::InterruptHandler;
use crate::records::ir_block::IrBlock;
use crate::records::ir_call_wrapper_x_64::IrCallWrapperX64;
use crate::records::ir_inst::IrInst;
use crate::records::ir_lowering_x_64::IrLoweringX64;
use crate::records::ir_op::IrOp;
use crate::records::label::Label;
use crate::records::native_context::NativeContext;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;
use crate::records::scoped_reg_x_64::ScopedRegX64;
use crate::records::scoped_spills::ScopedSpills;
use luaur_common::FFlag;
use luaur_vm::enums::lua_type::lua_Type;
use luaur_vm::macros::lua_multret::LUA_MULTRET;
use luaur_vm::macros::setnvalue::setnvalue;
use luaur_vm::records::call_info::CallInfo;
use luaur_vm::records::closure::{Closure, LClosure};
use luaur_vm::records::global_state::global_State;
use luaur_vm::records::lua_t_value::TValue;
use luaur_vm::records::proto::Proto;
use luaur_vm::records::t_string::TString;
use luaur_vm::records::up_val::UpVal;
use luaur_vm::type_aliases::buffer::Buffer;
use luaur_vm::type_aliases::instruction::Instruction;
use luaur_vm::type_aliases::lua_node::LuaNode;
use luaur_vm::type_aliases::lua_state::lua_State;
use luaur_vm::type_aliases::lua_table::LuaTable;
use luaur_vm::type_aliases::luau_fast_function::luau_FastFunction;
use luaur_vm::type_aliases::tms::TMS;
use luaur_vm::type_aliases::udata::Udata;
// local register-constant helpers (mirrors EmitCommonX64.h)
const kTValueSizeLog2: i32 = 4;
const kLuaNodeSizeLog2: i32 = 5;
const kOffsetOfTKeyTagNext: i32 = 12;
const kTKeyTagBits: i32 = 4;
const kTKeyTagMask: i32 = (1 << kTKeyTagBits) - 1;
const kStackOffsetToLocals: i32 = 48;
const kInvalidInstIdx: u32 = IrLoweringX64::kInvalidInstIdx;
const INT_MAX: i32 = i32::MAX;
const kTStringLenOffset: i32 = 36;
const kBufferLenOffset: i32 = 12;
const kClosureLUprefsOffset: i32 =
    (core::mem::offset_of!(Closure, inner) + core::mem::offset_of!(LClosure, uprefs)) as i32;
const kClosureLPoffset: i32 =
    (core::mem::offset_of!(Closure, inner) + core::mem::offset_of!(LClosure, p)) as i32;
const sClosure: OperandX64 = OperandX64::mem(
    SizeX64::qword,
    RegisterX64::noreg,
    1,
    RegisterX64::rsp,
    kStackOffsetToLocals,
);
const sCode: OperandX64 = OperandX64::mem(
    SizeX64::qword,
    RegisterX64::noreg,
    1,
    RegisterX64::rsp,
    kStackOffsetToLocals + 8,
);
const fn r_state() -> RegisterX64 {
    RegisterX64 {
        bits: (15u8 << RegisterX64::INDEX_SHIFT) | crate::enums::size_x_64::SizeX64::qword as u8,
    }
}
const fn r_base() -> RegisterX64 {
    RegisterX64 {
        bits: (14u8 << RegisterX64::INDEX_SHIFT) | crate::enums::size_x_64::SizeX64::qword as u8,
    }
}
const fn r_constants() -> RegisterX64 {
    RegisterX64 {
        bits: (12u8 << RegisterX64::INDEX_SHIFT) | crate::enums::size_x_64::SizeX64::qword as u8,
    }
}
const fn r_native_context() -> RegisterX64 {
    RegisterX64 {
        bits: (13u8 << RegisterX64::INDEX_SHIFT) | crate::enums::size_x_64::SizeX64::qword as u8,
    }
}
const fn xmm(index: u8) -> RegisterX64 {
    RegisterX64 {
        bits: (index << RegisterX64::INDEX_SHIFT) | crate::enums::size_x_64::SizeX64::xmmword as u8,
    }
}
const fn xmm0() -> RegisterX64 {
    xmm(0)
}
fn sized_mem(mut op: OperandX64, size: SizeX64) -> OperandX64 {
    op.memSize = size;
    op
}
use crate::functions::byte_reg::byte_reg;
use crate::functions::call_arith_helper::call_arith_helper;
use crate::functions::call_barrier_object::call_barrier_object;
use crate::functions::call_barrier_table_fast::call_barrier_table_fast;
use crate::functions::call_get_table::call_get_table;
use crate::functions::call_length_helper::call_length_helper;
use crate::functions::call_set_table::call_set_table;
use crate::functions::call_step_gc::call_step_gc;
use crate::functions::check_object_barrier_conditions::check_object_barrier_conditions;
use crate::functions::condition_op::condition_op;
use crate::functions::convert_number_to_index_or_jump::convert_number_to_index_or_jump;
use crate::functions::dword_reg::dword_reg;
use crate::functions::emit_builtin_emit_builtins_x_64::emit_builtin_ir_reg_alloc_x_64_assembly_builder_x_64_i32_i32_i32_i32 as emit_builtin;
use crate::functions::emit_fallback_emit_common_x_64::emit_fallback;
use crate::functions::emit_inst_call::emit_inst_call;
use crate::functions::emit_inst_for_g_loop::emit_inst_for_g_loop;
use crate::functions::emit_inst_return::emit_inst_return;
use crate::functions::emit_inst_set_list::emit_inst_set_list;
use crate::functions::emit_update_base_emit_common_x_64::emit_update_base;
use crate::functions::get_cmd_value_kind::get_cmd_value_kind;
use crate::functions::get_condition_int_emit_common_x_64::get_condition_int;
use crate::functions::get_inverse_condition_condition_x_64::get_inverse_condition;
use crate::functions::get_native_context_offset::get_native_context_offset;
use crate::functions::get_negated_condition_condition_x_64::get_negated_condition;
use crate::functions::get_negated_condition_ir_utils::get_negated_condition_ir_condition;
use crate::functions::get_op_ir_data::get_op_mut;
use crate::functions::get_table_node_at_cached_slot::get_table_node_at_cached_slot;
use crate::functions::is_gco::is_gco;
use crate::functions::jump_if_falsy::jump_if_falsy;
use crate::functions::jump_if_truthy::jump_if_truthy;
use crate::functions::jump_on_number_cmp::jump_on_number_cmp;
use crate::functions::luau_constant::luau_constant;
use crate::functions::luau_constant_address::luau_constant_address;
use crate::functions::luau_constant_tag::luau_constant_tag;
use crate::functions::luau_constant_value::luau_constant_value;
use crate::functions::luau_node_key_tag::luau_node_key_tag;
use crate::functions::luau_node_key_value::luau_node_key_value;
use crate::functions::luau_reg::luau_reg;
use crate::functions::luau_reg_address::luau_reg_address;
use crate::functions::luau_reg_extra::luau_reg_extra;
use crate::functions::luau_reg_tag::luau_reg_tag;
use crate::functions::luau_reg_value::luau_reg_value;
use crate::functions::luau_reg_value_int::luau_reg_value_int;
use crate::functions::luau_reg_value_int_64::luau_reg_value_int_64;
use crate::functions::luau_reg_value_vector::luau_reg_value_vector;
use crate::functions::produces_dirty_high_register_bits::produces_dirty_high_register_bits;
use crate::functions::qword_reg::qword_reg;
use crate::functions::vm_const_op::vm_const_op;
use crate::functions::vm_reg_op::vm_reg_op;
use crate::functions::vm_upvalue_op::vm_upvalue_op;
use crate::functions::word_reg::word_reg;

impl IrLoweringX64 {
    pub fn lower_inst(&mut self, inst: &mut IrInst, index: u32, next: &IrBlock) {
        unsafe {
            self.regs.curr_inst_idx = index;

            self.value_tracker.before_inst_lowering(inst);
            match inst.cmd {
                IrCmd::LOAD_TAG => {
                    inst.reg_x64 = self.regs.alloc_reg(SizeX64::dword, index);

                    if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::VmReg {
                        (*self.build).mov(
                            OperandX64::reg(inst.reg_x64),
                            luau_reg_tag(vm_reg_op((*get_op_mut(inst, 0)))),
                        );
                    } else if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::VmConst
                    {
                        (*self.build).mov(
                            OperandX64::reg(inst.reg_x64),
                            luau_constant_tag(vm_const_op((*get_op_mut(inst, 0)))),
                        );
                    }
                    // If we have a register, we assume it's a pointer to TValue
                    // We might introduce explicit operand types in the future to make this more robust
                    else if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Inst {
                        (*self.build).mov(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::mem(
                                SizeX64::dword,
                                RegisterX64::noreg,
                                1,
                                self.reg_op((*get_op_mut(inst, 0))),
                                (core::mem::offset_of!(TValue, tt) as i32),
                            ),
                        );
                    } else {
                        CODEGEN_ASSERT!(false, "Unsupported instruction form");
                    }
                }
                IrCmd::LOAD_POINTER => {
                    inst.reg_x64 = self.regs.alloc_reg(SizeX64::qword, index);

                    if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::VmReg {
                        (*self.build).mov(
                            OperandX64::reg(inst.reg_x64),
                            luau_reg_value(vm_reg_op((*get_op_mut(inst, 0)))),
                        );
                    } else if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::VmConst
                    {
                        (*self.build).mov(
                            OperandX64::reg(inst.reg_x64),
                            luau_constant_value(vm_const_op((*get_op_mut(inst, 0)))),
                        );
                    }
                    // If we have a register, we assume it's a pointer to TValue
                    // We might introduce explicit operand types in the future to make this more robust
                    else if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Inst {
                        (*self.build).mov(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::mem(
                                SizeX64::qword,
                                RegisterX64::noreg,
                                1,
                                self.reg_op((*get_op_mut(inst, 0))),
                                (core::mem::offset_of!(TValue, value) as i32),
                            ),
                        );
                    } else {
                        CODEGEN_ASSERT!(false, "Unsupported instruction form");
                    }
                }
                IrCmd::LOAD_DOUBLE => {
                    inst.reg_x64 = self.regs.alloc_reg(SizeX64::xmmword, index);

                    if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::VmReg {
                        (*self.build).vmovsd_operand_x_64_operand_x_64(
                            OperandX64::reg(inst.reg_x64),
                            luau_reg_value(vm_reg_op((*get_op_mut(inst, 0)))),
                        );
                    } else if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::VmConst
                    {
                        (*self.build).vmovsd_operand_x_64_operand_x_64(
                            OperandX64::reg(inst.reg_x64),
                            luau_constant_value(vm_const_op((*get_op_mut(inst, 0)))),
                        );
                    } else {
                        CODEGEN_ASSERT!(false, "Unsupported instruction form");
                    }
                }
                IrCmd::LOAD_INT => {
                    inst.reg_x64 = self.regs.alloc_reg(SizeX64::dword, index);

                    (*self.build).mov(
                        OperandX64::reg(inst.reg_x64),
                        luau_reg_value_int(vm_reg_op((*get_op_mut(inst, 0)))),
                    );
                }
                IrCmd::LOAD_INT64 => {
                    inst.reg_x64 = self.regs.alloc_reg(SizeX64::qword, index);

                    if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::VmReg {
                        (*self.build).mov(
                            OperandX64::reg(inst.reg_x64),
                            luau_reg_value_int_64(vm_reg_op((*get_op_mut(inst, 0)))),
                        );
                    } else if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::VmConst
                    {
                        (*self.build).mov(
                            OperandX64::reg(inst.reg_x64),
                            luau_constant_value(vm_const_op((*get_op_mut(inst, 0)))),
                        );
                    } else {
                        CODEGEN_ASSERT!(false, "Unsupported instruction form");
                    }
                }
                IrCmd::LOAD_FLOAT => {
                    inst.reg_x64 = self.regs.alloc_reg(SizeX64::xmmword, index);

                    if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::VmReg {
                        (*self.build).vmovss_operand_x_64_operand_x_64(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::mem(
                                SizeX64::dword,
                                RegisterX64::noreg,
                                1,
                                r_base(),
                                vm_reg_op((*get_op_mut(inst, 0)))
                                    * (core::mem::size_of::<TValue>() as i32)
                                    + (core::mem::offset_of!(TValue, value) as i32)
                                    + self.int_op((*get_op_mut(inst, 1))),
                            ),
                        );
                    } else if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::VmConst
                    {
                        (*self.build).vmovss_operand_x_64_operand_x_64(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::mem(
                                SizeX64::dword,
                                RegisterX64::noreg,
                                1,
                                r_constants(),
                                vm_const_op((*get_op_mut(inst, 0)))
                                    * (core::mem::size_of::<TValue>() as i32)
                                    + (core::mem::offset_of!(TValue, value) as i32)
                                    + self.int_op((*get_op_mut(inst, 1))),
                            ),
                        );
                    } else {
                        CODEGEN_ASSERT!(false, "Unsupported instruction form");
                    }
                }
                IrCmd::LOAD_TVALUE => {
                    inst.reg_x64 = self.regs.alloc_reg(SizeX64::xmmword, index);

                    let mut addrOffset = if HAS_OP_B!(inst) {
                        self.int_op((*get_op_mut(inst, 1)))
                    } else {
                        0
                    };

                    if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::VmReg {
                        (*self.build).vmovups(
                            OperandX64::reg(inst.reg_x64),
                            luau_reg(vm_reg_op((*get_op_mut(inst, 0)))),
                        );
                    } else if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::VmConst
                    {
                        (*self.build).vmovups(
                            OperandX64::reg(inst.reg_x64),
                            luau_constant(vm_const_op((*get_op_mut(inst, 0)))),
                        );
                    } else if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Inst {
                        (*self.build).vmovups(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::mem(
                                SizeX64::xmmword,
                                RegisterX64::noreg,
                                1,
                                self.reg_op((*get_op_mut(inst, 0))),
                                addrOffset,
                            ),
                        );
                    } else {
                        CODEGEN_ASSERT!(false, "Unsupported instruction form");
                    }
                }
                IrCmd::LOAD_ENV => {
                    inst.reg_x64 = self.regs.alloc_reg(SizeX64::qword, index);

                    (*self.build).mov(OperandX64::reg(inst.reg_x64), sClosure);
                    (*self.build).mov(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::mem(
                            SizeX64::qword,
                            RegisterX64::noreg,
                            1,
                            inst.reg_x64,
                            (core::mem::offset_of!(Closure, env) as i32),
                        ),
                    );
                }
                IrCmd::GET_ARR_ADDR => {
                    if (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Inst {
                        inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                            SizeX64::qword,
                            index,
                            &[(*get_op_mut(inst, (((1) as u32) as u32)))],
                        );

                        if dword_reg(inst.reg_x64) != self.reg_op((*get_op_mut(inst, 1))) {
                            (*self.build).mov(
                                OperandX64::reg(dword_reg(inst.reg_x64)),
                                OperandX64::reg(self.reg_op((*get_op_mut(inst, 1)))),
                            );
                        }

                        (*self.build).shl(
                            OperandX64::reg(dword_reg(inst.reg_x64)),
                            OperandX64::imm(kTValueSizeLog2),
                        );
                        (*self.build).add(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::mem(
                                SizeX64::qword,
                                RegisterX64::noreg,
                                1,
                                self.reg_op((*get_op_mut(inst, 0))),
                                (core::mem::offset_of!(LuaTable, array) as i32),
                            ),
                        );
                    } else if (*get_op_mut(inst, (((1) as u32) as u32))).kind()
                        == IrOpKind::Constant
                    {
                        inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                            SizeX64::qword,
                            index,
                            &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                        );

                        (*self.build).mov(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::mem(
                                SizeX64::qword,
                                RegisterX64::noreg,
                                1,
                                self.reg_op((*get_op_mut(inst, 0))),
                                (core::mem::offset_of!(LuaTable, array) as i32),
                            ),
                        );

                        if self.int_op((*get_op_mut(inst, 1))) != 0 {
                            (*self.build).lea_operand_x_64_operand_x_64(
                                OperandX64::reg(inst.reg_x64),
                                OperandX64::mem(
                                    SizeX64::none,
                                    RegisterX64::noreg,
                                    1,
                                    inst.reg_x64,
                                    self.int_op((*get_op_mut(inst, 1)))
                                        * (core::mem::size_of::<TValue>() as i32),
                                ),
                            );
                        }
                    } else {
                        CODEGEN_ASSERT!(false, "Unsupported instruction form");
                    }
                }
                IrCmd::GET_SLOT_NODE_ADDR => {
                    inst.reg_x64 = self.regs.alloc_reg(SizeX64::qword, index);

                    let mut tmp = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };
                    tmp.alloc(SizeX64::qword);

                    get_table_node_at_cached_slot(
                        &mut *self.build,
                        tmp.reg,
                        inst.reg_x64,
                        self.reg_op((*get_op_mut(inst, 0))),
                        (((self.uint_op((*get_op_mut(inst, 1)))) as i32) as i32),
                    );
                }
                IrCmd::GET_HASH_NODE_ADDR => {
                    {
                        // Custom bit shift value can only be placed in RegisterX64::cl
                        let mut shiftTmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: self.regs.take_reg(RegisterX64::rcx, kInvalidInstIdx),
                        };

                        inst.reg_x64 = self.regs.alloc_reg(SizeX64::qword, index);

                        let mut tmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp.alloc(SizeX64::qword);

                        (*self.build).mov(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::mem(
                                SizeX64::qword,
                                RegisterX64::noreg,
                                1,
                                self.reg_op((*get_op_mut(inst, 0))),
                                (core::mem::offset_of!(LuaTable, node) as i32),
                            ),
                        );
                        (*self.build).mov(
                            OperandX64::reg(dword_reg(tmp.reg)),
                            OperandX64::imm((1) as i32),
                        );
                        (*self.build).mov(
                            OperandX64::reg(byte_reg(shiftTmp.reg)),
                            OperandX64::mem(
                                SizeX64::byte,
                                RegisterX64::noreg,
                                1,
                                self.reg_op((*get_op_mut(inst, 0))),
                                (core::mem::offset_of!(LuaTable, lsizenode) as i32),
                            ),
                        );
                        (*self.build).shl(
                            OperandX64::reg(dword_reg(tmp.reg)),
                            OperandX64::reg(byte_reg(shiftTmp.reg)),
                        );
                        (*self.build).dec(OperandX64::reg(dword_reg(tmp.reg)));
                        (*self.build).and_(
                            OperandX64::reg(dword_reg(tmp.reg)),
                            OperandX64::imm((self.uint_op((*get_op_mut(inst, 1)))) as i32),
                        );
                        (*self.build)
                            .shl(OperandX64::reg(tmp.reg), OperandX64::imm(kLuaNodeSizeLog2));
                        (*self.build).add(OperandX64::reg(inst.reg_x64), OperandX64::reg(tmp.reg));
                    };
                }
                IrCmd::GET_CLOSURE_UPVAL_ADDR => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::qword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );

                    if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Undef {
                        (*self.build).mov(OperandX64::reg(inst.reg_x64), sClosure);
                    } else {
                        let mut cl = self.reg_op((*get_op_mut(inst, 0)));
                        if inst.reg_x64 != cl {
                            (*self.build).mov(OperandX64::reg(inst.reg_x64), OperandX64::reg(cl));
                        }
                    }

                    (*self.build).add(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::imm(
                            (core::mem::offset_of!(Closure, inner) as i32)
                                + (core::mem::offset_of!(LClosure, uprefs) as i32)
                                + (core::mem::size_of::<TValue>() as i32)
                                    * vm_upvalue_op((*get_op_mut(inst, 1))) as i32,
                        ),
                    );
                }
                IrCmd::STORE_TAG => {
                    if (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Constant {
                        if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Inst {
                            (*self.build).mov(
                                OperandX64::mem(
                                    SizeX64::dword,
                                    RegisterX64::noreg,
                                    1,
                                    self.reg_op((*get_op_mut(inst, 0))),
                                    (core::mem::offset_of!(TValue, tt) as i32),
                                ),
                                OperandX64::imm((self.tag_op((*get_op_mut(inst, 1)))) as i32),
                            );
                        } else {
                            (*self.build).mov(
                                luau_reg_tag(vm_reg_op((*get_op_mut(inst, 0)))),
                                OperandX64::imm((self.tag_op((*get_op_mut(inst, 1)))) as i32),
                            );
                        }
                    } else {
                        CODEGEN_ASSERT!(false, "Unsupported instruction form");
                    }
                }
                IrCmd::STORE_POINTER => {
                    let mut valueLhs =
                        if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Inst {
                            OperandX64::mem(
                                SizeX64::qword,
                                RegisterX64::noreg,
                                1,
                                self.reg_op((*get_op_mut(inst, 0))),
                                (core::mem::offset_of!(TValue, value) as i32),
                            )
                        } else {
                            luau_reg_value(vm_reg_op((*get_op_mut(inst, 0))))
                        };

                    if (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Constant {
                        CODEGEN_ASSERT!(self.int_op((*get_op_mut(inst, 1))) == 0);
                        (*self.build).mov(valueLhs, OperandX64::imm((0) as i32));
                    } else if (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Inst {
                        (*self.build).mov(
                            valueLhs,
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 1)))),
                        );
                    } else {
                        CODEGEN_ASSERT!(false, "Unsupported instruction form");
                    }
                }
                IrCmd::STORE_EXTRA => {
                    if (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Constant {
                        if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Inst {
                            (*self.build).mov(
                                OperandX64::mem(
                                    SizeX64::dword,
                                    RegisterX64::noreg,
                                    1,
                                    self.reg_op((*get_op_mut(inst, 0))),
                                    (core::mem::offset_of!(TValue, extra) as i32),
                                ),
                                OperandX64::imm((self.int_op((*get_op_mut(inst, 1)))) as i32),
                            );
                        } else {
                            (*self.build).mov(
                                luau_reg_extra(vm_reg_op((*get_op_mut(inst, 0)))),
                                OperandX64::imm((self.int_op((*get_op_mut(inst, 1)))) as i32),
                            );
                        }
                    } else {
                        CODEGEN_ASSERT!(false, "Unsupported instruction form");
                    }
                }
                IrCmd::STORE_DOUBLE => {
                    let mut valueLhs =
                        if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Inst {
                            OperandX64::mem(
                                SizeX64::qword,
                                RegisterX64::noreg,
                                1,
                                self.reg_op((*get_op_mut(inst, 0))),
                                (core::mem::offset_of!(TValue, value) as i32),
                            )
                        } else {
                            luau_reg_value(vm_reg_op((*get_op_mut(inst, 0))))
                        };

                    if (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Constant {
                        let mut tmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp.alloc(SizeX64::xmmword);

                        (*self.build).vmovsd_operand_x_64_operand_x_64(
                            OperandX64::reg(tmp.reg),
                            (*self.build).f64(self.double_op((*get_op_mut(inst, 1)))),
                        );
                        (*self.build)
                            .vmovsd_operand_x_64_operand_x_64(valueLhs, OperandX64::reg(tmp.reg));
                    } else if (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Inst {
                        (*self.build).vmovsd_operand_x_64_operand_x_64(
                            valueLhs,
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 1)))),
                        );
                    } else {
                        CODEGEN_ASSERT!(false, "Unsupported instruction form");
                    }
                }
                IrCmd::STORE_INT => {
                    if (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Constant {
                        (*self.build).mov(
                            luau_reg_value_int(vm_reg_op((*get_op_mut(inst, 0)))),
                            OperandX64::imm((self.int_op((*get_op_mut(inst, 1)))) as i32),
                        );
                    } else if (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Inst {
                        (*self.build).mov(
                            luau_reg_value_int(vm_reg_op((*get_op_mut(inst, 0)))),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 1)))),
                        );
                    } else {
                        CODEGEN_ASSERT!(false, "Unsupported instruction form");
                    }
                }
                IrCmd::STORE_INT64 => {
                    if (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Constant {
                        let mut value = self.int64_op((*get_op_mut(inst, 1)));

                        // x64 mov r/m64, imm32 sign-extends
                        // otherwise we use register for values outside that range
                        if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
                            (*self.build).mov(
                                luau_reg_value_int_64(vm_reg_op((*get_op_mut(inst, 0)))),
                                OperandX64::imm(((value) as i32) as i32),
                            );
                        } else {
                            let mut tmp = ScopedRegX64 {
                                owner: &mut self.regs,
                                reg: RegisterX64::noreg,
                            };
                            tmp.alloc(SizeX64::qword);
                            (*self.build).mov64(tmp.reg, value);
                            (*self.build).mov(
                                luau_reg_value_int_64(vm_reg_op((*get_op_mut(inst, 0)))),
                                OperandX64::reg(tmp.reg),
                            );
                        }
                    } else if (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Inst {
                        (*self.build).mov(
                            luau_reg_value_int_64(vm_reg_op((*get_op_mut(inst, 0)))),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 1)))),
                        );
                    } else {
                        CODEGEN_ASSERT!(false, "Unsupported instruction form");
                    }
                }
                IrCmd::STORE_VECTOR => {
                    self.store_float(
                        luau_reg_value_vector(vm_reg_op((*get_op_mut(inst, 0))), 0),
                        (*get_op_mut(inst, 1)),
                    );
                    self.store_float(
                        luau_reg_value_vector(vm_reg_op((*get_op_mut(inst, 0))), 1),
                        (*get_op_mut(inst, 2)),
                    );
                    self.store_float(
                        luau_reg_value_vector(vm_reg_op((*get_op_mut(inst, 0))), 2),
                        (*get_op_mut(inst, 3)),
                    );

                    if HAS_OP_E!(inst) {
                        (*self.build).mov(
                            luau_reg_tag(vm_reg_op((*get_op_mut(inst, 0)))),
                            OperandX64::imm((self.tag_op((*get_op_mut(inst, 4)))) as i32),
                        );
                    }
                }
                IrCmd::STORE_TVALUE => {
                    let mut addrOffset = if HAS_OP_C!(inst) {
                        self.int_op((*get_op_mut(inst, 2)))
                    } else {
                        0
                    };

                    if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::VmReg {
                        (*self.build).vmovups(
                            luau_reg(vm_reg_op((*get_op_mut(inst, 0)))),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 1)))),
                        );
                    } else if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Inst {
                        (*self.build).vmovups(
                            OperandX64::mem(
                                SizeX64::xmmword,
                                RegisterX64::noreg,
                                1,
                                self.reg_op((*get_op_mut(inst, 0))),
                                addrOffset,
                            ),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 1)))),
                        );
                    } else {
                        CODEGEN_ASSERT!(false, "Unsupported instruction form");
                    }
                }
                IrCmd::STORE_SPLIT_TVALUE => {
                    {
                        let mut addrOffset = if HAS_OP_D!(inst) {
                            self.int_op((*get_op_mut(inst, 3)))
                        } else {
                            0
                        };

                        let mut tagLhs = if (*get_op_mut(inst, (((0) as u32) as u32))).kind()
                            == IrOpKind::Inst
                        {
                            OperandX64::mem(
                                SizeX64::dword,
                                RegisterX64::noreg,
                                1,
                                self.reg_op((*get_op_mut(inst, 0))),
                                (core::mem::offset_of!(TValue, tt) as i32) + addrOffset,
                            )
                        } else {
                            luau_reg_tag(vm_reg_op((*get_op_mut(inst, 0))))
                        };
                        (*self.build).mov(
                            tagLhs,
                            OperandX64::imm((self.tag_op((*get_op_mut(inst, 1)))) as i32),
                        );

                        if self.tag_op((*get_op_mut(inst, 1))) == lua_Type::LUA_TBOOLEAN as u8 {
                            let mut valueLhs = if (*get_op_mut(inst, (((0) as u32) as u32))).kind()
                                == IrOpKind::Inst
                            {
                                OperandX64::mem(
                                    SizeX64::dword,
                                    RegisterX64::noreg,
                                    1,
                                    self.reg_op((*get_op_mut(inst, 0))),
                                    (core::mem::offset_of!(TValue, value) as i32) + addrOffset,
                                )
                            } else {
                                luau_reg_value_int(vm_reg_op((*get_op_mut(inst, 0))))
                            };
                            (*self.build).mov(
                                valueLhs,
                                if (*get_op_mut(inst, 2)).kind() == IrOpKind::Constant {
                                    OperandX64::imm(self.int_op((*get_op_mut(inst, 2))))
                                } else {
                                    OperandX64::reg(self.reg_op((*get_op_mut(inst, 2))))
                                },
                            );
                        } else if self.tag_op((*get_op_mut(inst, 1))) == lua_Type::LUA_TNUMBER as u8
                        {
                            let mut valueLhs = if (*get_op_mut(inst, (((0) as u32) as u32))).kind()
                                == IrOpKind::Inst
                            {
                                OperandX64::mem(
                                    SizeX64::qword,
                                    RegisterX64::noreg,
                                    1,
                                    self.reg_op((*get_op_mut(inst, 0))),
                                    (core::mem::offset_of!(TValue, value) as i32) + addrOffset,
                                )
                            } else {
                                luau_reg_value(vm_reg_op((*get_op_mut(inst, 0))))
                            };

                            if (*get_op_mut(inst, (((2) as u32) as u32))).kind()
                                == IrOpKind::Constant
                            {
                                let mut tmp = ScopedRegX64 {
                                    owner: &mut self.regs,
                                    reg: RegisterX64::noreg,
                                };
                                tmp.alloc(SizeX64::xmmword);

                                (*self.build).vmovsd_operand_x_64_operand_x_64(
                                    OperandX64::reg(tmp.reg),
                                    (*self.build).f64(self.double_op((*get_op_mut(inst, 2)))),
                                );
                                (*self.build).vmovsd_operand_x_64_operand_x_64(
                                    valueLhs,
                                    OperandX64::reg(tmp.reg),
                                );
                            } else {
                                (*self.build).vmovsd_operand_x_64_operand_x_64(
                                    valueLhs,
                                    OperandX64::reg(self.reg_op((*get_op_mut(inst, 2)))),
                                );
                            }
                        } else if self.tag_op((*get_op_mut(inst, 1)))
                            == lua_Type::LUA_TINTEGER as u8
                        {
                            let mut valueLhs = if (*get_op_mut(inst, (((0) as u32) as u32))).kind()
                                == IrOpKind::Inst
                            {
                                OperandX64::mem(
                                    SizeX64::qword,
                                    RegisterX64::noreg,
                                    1,
                                    self.reg_op((*get_op_mut(inst, 0))),
                                    (core::mem::offset_of!(TValue, value) as i32) + addrOffset,
                                )
                            } else {
                                luau_reg_value_int_64(vm_reg_op((*get_op_mut(inst, 0))))
                            };

                            if (*get_op_mut(inst, (((2) as u32) as u32))).kind()
                                == IrOpKind::Constant
                            {
                                let mut value = self.int64_op((*get_op_mut(inst, 2)));

                                // x64 mov r/m64, imm32 sign-extends
                                if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
                                    (*self.build)
                                        .mov(valueLhs, OperandX64::imm(((value) as i32) as i32));
                                } else {
                                    let mut tmp = ScopedRegX64 {
                                        owner: &mut self.regs,
                                        reg: RegisterX64::noreg,
                                    };
                                    tmp.alloc(SizeX64::qword);
                                    (*self.build).mov64(tmp.reg, value);
                                    (*self.build).mov(valueLhs, OperandX64::reg(tmp.reg));
                                }
                            } else {
                                (*self.build).mov(
                                    valueLhs,
                                    OperandX64::reg(self.reg_op((*get_op_mut(inst, 2)))),
                                );
                            }
                        } else if is_gco((((self.tag_op((*get_op_mut(inst, 1)))) as u8) as u8)) {
                            let mut valueLhs = if (*get_op_mut(inst, (((0) as u32) as u32))).kind()
                                == IrOpKind::Inst
                            {
                                OperandX64::mem(
                                    SizeX64::qword,
                                    RegisterX64::noreg,
                                    1,
                                    self.reg_op((*get_op_mut(inst, 0))),
                                    (core::mem::offset_of!(TValue, value) as i32) + addrOffset,
                                )
                            } else {
                                luau_reg_value(vm_reg_op((*get_op_mut(inst, 0))))
                            };
                            (*self.build).mov(
                                valueLhs,
                                OperandX64::reg(self.reg_op((*get_op_mut(inst, 2)))),
                            );
                        } else {
                            CODEGEN_ASSERT!(false, "Unsupported instruction form");
                        }
                    }
                }
                IrCmd::ADD_INT => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::dword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );
                    let op0 = *get_op_mut(inst, 0);
                    let op1 = *get_op_mut(inst, 1);

                    if op0.kind() == IrOpKind::Constant {
                        (*self.build).lea_operand_x_64_operand_x_64(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::mem(
                                SizeX64::none,
                                RegisterX64::noreg,
                                1,
                                self.reg_op(op1),
                                self.int_op(op0),
                            ),
                        );
                    } else if op0.kind() == IrOpKind::Inst {
                        if inst.reg_x64 == self.reg_op(op0) {
                            if op1.kind() == IrOpKind::Inst {
                                (*self.build).add(
                                    OperandX64::reg(inst.reg_x64),
                                    OperandX64::reg(self.reg_op(op1)),
                                );
                            } else if self.int_op(op1) == 1 {
                                (*self.build).inc(OperandX64::reg(inst.reg_x64));
                            } else {
                                (*self.build).add(
                                    OperandX64::reg(inst.reg_x64),
                                    OperandX64::imm((self.int_op(op1)) as i32),
                                );
                            }
                        } else {
                            if op1.kind() == IrOpKind::Inst {
                                (*self.build).lea_operand_x_64_operand_x_64(
                                    OperandX64::reg(inst.reg_x64),
                                    OperandX64::mem(
                                        SizeX64::none,
                                        self.reg_op(op1),
                                        1,
                                        self.reg_op(op0),
                                        0,
                                    ),
                                );
                            } else {
                                (*self.build).lea_operand_x_64_operand_x_64(
                                    OperandX64::reg(inst.reg_x64),
                                    OperandX64::mem(
                                        SizeX64::none,
                                        RegisterX64::noreg,
                                        1,
                                        self.reg_op(op0),
                                        self.int_op(op1),
                                    ),
                                );
                            }
                        }
                    } else {
                        CODEGEN_ASSERT!(false, "Unsupported instruction form");
                    }
                }
                IrCmd::ADD_INT64 => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::qword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );
                    let op0 = *get_op_mut(inst, 0);
                    let op1 = *get_op_mut(inst, 1);

                    if op0.kind() == IrOpKind::Constant {
                        let mut value = self.int64_op(op0);

                        if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
                            (*self.build).lea_operand_x_64_operand_x_64(
                                OperandX64::reg(inst.reg_x64),
                                OperandX64::mem(
                                    SizeX64::none,
                                    RegisterX64::noreg,
                                    1,
                                    self.reg_op(op1),
                                    ((value) as i32),
                                ),
                            );
                        } else {
                            (*self.build).mov64(inst.reg_x64, value);
                            (*self.build).add(
                                OperandX64::reg(inst.reg_x64),
                                OperandX64::reg(self.reg_op(op1)),
                            );
                        }
                    } else if op0.kind() == IrOpKind::Inst {
                        if inst.reg_x64 == self.reg_op(op0) {
                            if op1.kind() == IrOpKind::Inst {
                                (*self.build).add(
                                    OperandX64::reg(inst.reg_x64),
                                    OperandX64::reg(self.reg_op(op1)),
                                );
                            } else if self.int64_op(op1) == 1 {
                                (*self.build).inc(OperandX64::reg(inst.reg_x64));
                            } else {
                                let mut value = self.int64_op(op1);

                                if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
                                    (*self.build).add(
                                        OperandX64::reg(inst.reg_x64),
                                        OperandX64::imm(((value) as i32) as i32),
                                    );
                                } else {
                                    let mut tmp = ScopedRegX64 {
                                        owner: &mut self.regs,
                                        reg: RegisterX64::noreg,
                                    };
                                    tmp.alloc(SizeX64::qword);
                                    (*self.build).mov64(tmp.reg, value);
                                    (*self.build).add(
                                        OperandX64::reg(inst.reg_x64),
                                        OperandX64::reg(tmp.reg),
                                    );
                                }
                            }
                        } else {
                            if op1.kind() == IrOpKind::Inst {
                                (*self.build).lea_operand_x_64_operand_x_64(
                                    OperandX64::reg(inst.reg_x64),
                                    OperandX64::mem(
                                        SizeX64::none,
                                        self.reg_op(op1),
                                        1,
                                        self.reg_op(op0),
                                        0,
                                    ),
                                );
                            } else {
                                let mut value = self.int64_op(op1);

                                if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
                                    (*self.build).lea_operand_x_64_operand_x_64(
                                        OperandX64::reg(inst.reg_x64),
                                        OperandX64::mem(
                                            SizeX64::none,
                                            RegisterX64::noreg,
                                            1,
                                            self.reg_op(op0),
                                            ((value) as i32),
                                        ),
                                    );
                                } else {
                                    (*self.build).mov64(inst.reg_x64, value);
                                    (*self.build).add(
                                        OperandX64::reg(inst.reg_x64),
                                        OperandX64::reg(self.reg_op(op0)),
                                    );
                                }
                            }
                        }
                    } else {
                        CODEGEN_ASSERT!(false, "Unsupported instruction form");
                    }
                }
                IrCmd::SUB_INT => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::dword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );
                    let op0 = *get_op_mut(inst, 0);
                    let op1 = *get_op_mut(inst, 1);

                    if op0.kind() == IrOpKind::Inst {
                        if op1.kind() == IrOpKind::Constant {
                            if inst.reg_x64 != self.reg_op(op0) {
                                (*self.build).lea_operand_x_64_operand_x_64(
                                    OperandX64::reg(inst.reg_x64),
                                    OperandX64::mem(
                                        SizeX64::none,
                                        RegisterX64::noreg,
                                        1,
                                        self.reg_op(op0),
                                        -self.int_op(op1),
                                    ),
                                );
                            } else {
                                (*self.build).sub(
                                    OperandX64::reg(inst.reg_x64),
                                    OperandX64::imm((self.int_op(op1)) as i32),
                                );
                            }
                        } else {
                            // If result reuses the source, we can subtract in place, otherwise we need to setup our initial value
                            if inst.reg_x64 != self.reg_op(op0) {
                                (*self.build).mov(
                                    OperandX64::reg(inst.reg_x64),
                                    OperandX64::reg(self.reg_op(op0)),
                                );
                            }

                            (*self.build).sub(
                                OperandX64::reg(inst.reg_x64),
                                OperandX64::reg(self.reg_op(op1)),
                            );
                        }
                    } else if op1.kind() == IrOpKind::Inst {
                        (*self.build).mov(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::imm((self.int_op(op0)) as i32),
                        );
                        (*self.build).sub(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(self.reg_op(op1)),
                        );
                    } else {
                        CODEGEN_ASSERT!(false, "Unsupported instruction form");
                    }
                }
                IrCmd::SUB_INT64 => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::qword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );
                    let op0 = *get_op_mut(inst, 0);
                    let op1 = *get_op_mut(inst, 1);

                    if op0.kind() == IrOpKind::Inst {
                        if op1.kind() == IrOpKind::Constant {
                            let mut value = self.int64_op(op1);

                            if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
                                if inst.reg_x64 != self.reg_op(op0) {
                                    (*self.build).lea_operand_x_64_operand_x_64(
                                        OperandX64::reg(inst.reg_x64),
                                        OperandX64::mem(
                                            SizeX64::none,
                                            RegisterX64::noreg,
                                            1,
                                            self.reg_op(op0),
                                            -((value) as i32),
                                        ),
                                    );
                                } else {
                                    (*self.build).sub(
                                        OperandX64::reg(inst.reg_x64),
                                        OperandX64::imm(((value) as i32) as i32),
                                    );
                                }
                            } else {
                                let mut tmp = ScopedRegX64 {
                                    owner: &mut self.regs,
                                    reg: RegisterX64::noreg,
                                };
                                tmp.alloc(SizeX64::qword);
                                (*self.build).mov64(tmp.reg, value);

                                if inst.reg_x64 != self.reg_op(op0) {
                                    (*self.build).mov(
                                        OperandX64::reg(inst.reg_x64),
                                        OperandX64::reg(self.reg_op(op0)),
                                    );
                                }

                                (*self.build)
                                    .sub(OperandX64::reg(inst.reg_x64), OperandX64::reg(tmp.reg));
                            }
                        } else {
                            // If result reuses the source, we can subtract in place, otherwise we need to setup our initial value
                            if inst.reg_x64 != self.reg_op(op0) {
                                (*self.build).mov(
                                    OperandX64::reg(inst.reg_x64),
                                    OperandX64::reg(self.reg_op(op0)),
                                );
                            }

                            (*self.build).sub(
                                OperandX64::reg(inst.reg_x64),
                                OperandX64::reg(self.reg_op(op1)),
                            );
                        }
                    } else if op1.kind() == IrOpKind::Inst {
                        (*self.build).mov64(inst.reg_x64, ((self.int64_op(op0)) as i64));
                        (*self.build).sub(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(self.reg_op(op1)),
                        );
                    } else {
                        CODEGEN_ASSERT!(false, "Unsupported instruction form");
                    }
                }
                IrCmd::SEXTI8_INT => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::dword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );

                    (*self.build).movsx(
                        inst.reg_x64,
                        OperandX64::reg(byte_reg(self.reg_op((*get_op_mut(inst, 0))))),
                    );
                }
                IrCmd::SEXTI16_INT => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::dword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );

                    (*self.build).movsx(
                        inst.reg_x64,
                        OperandX64::reg(word_reg(self.reg_op((*get_op_mut(inst, 0))))),
                    );
                }
                IrCmd::ADD_NUM => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[
                            (*get_op_mut(inst, (((0) as u32) as u32))),
                            (*get_op_mut(inst, (((1) as u32) as u32))),
                        ],
                    );

                    if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Constant {
                        let mut tmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp.alloc(SizeX64::xmmword);

                        (*self.build).vmovsd_operand_x_64_operand_x_64(
                            OperandX64::reg(tmp.reg),
                            self.mem_reg_double_op((*get_op_mut(inst, 0))),
                        );
                        (*self.build).vaddsd(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(tmp.reg),
                            self.mem_reg_double_op((*get_op_mut(inst, 1))),
                        );
                    } else {
                        (*self.build).vaddsd(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                            self.mem_reg_double_op((*get_op_mut(inst, 1))),
                        );
                    }
                }
                IrCmd::SUB_NUM => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[
                            (*get_op_mut(inst, (((0) as u32) as u32))),
                            (*get_op_mut(inst, (((1) as u32) as u32))),
                        ],
                    );

                    if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Constant {
                        let mut tmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp.alloc(SizeX64::xmmword);

                        (*self.build).vmovsd_operand_x_64_operand_x_64(
                            OperandX64::reg(tmp.reg),
                            self.mem_reg_double_op((*get_op_mut(inst, 0))),
                        );
                        (*self.build).vsubsd(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(tmp.reg),
                            self.mem_reg_double_op((*get_op_mut(inst, 1))),
                        );
                    } else {
                        (*self.build).vsubsd(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                            self.mem_reg_double_op((*get_op_mut(inst, 1))),
                        );
                    }
                }
                IrCmd::MUL_NUM => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[
                            (*get_op_mut(inst, (((0) as u32) as u32))),
                            (*get_op_mut(inst, (((1) as u32) as u32))),
                        ],
                    );

                    if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Constant {
                        let mut tmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp.alloc(SizeX64::xmmword);

                        (*self.build).vmovsd_operand_x_64_operand_x_64(
                            OperandX64::reg(tmp.reg),
                            self.mem_reg_double_op((*get_op_mut(inst, 0))),
                        );
                        (*self.build).vmulsd(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(tmp.reg),
                            self.mem_reg_double_op((*get_op_mut(inst, 1))),
                        );
                    } else {
                        (*self.build).vmulsd(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                            self.mem_reg_double_op((*get_op_mut(inst, 1))),
                        );
                    }
                }
                IrCmd::MUL_INT64 => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::qword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );

                    (*self.build).mov(
                        OperandX64::reg(inst.reg_x64),
                        self.mem_reg_int_64_op((*get_op_mut(inst, 0))),
                    );
                    (*self.build).imul_operand_x_64_operand_x_64(
                        OperandX64::reg(inst.reg_x64),
                        self.mem_reg_int_64_op((*get_op_mut(inst, 1))),
                    );
                }
                IrCmd::DIV_NUM => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[
                            (*get_op_mut(inst, (((0) as u32) as u32))),
                            (*get_op_mut(inst, (((1) as u32) as u32))),
                        ],
                    );

                    if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Constant {
                        let mut tmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp.alloc(SizeX64::xmmword);

                        (*self.build).vmovsd_operand_x_64_operand_x_64(
                            OperandX64::reg(tmp.reg),
                            self.mem_reg_double_op((*get_op_mut(inst, 0))),
                        );
                        (*self.build).vdivsd(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(tmp.reg),
                            self.mem_reg_double_op((*get_op_mut(inst, 1))),
                        );
                    } else {
                        (*self.build).vdivsd(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                            self.mem_reg_double_op((*get_op_mut(inst, 1))),
                        );
                    }
                }
                IrCmd::DIV_INT64 => {
                    {
                        // idiv clobbers RegisterX64::rax (quotient) and RegisterX64::rdx (remainder)
                        let mut divRax = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        divRax.alloc(SizeX64::dword);
                        let mut divRdx = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        divRdx.alloc(SizeX64::dword);
                        divRax.take(RegisterX64::rax);
                        divRdx.take(RegisterX64::rdx);

                        (*self.build).mov(
                            OperandX64::reg(RegisterX64::rax),
                            self.mem_reg_int_64_op((*get_op_mut(inst, 0))),
                        );
                        (*self.build).cqo(); // sign-extend RAX into RDX:RAX
                        (*self.build).idiv(self.mem_reg_int_64_op((*get_op_mut(inst, 1))));

                        inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                            SizeX64::qword,
                            index,
                            &[
                                (*get_op_mut(inst, (((0) as u32) as u32))),
                                (*get_op_mut(inst, (((1) as u32) as u32))),
                            ],
                        );
                        (*self.build).mov(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(RegisterX64::rax),
                        );
                    }
                }
                IrCmd::IDIV_NUM => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[
                            (*get_op_mut(inst, (((0) as u32) as u32))),
                            (*get_op_mut(inst, (((1) as u32) as u32))),
                        ],
                    );

                    if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Constant {
                        let mut tmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp.alloc(SizeX64::xmmword);

                        (*self.build).vmovsd_operand_x_64_operand_x_64(
                            OperandX64::reg(tmp.reg),
                            self.mem_reg_double_op((*get_op_mut(inst, 0))),
                        );
                        (*self.build).vdivsd(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(tmp.reg),
                            self.mem_reg_double_op((*get_op_mut(inst, 1))),
                        );
                    } else {
                        (*self.build).vdivsd(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                            self.mem_reg_double_op((*get_op_mut(inst, 1))),
                        );
                    }
                    (*self.build).vroundsd(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(inst.reg_x64),
                        RoundingModeX64::RoundToNegativeInfinity,
                    );
                }
                IrCmd::IDIV_INT64 => {
                    {
                        // idiv clobbers RegisterX64::rax (quotient) and RegisterX64::rdx (remainder)
                        let mut divRax = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        divRax.alloc(SizeX64::dword);
                        divRax.take(RegisterX64::rax);
                        let mut divRdx = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        divRdx.alloc(SizeX64::dword);
                        divRdx.take(RegisterX64::rdx);
                        let mut tempB = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tempB.alloc(SizeX64::qword);

                        (*self.build).mov(
                            OperandX64::reg(tempB.reg),
                            self.mem_reg_int_64_op((*get_op_mut(inst, 1))),
                        );

                        // idiv divides RDX:RAX by operand; quotient in RAX, remainder in RDX
                        (*self.build).mov(
                            OperandX64::reg(RegisterX64::rax),
                            self.mem_reg_int_64_op((*get_op_mut(inst, 0))),
                        );
                        (*self.build).cqo(); // sign-extend RAX into RDX:RAX
                        (*self.build).idiv(OperandX64::reg(tempB.reg));

                        inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                            SizeX64::qword,
                            index,
                            &[
                                (*get_op_mut(inst, (((0) as u32) as u32))),
                                (*get_op_mut(inst, (((1) as u32) as u32))),
                            ],
                        );
                        (*self.build).mov(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(RegisterX64::rax),
                        ); // start with truncated quotient

                        let mut done = Label::default();
                        (*self.build).test(
                            OperandX64::reg(RegisterX64::rdx),
                            OperandX64::reg(RegisterX64::rdx),
                        );
                        (*self.build).jcc(ConditionX64::Equal, &mut done); // remainder == 0, no adjustment needed

                        (*self.build).xor_(
                            OperandX64::reg(RegisterX64::rdx),
                            OperandX64::reg(tempB.reg),
                        );
                        (*self.build).jcc(ConditionX64::GreaterEqual, &mut done); // same sign, no adjustment

                        (*self.build)
                            .sub(OperandX64::reg(inst.reg_x64), OperandX64::imm((1) as i32)); // floor adjustment
                        (*self.build).set_label(&mut done);
                    }
                }
                IrCmd::UDIV_INT64 => {
                    {
                        // div clobbers RegisterX64::rax (quotient) and RegisterX64::rdx (remainder)
                        let mut divRax = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        divRax.alloc(SizeX64::dword);
                        let mut divRdx = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        divRdx.alloc(SizeX64::dword);
                        divRax.take(RegisterX64::rax);
                        divRdx.take(RegisterX64::rdx);

                        (*self.build).mov(
                            OperandX64::reg(RegisterX64::rax),
                            self.mem_reg_int_64_op((*get_op_mut(inst, 0))),
                        );
                        (*self.build).xor_(
                            OperandX64::reg(RegisterX64::rdx),
                            OperandX64::reg(RegisterX64::rdx),
                        ); // zero-extend RAX into RDX:RAX
                        (*self.build).div(self.mem_reg_int_64_op((*get_op_mut(inst, 1))));

                        inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                            SizeX64::qword,
                            index,
                            &[
                                (*get_op_mut(inst, (((0) as u32) as u32))),
                                (*get_op_mut(inst, (((1) as u32) as u32))),
                            ],
                        );
                        (*self.build).mov(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(RegisterX64::rax),
                        );
                    }
                }
                IrCmd::REM_INT64 => {
                    {
                        // idiv clobbers RegisterX64::rax (quotient) and RegisterX64::rdx (remainder)
                        let mut divRax = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        divRax.alloc(SizeX64::dword);
                        let mut divRdx = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        divRdx.alloc(SizeX64::dword);
                        divRax.take(RegisterX64::rax);
                        divRdx.take(RegisterX64::rdx);
                        let mut tempB = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tempB.alloc(SizeX64::qword);
                        let mut tempA = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tempA.alloc(SizeX64::qword);
                        (*self.build).mov(
                            OperandX64::reg(tempA.reg),
                            self.mem_reg_int_64_op((*get_op_mut(inst, 0))),
                        );
                        (*self.build).mov(
                            OperandX64::reg(tempB.reg),
                            self.mem_reg_int_64_op((*get_op_mut(inst, 1))),
                        );

                        // guard against dividend == i64::MIN && divisor == -1 (signed overflow)
                        // if that occurs, we must return 0
                        let mut skip = Label::default();
                        let mut done = Label::default();

                        (*self.build).cmp(OperandX64::reg(tempB.reg), OperandX64::imm((-1) as i32));
                        (*self.build).jcc(ConditionX64::NotEqual, &mut skip);

                        let mut tmpMin = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmpMin.alloc(SizeX64::qword);
                        (*self.build).mov(
                            OperandX64::reg(RegisterX64::rdx),
                            OperandX64::imm((0) as i32),
                        );
                        (*self.build).mov64(tmpMin.reg, i64::MIN);
                        (*self.build).cmp(OperandX64::reg(tempA.reg), OperandX64::reg(tmpMin.reg));
                        (*self.build).jcc(ConditionX64::Equal, &mut done);

                        (*self.build).set_label(&mut skip);

                        (*self.build).mov(
                            OperandX64::reg(RegisterX64::rax),
                            OperandX64::reg(tempA.reg),
                        );
                        (*self.build).cqo(); // sign-extend RAX into RDX:RAX
                        (*self.build).idiv(OperandX64::reg(tempB.reg));

                        (*self.build).set_label(&mut done);
                        inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                            SizeX64::qword,
                            index,
                            &[
                                (*get_op_mut(inst, (((0) as u32) as u32))),
                                (*get_op_mut(inst, (((1) as u32) as u32))),
                            ],
                        );
                        (*self.build).mov(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(RegisterX64::rdx),
                        );
                    }
                }
                IrCmd::UREM_INT64 => {
                    {
                        // div clobbers RegisterX64::rax (quotient) and RegisterX64::rdx (remainder)
                        let mut divRax = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        divRax.alloc(SizeX64::dword);
                        let mut divRdx = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        divRdx.alloc(SizeX64::dword);
                        divRax.take(RegisterX64::rax);
                        divRdx.take(RegisterX64::rdx);

                        (*self.build).mov(
                            OperandX64::reg(RegisterX64::rax),
                            self.mem_reg_int_64_op((*get_op_mut(inst, 0))),
                        );
                        (*self.build).xor_(
                            OperandX64::reg(RegisterX64::rdx),
                            OperandX64::reg(RegisterX64::rdx),
                        ); // zero-extend RAX into RDX:RAX
                        (*self.build).div(self.mem_reg_int_64_op((*get_op_mut(inst, 1))));

                        inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                            SizeX64::qword,
                            index,
                            &[
                                (*get_op_mut(inst, (((0) as u32) as u32))),
                                (*get_op_mut(inst, (((1) as u32) as u32))),
                            ],
                        );
                        (*self.build).mov(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(RegisterX64::rdx),
                        );
                    }
                }
                IrCmd::MULADD_NUM => {
                    if ((*self.build).features & FeaturesX64::Feature_FMA3 as u32) != 0 {
                        if (*get_op_mut(inst, (((0) as u32) as u32))).kind() != IrOpKind::Inst {
                            inst.reg_x64 = self.regs.alloc_reg(SizeX64::xmmword, index);
                            (*self.build).vmovsd_operand_x_64_operand_x_64(
                                OperandX64::reg(inst.reg_x64),
                                self.mem_reg_double_op((*get_op_mut(inst, 0))),
                            );
                        } else {
                            inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                                SizeX64::xmmword,
                                index,
                                &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                            );
                            let mut aReg = self.reg_op((*get_op_mut(inst, 0)));
                            if inst.reg_x64 != aReg {
                                (*self.build)
                                    .vmovupd(OperandX64::reg(inst.reg_x64), OperandX64::reg(aReg));
                            }
                        }

                        let mut optBTmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        optBTmp.alloc(SizeX64::dword);
                        let mut bReg = RegisterX64::noreg;

                        if (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Constant {
                            optBTmp.alloc(SizeX64::xmmword);

                            (*self.build).vmovsd_operand_x_64_operand_x_64(
                                OperandX64::reg(optBTmp.reg),
                                self.mem_reg_double_op((*get_op_mut(inst, 1))),
                            );
                            bReg = optBTmp.reg;
                        } else {
                            bReg = self.reg_op((*get_op_mut(inst, 1)));
                        }

                        (*self.build).vfmadd213pd(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(bReg),
                            self.mem_reg_double_op((*get_op_mut(inst, 2))),
                        );
                    } else {
                        inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                            SizeX64::xmmword,
                            index,
                            &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                        );

                        if (*get_op_mut(inst, (((0) as u32) as u32))).kind() != IrOpKind::Inst
                            && (*get_op_mut(inst, (((1) as u32) as u32))).kind() != IrOpKind::Inst
                        {
                            (*self.build).vmovsd_operand_x_64_operand_x_64(
                                OperandX64::reg(inst.reg_x64),
                                self.mem_reg_double_op((*get_op_mut(inst, 0))),
                            );
                            (*self.build).vmulsd(
                                OperandX64::reg(inst.reg_x64),
                                OperandX64::reg(inst.reg_x64),
                                self.mem_reg_double_op((*get_op_mut(inst, 1))),
                            );
                        } else if (*get_op_mut(inst, (((0) as u32) as u32))).kind()
                            == IrOpKind::Inst
                        {
                            (*self.build).vmulsd(
                                OperandX64::reg(inst.reg_x64),
                                OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                                self.mem_reg_double_op((*get_op_mut(inst, 1))),
                            );
                        } else {
                            CODEGEN_ASSERT!(
                                (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Inst
                            );
                            (*self.build).vmulsd(
                                OperandX64::reg(inst.reg_x64),
                                OperandX64::reg(self.reg_op((*get_op_mut(inst, 1)))),
                                self.mem_reg_double_op((*get_op_mut(inst, 0))),
                            );
                        }

                        (*self.build).vaddsd(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(inst.reg_x64),
                            self.mem_reg_double_op((*get_op_mut(inst, 2))),
                        );
                    }
                }
                IrCmd::MOD_NUM => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[
                            (*get_op_mut(inst, (((0) as u32) as u32))),
                            (*get_op_mut(inst, (((1) as u32) as u32))),
                        ],
                    );

                    let mut optLhsTmp = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };
                    optLhsTmp.alloc(SizeX64::dword);
                    let mut lhs = RegisterX64::noreg;

                    if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Constant {
                        optLhsTmp.alloc(SizeX64::xmmword);

                        (*self.build).vmovsd_operand_x_64_operand_x_64(
                            OperandX64::reg(optLhsTmp.reg),
                            self.mem_reg_double_op((*get_op_mut(inst, 0))),
                        );
                        lhs = optLhsTmp.reg;
                    } else {
                        lhs = self.reg_op((*get_op_mut(inst, 0)));
                    }

                    if (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Inst {
                        let mut tmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp.alloc(SizeX64::xmmword);

                        (*self.build).vdivsd(
                            OperandX64::reg(tmp.reg),
                            OperandX64::reg(lhs),
                            self.mem_reg_double_op((*get_op_mut(inst, 1))),
                        );
                        (*self.build).vroundsd(
                            OperandX64::reg(tmp.reg),
                            OperandX64::reg(tmp.reg),
                            OperandX64::reg(tmp.reg),
                            RoundingModeX64::RoundToNegativeInfinity,
                        );
                        (*self.build).vmulsd(
                            OperandX64::reg(tmp.reg),
                            OperandX64::reg(tmp.reg),
                            self.mem_reg_double_op((*get_op_mut(inst, 1))),
                        );
                        (*self.build).vsubsd(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(lhs),
                            OperandX64::reg(tmp.reg),
                        );
                    } else {
                        let mut tmp1 = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp1.alloc(SizeX64::xmmword);
                        let mut tmp2 = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp2.alloc(SizeX64::xmmword);

                        (*self.build).vmovsd_operand_x_64_operand_x_64(
                            OperandX64::reg(tmp1.reg),
                            self.mem_reg_double_op((*get_op_mut(inst, 1))),
                        );
                        (*self.build).vdivsd(
                            OperandX64::reg(tmp2.reg),
                            OperandX64::reg(lhs),
                            OperandX64::reg(tmp1.reg),
                        );
                        (*self.build).vroundsd(
                            OperandX64::reg(tmp2.reg),
                            OperandX64::reg(tmp2.reg),
                            OperandX64::reg(tmp2.reg),
                            RoundingModeX64::RoundToNegativeInfinity,
                        );
                        (*self.build).vmulsd(
                            OperandX64::reg(tmp1.reg),
                            OperandX64::reg(tmp2.reg),
                            OperandX64::reg(tmp1.reg),
                        );
                        (*self.build).vsubsd(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(lhs),
                            OperandX64::reg(tmp1.reg),
                        );
                    }
                }
                IrCmd::MOD_INT64 => {
                    {
                        // idiv clobbers RegisterX64::rax (quotient) and RegisterX64::rdx (remainder)
                        let mut divRax = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        divRax.alloc(SizeX64::dword);
                        divRax.take(RegisterX64::rax);
                        let mut divRdx = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        divRdx.alloc(SizeX64::dword);
                        divRdx.take(RegisterX64::rdx);
                        let mut tempB = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tempB.alloc(SizeX64::qword);
                        let mut tempA = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tempA.alloc(SizeX64::qword);
                        (*self.build).mov(
                            OperandX64::reg(tempA.reg),
                            self.mem_reg_int_64_op((*get_op_mut(inst, 0))),
                        );
                        (*self.build).mov(
                            OperandX64::reg(tempB.reg),
                            self.mem_reg_int_64_op((*get_op_mut(inst, 1))),
                        );

                        inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                            SizeX64::qword,
                            index,
                            &[
                                (*get_op_mut(inst, (((0) as u32) as u32))),
                                (*get_op_mut(inst, (((1) as u32) as u32))),
                            ],
                        );

                        // guard against dividend == i64::MIN && divisor == -1 (signed overflow)
                        // if that occurs, we must return 0
                        let mut skip = Label::default();
                        let mut done = Label::default();

                        (*self.build).cmp(OperandX64::reg(tempB.reg), OperandX64::imm((-1) as i32));
                        (*self.build).jcc(ConditionX64::NotEqual, &mut skip);

                        let mut tmpMin = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmpMin.alloc(SizeX64::qword);
                        (*self.build)
                            .mov(OperandX64::reg(inst.reg_x64), OperandX64::imm((0) as i32));
                        (*self.build).mov64(tmpMin.reg, i64::MIN);
                        (*self.build).cmp(OperandX64::reg(tempA.reg), OperandX64::reg(tmpMin.reg));
                        (*self.build).jcc(ConditionX64::Equal, &mut done);

                        (*self.build).set_label(&mut skip);

                        (*self.build).mov(
                            OperandX64::reg(RegisterX64::rax),
                            OperandX64::reg(tempA.reg),
                        );
                        (*self.build).cqo();
                        (*self.build).idiv(OperandX64::reg(tempB.reg));

                        (*self.build).mov(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(RegisterX64::rdx),
                        );

                        (*self.build).test(
                            OperandX64::reg(RegisterX64::rdx),
                            OperandX64::reg(RegisterX64::rdx),
                        );
                        (*self.build).jcc(ConditionX64::Equal, &mut done);

                        (*self.build).xor_(
                            OperandX64::reg(RegisterX64::rdx),
                            OperandX64::reg(tempB.reg),
                        );
                        (*self.build).jcc(ConditionX64::GreaterEqual, &mut done);

                        (*self.build)
                            .add(OperandX64::reg(inst.reg_x64), OperandX64::reg(tempB.reg));
                        (*self.build).set_label(&mut done);
                    }
                }
                IrCmd::MIN_NUM => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[
                            (*get_op_mut(inst, (((0) as u32) as u32))),
                            (*get_op_mut(inst, (((1) as u32) as u32))),
                        ],
                    );

                    if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Constant {
                        let mut tmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp.alloc(SizeX64::xmmword);

                        (*self.build).vmovsd_operand_x_64_operand_x_64(
                            OperandX64::reg(tmp.reg),
                            self.mem_reg_double_op((*get_op_mut(inst, 0))),
                        );
                        (*self.build).vminsd(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(tmp.reg),
                            self.mem_reg_double_op((*get_op_mut(inst, 1))),
                        );
                    } else {
                        (*self.build).vminsd(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                            self.mem_reg_double_op((*get_op_mut(inst, 1))),
                        );
                    }
                }
                IrCmd::MAX_NUM => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[
                            (*get_op_mut(inst, (((0) as u32) as u32))),
                            (*get_op_mut(inst, (((1) as u32) as u32))),
                        ],
                    );

                    if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Constant {
                        let mut tmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp.alloc(SizeX64::xmmword);

                        (*self.build).vmovsd_operand_x_64_operand_x_64(
                            OperandX64::reg(tmp.reg),
                            self.mem_reg_double_op((*get_op_mut(inst, 0))),
                        );
                        (*self.build).vmaxsd(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(tmp.reg),
                            self.mem_reg_double_op((*get_op_mut(inst, 1))),
                        );
                    } else {
                        (*self.build).vmaxsd(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                            self.mem_reg_double_op((*get_op_mut(inst, 1))),
                        );
                    }
                }
                IrCmd::UNM_NUM => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );

                    (*self.build).vxorpd(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                        (*self.build).f64(-0.0),
                    );
                }
                IrCmd::FLOOR_NUM => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );

                    (*self.build).vroundsd(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(inst.reg_x64),
                        self.mem_reg_double_op((*get_op_mut(inst, 0))),
                        RoundingModeX64::RoundToNegativeInfinity,
                    );
                }
                IrCmd::CEIL_NUM => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );

                    (*self.build).vroundsd(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(inst.reg_x64),
                        self.mem_reg_double_op((*get_op_mut(inst, 0))),
                        RoundingModeX64::RoundToPositiveInfinity,
                    );
                }
                IrCmd::ROUND_NUM => {
                    {
                        inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                            SizeX64::xmmword,
                            index,
                            &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                        );

                        let mut tmp1 = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp1.alloc(SizeX64::xmmword);
                        let mut tmp2 = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp2.alloc(SizeX64::xmmword);

                        if (*get_op_mut(inst, (((0) as u32) as u32))).kind() != IrOpKind::Inst {
                            (*self.build).vmovsd_operand_x_64_operand_x_64(
                                OperandX64::reg(inst.reg_x64),
                                self.mem_reg_double_op((*get_op_mut(inst, 0))),
                            );
                        } else if self.reg_op((*get_op_mut(inst, 0))) != inst.reg_x64 {
                            (*self.build).vmovsd_operand_x_64_operand_x_64_operand_x_64(
                                OperandX64::reg(inst.reg_x64),
                                OperandX64::reg(inst.reg_x64),
                                OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                            );
                        }

                        (*self.build).vandpd(
                            OperandX64::reg(tmp1.reg),
                            OperandX64::reg(inst.reg_x64),
                            (*self.build).f64x2(-0.0, -0.0),
                        );
                        (*self.build).vmovsd_operand_x_64_operand_x_64(
                            OperandX64::reg(tmp2.reg),
                            (*self.build).i64(0x3fdfffffffffffff),
                        ); // 0.49999999999999994
                        (*self.build).vorpd(
                            OperandX64::reg(tmp1.reg),
                            OperandX64::reg(tmp1.reg),
                            OperandX64::reg(tmp2.reg),
                        );
                        (*self.build).vaddsd(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(tmp1.reg),
                        );
                        (*self.build).vroundsd(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(inst.reg_x64),
                            RoundingModeX64::RoundToZero,
                        );
                    }
                }
                IrCmd::SQRT_NUM => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );

                    (*self.build).vsqrtsd(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(inst.reg_x64),
                        self.mem_reg_double_op((*get_op_mut(inst, 0))),
                    );
                }
                IrCmd::ABS_NUM => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );

                    if (*get_op_mut(inst, (((0) as u32) as u32))).kind() != IrOpKind::Inst {
                        (*self.build).vmovsd_operand_x_64_operand_x_64(
                            OperandX64::reg(inst.reg_x64),
                            self.mem_reg_double_op((*get_op_mut(inst, 0))),
                        );
                    } else if self.reg_op((*get_op_mut(inst, 0))) != inst.reg_x64 {
                        (*self.build).vmovsd_operand_x_64_operand_x_64_operand_x_64(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                        );
                    }

                    (*self.build).vandpd(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(inst.reg_x64),
                        (*self.build).i64(!(1 << 63)),
                    );
                }
                IrCmd::SIGN_NUM => {
                    {
                        inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                            SizeX64::xmmword,
                            index,
                            &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                        );

                        let mut tmp0 = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp0.alloc(SizeX64::xmmword);
                        let mut tmp1 = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp1.alloc(SizeX64::xmmword);
                        let mut tmp2 = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp2.alloc(SizeX64::xmmword);

                        (*self.build).vxorpd(
                            OperandX64::reg(tmp0.reg),
                            OperandX64::reg(tmp0.reg),
                            OperandX64::reg(tmp0.reg),
                        );

                        // Set tmp1 to -1 if arg < 0, else 0
                        (*self.build).vcmpltsd(
                            OperandX64::reg(tmp1.reg),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                            OperandX64::reg(tmp0.reg),
                        );
                        (*self.build).vmovsd_operand_x_64_operand_x_64(
                            OperandX64::reg(tmp2.reg),
                            (*self.build).f64(-1.0),
                        );
                        (*self.build).vandpd(
                            OperandX64::reg(tmp1.reg),
                            OperandX64::reg(tmp1.reg),
                            OperandX64::reg(tmp2.reg),
                        );

                        // Set mask bit to 1 if 0 < arg, else 0
                        (*self.build).vcmpltsd(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(tmp0.reg),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                        );

                        // Result = if (mask-bit == 1) { 1.0 } else { tmp1
                        // If arg < 0 then tmp1 is -1 and mask-bit is 0 }, result is -1
                        // If arg == 0 then tmp1 is 0 and mask-bit is 0, result is 0
                        // If arg > 0 then tmp1 is 0 and mask-bit is 1, result is 1
                        (*self.build).vblendvpd(
                            inst.reg_x64,
                            tmp1.reg,
                            (*self.build).f64x2(1.0, 1.0),
                            inst.reg_x64,
                        );
                    }
                }
                IrCmd::ADD_FLOAT => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[
                            (*get_op_mut(inst, (((0) as u32) as u32))),
                            (*get_op_mut(inst, (((1) as u32) as u32))),
                        ],
                    );

                    if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Constant {
                        let mut tmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp.alloc(SizeX64::xmmword);

                        (*self.build).vmovss_operand_x_64_operand_x_64(
                            OperandX64::reg(tmp.reg),
                            self.mem_reg_float_op((*get_op_mut(inst, 0))),
                        );
                        (*self.build).vaddss(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(tmp.reg),
                            self.mem_reg_float_op((*get_op_mut(inst, 1))),
                        );
                    } else {
                        (*self.build).vaddss(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                            self.mem_reg_float_op((*get_op_mut(inst, 1))),
                        );
                    }
                }
                IrCmd::SUB_FLOAT => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[
                            (*get_op_mut(inst, (((0) as u32) as u32))),
                            (*get_op_mut(inst, (((1) as u32) as u32))),
                        ],
                    );

                    if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Constant {
                        let mut tmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp.alloc(SizeX64::xmmword);

                        (*self.build).vmovss_operand_x_64_operand_x_64(
                            OperandX64::reg(tmp.reg),
                            self.mem_reg_float_op((*get_op_mut(inst, 0))),
                        );
                        (*self.build).vsubss(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(tmp.reg),
                            self.mem_reg_float_op((*get_op_mut(inst, 1))),
                        );
                    } else {
                        (*self.build).vsubss(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                            self.mem_reg_float_op((*get_op_mut(inst, 1))),
                        );
                    }
                }
                IrCmd::MUL_FLOAT => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[
                            (*get_op_mut(inst, (((0) as u32) as u32))),
                            (*get_op_mut(inst, (((1) as u32) as u32))),
                        ],
                    );

                    if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Constant {
                        let mut tmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp.alloc(SizeX64::xmmword);

                        (*self.build).vmovss_operand_x_64_operand_x_64(
                            OperandX64::reg(tmp.reg),
                            self.mem_reg_float_op((*get_op_mut(inst, 0))),
                        );
                        (*self.build).vmulss(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(tmp.reg),
                            self.mem_reg_float_op((*get_op_mut(inst, 1))),
                        );
                    } else {
                        (*self.build).vmulss(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                            self.mem_reg_float_op((*get_op_mut(inst, 1))),
                        );
                    }
                }
                IrCmd::DIV_FLOAT => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[
                            (*get_op_mut(inst, (((0) as u32) as u32))),
                            (*get_op_mut(inst, (((1) as u32) as u32))),
                        ],
                    );

                    if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Constant {
                        let mut tmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp.alloc(SizeX64::xmmword);

                        (*self.build).vmovss_operand_x_64_operand_x_64(
                            OperandX64::reg(tmp.reg),
                            self.mem_reg_float_op((*get_op_mut(inst, 0))),
                        );
                        (*self.build).vdivss(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(tmp.reg),
                            self.mem_reg_float_op((*get_op_mut(inst, 1))),
                        );
                    } else {
                        (*self.build).vdivss(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                            self.mem_reg_float_op((*get_op_mut(inst, 1))),
                        );
                    }
                }
                IrCmd::MIN_FLOAT => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[
                            (*get_op_mut(inst, (((0) as u32) as u32))),
                            (*get_op_mut(inst, (((1) as u32) as u32))),
                        ],
                    );

                    if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Constant {
                        let mut tmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp.alloc(SizeX64::xmmword);

                        (*self.build).vmovss_operand_x_64_operand_x_64(
                            OperandX64::reg(tmp.reg),
                            self.mem_reg_float_op((*get_op_mut(inst, 0))),
                        );
                        (*self.build).vminss(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(tmp.reg),
                            self.mem_reg_float_op((*get_op_mut(inst, 1))),
                        );
                    } else {
                        (*self.build).vminss(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                            self.mem_reg_float_op((*get_op_mut(inst, 1))),
                        );
                    }
                }
                IrCmd::MAX_FLOAT => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[
                            (*get_op_mut(inst, (((0) as u32) as u32))),
                            (*get_op_mut(inst, (((1) as u32) as u32))),
                        ],
                    );

                    if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Constant {
                        let mut tmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp.alloc(SizeX64::xmmword);

                        (*self.build).vmovss_operand_x_64_operand_x_64(
                            OperandX64::reg(tmp.reg),
                            self.mem_reg_float_op((*get_op_mut(inst, 0))),
                        );
                        (*self.build).vmaxss(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(tmp.reg),
                            self.mem_reg_float_op((*get_op_mut(inst, 1))),
                        );
                    } else {
                        (*self.build).vmaxss(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                            self.mem_reg_float_op((*get_op_mut(inst, 1))),
                        );
                    }
                }
                IrCmd::UNM_FLOAT => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );

                    (*self.build).vxorps(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                        (*self.build).f32(-0.0),
                    );
                }
                IrCmd::FLOOR_FLOAT => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );

                    (*self.build).vroundss(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(inst.reg_x64),
                        self.mem_reg_float_op((*get_op_mut(inst, 0))),
                        RoundingModeX64::RoundToNegativeInfinity,
                    );
                }
                IrCmd::CEIL_FLOAT => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );

                    (*self.build).vroundss(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(inst.reg_x64),
                        self.mem_reg_float_op((*get_op_mut(inst, 0))),
                        RoundingModeX64::RoundToPositiveInfinity,
                    );
                }
                IrCmd::SQRT_FLOAT => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );

                    (*self.build).vsqrtss(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(inst.reg_x64),
                        self.mem_reg_float_op((*get_op_mut(inst, 0))),
                    );
                }
                IrCmd::ABS_FLOAT => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );

                    if (*get_op_mut(inst, (((0) as u32) as u32))).kind() != IrOpKind::Inst {
                        (*self.build).vmovss_operand_x_64_operand_x_64(
                            OperandX64::reg(inst.reg_x64),
                            self.mem_reg_float_op((*get_op_mut(inst, 0))),
                        );
                    } else if self.reg_op((*get_op_mut(inst, 0))) != inst.reg_x64 {
                        (*self.build).vmovss_operand_x_64_operand_x_64_operand_x_64(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                        );
                    }

                    (*self.build).vandps(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(inst.reg_x64),
                        (*self.build).i32(0x7fffffff),
                    );
                }
                IrCmd::SIGN_FLOAT => {
                    {
                        inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                            SizeX64::xmmword,
                            index,
                            &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                        );

                        let mut tmp0 = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp0.alloc(SizeX64::xmmword);
                        let mut tmp1 = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp1.alloc(SizeX64::xmmword);
                        let mut tmp2 = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp2.alloc(SizeX64::xmmword);

                        (*self.build).vxorps(
                            OperandX64::reg(tmp0.reg),
                            OperandX64::reg(tmp0.reg),
                            OperandX64::reg(tmp0.reg),
                        );

                        // Set tmp1 to -1 if arg < 0, else 0
                        (*self.build).vcmpltss(
                            OperandX64::reg(tmp1.reg),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                            OperandX64::reg(tmp0.reg),
                        );
                        (*self.build).vmovss_operand_x_64_operand_x_64(
                            OperandX64::reg(tmp2.reg),
                            (*self.build).f32(-1.0),
                        );
                        (*self.build).vandps(
                            OperandX64::reg(tmp1.reg),
                            OperandX64::reg(tmp1.reg),
                            OperandX64::reg(tmp2.reg),
                        );

                        // Set mask bit to 1 if 0 < arg, else 0
                        (*self.build).vcmpltss(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(tmp0.reg),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                        );

                        // Result = if (mask-bit == 1) { 1.0 } else { tmp1
                        // If arg < 0 then tmp1 is -1 and mask-bit is 0 }, result is -1
                        // If arg == 0 then tmp1 is 0 and mask-bit is 0, result is 0
                        // If arg > 0 then tmp1 is 0 and mask-bit is 1, result is 1
                        (*self.build).vblendvps(
                            inst.reg_x64,
                            tmp1.reg,
                            (*self.build).f32x4(1.0, 1.0, 1.0, 1.0),
                            inst.reg_x64,
                        );
                    }
                }
                IrCmd::SELECT_NUM => {
                    {
                        inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                            SizeX64::xmmword,
                            index,
                            &[
                                (*get_op_mut(inst, (((0) as u32) as u32))),
                                (*get_op_mut(inst, (((2) as u32) as u32))),
                                (*get_op_mut(inst, (((3) as u32) as u32))),
                            ],
                        ); // can't reuse b if a is a memory operand

                        let mut tmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp.alloc(SizeX64::xmmword);

                        if (*get_op_mut(inst, (((2) as u32) as u32))).kind() == IrOpKind::Inst {
                            (*self.build).vcmpeqsd(
                                OperandX64::reg(tmp.reg),
                                OperandX64::reg(self.reg_op((*get_op_mut(inst, 2)))),
                                self.mem_reg_double_op((*get_op_mut(inst, 3))),
                            );
                        } else {
                            (*self.build).vmovsd_operand_x_64_operand_x_64(
                                OperandX64::reg(tmp.reg),
                                self.mem_reg_double_op((*get_op_mut(inst, 2))),
                            );
                            (*self.build).vcmpeqsd(
                                OperandX64::reg(tmp.reg),
                                OperandX64::reg(tmp.reg),
                                self.mem_reg_double_op((*get_op_mut(inst, 3))),
                            );
                        }

                        if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Inst {
                            (*self.build).vblendvpd(
                                inst.reg_x64,
                                self.reg_op((*get_op_mut(inst, 0))),
                                self.mem_reg_double_op((*get_op_mut(inst, 1))),
                                tmp.reg,
                            );
                        } else {
                            (*self.build).vmovsd_operand_x_64_operand_x_64(
                                OperandX64::reg(inst.reg_x64),
                                self.mem_reg_double_op((*get_op_mut(inst, 0))),
                            );
                            (*self.build).vblendvpd(
                                inst.reg_x64,
                                inst.reg_x64,
                                self.mem_reg_double_op((*get_op_mut(inst, 1))),
                                tmp.reg,
                            );
                        }
                    }
                }
                IrCmd::SELECT_INT64 => {
                    {
                        // Select B if C cond D, otherwise select A
                        // A, B: int64 (endpoints), C, D: int64 (condition arguments), E: condition
                        inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                            SizeX64::qword,
                            index,
                            &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                        );

                        let mut cond = condition_op((*get_op_mut(inst, 4)));

                        // Start with falseVal (A), conditionally replace with trueVal (B)
                        (*self.build).mov(
                            OperandX64::reg(inst.reg_x64),
                            self.mem_reg_int_64_op((*get_op_mut(inst, 0))),
                        );

                        let mut tmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp.alloc(SizeX64::qword);
                        // Compare C vs D
                        if (*get_op_mut(inst, (((2) as u32) as u32))).kind() == IrOpKind::Inst {
                            (*self.build).cmp(
                                OperandX64::reg(self.reg_op((*get_op_mut(inst, 2)))),
                                self.mem_reg_int_64_op((*get_op_mut(inst, 3))),
                            );
                        } else {
                            (*self.build).mov(
                                OperandX64::reg(tmp.reg),
                                self.mem_reg_int_64_op((*get_op_mut(inst, 2))),
                            );
                            (*self.build).cmp(
                                OperandX64::reg(tmp.reg),
                                self.mem_reg_int_64_op((*get_op_mut(inst, 3))),
                            );
                        }

                        // If condition is true, select B instead
                        (*self.build).cmov(
                            get_condition_int(cond),
                            inst.reg_x64,
                            self.mem_reg_int_64_op((*get_op_mut(inst, 1))),
                        );
                    }
                }
                IrCmd::SELECT_VEC => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[
                            (*get_op_mut(inst, (((2) as u32) as u32))),
                            (*get_op_mut(inst, (((3) as u32) as u32))),
                        ],
                    );

                    let mut tmp1 = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };
                    let mut tmp2 = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };
                    let mut tmpc = self.vec_op((*get_op_mut(inst, 2)), &mut tmp1);
                    let mut tmpd = self.vec_op((*get_op_mut(inst, 3)), &mut tmp2);

                    (*self.build).vcmpeqps(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(tmpc),
                        OperandX64::reg(tmpd),
                    );
                    (*self.build).vblendvps(
                        inst.reg_x64,
                        self.vec_op((*get_op_mut(inst, 0)), &mut tmp1),
                        OperandX64::reg(self.vec_op((*get_op_mut(inst, 1)), &mut tmp2)),
                        inst.reg_x64,
                    );
                }
                IrCmd::SELECT_IF_TRUTHY => {
                    {
                        inst.reg_x64 = self.regs.alloc_reg(SizeX64::xmmword, index); // No reuse since multiple inputs can be shared

                        // Place lhs as the result, we will overwrite it with rhs if 'A' is falsy later
                        (*self.build).vmovaps(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 1)))),
                        );

                        // Get rhs register early, so a potential restore happens on both sides of a conditional control flow
                        let mut c = self.reg_op((*get_op_mut(inst, 2)));

                        let mut tmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp.alloc(SizeX64::dword);
                        let mut saveRhs = Label::default();
                        let mut exit = Label::default();

                        // Check tag first
                        (*self.build).vpextrd(
                            tmp.reg,
                            self.reg_op((*get_op_mut(inst, 0))),
                            (((3) as u8) as u8),
                        );
                        (*self.build).cmp(
                            OperandX64::reg(tmp.reg),
                            OperandX64::imm((lua_Type::LUA_TBOOLEAN as u8) as i32),
                        );

                        (*self.build).jcc(ConditionX64::Below, &mut saveRhs); // rhs if 'A' is nil
                        (*self.build).jcc(ConditionX64::Above, &mut exit); // Keep lhs if 'A' is not a boolean

                        // Check the boolean value
                        (*self.build).vpextrd(
                            tmp.reg,
                            self.reg_op((*get_op_mut(inst, 0))),
                            (((0) as u8) as u8),
                        );
                        (*self.build).test(OperandX64::reg(tmp.reg), OperandX64::reg(tmp.reg));
                        (*self.build).jcc(ConditionX64::NotZero, &mut exit); // Keep lhs if 'A' is true

                        (*self.build).set_label(&mut saveRhs);
                        (*self.build).vmovaps(OperandX64::reg(inst.reg_x64), OperandX64::reg(c));

                        (*self.build).set_label(&mut exit);
                    }
                }
                IrCmd::ADD_VEC => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[
                            (*get_op_mut(inst, (((0) as u32) as u32))),
                            (*get_op_mut(inst, (((1) as u32) as u32))),
                        ],
                    );

                    let mut tmp1 = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };
                    let mut tmp2 = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };

                    let op0 = *get_op_mut(inst, 0);
                    let op1 = *get_op_mut(inst, 1);
                    let mut tmpa = self.vec_op(op0, &mut tmp1);
                    let mut tmpb = if op0 == op1 {
                        tmpa
                    } else {
                        self.vec_op(op1, &mut tmp2)
                    };

                    (*self.build).vaddps(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(tmpa),
                        OperandX64::reg(tmpb),
                    );
                }
                IrCmd::SUB_VEC => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[
                            (*get_op_mut(inst, (((0) as u32) as u32))),
                            (*get_op_mut(inst, (((1) as u32) as u32))),
                        ],
                    );

                    let mut tmp1 = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };
                    let mut tmp2 = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };

                    let op0 = *get_op_mut(inst, 0);
                    let op1 = *get_op_mut(inst, 1);
                    let mut tmpa = self.vec_op(op0, &mut tmp1);
                    let mut tmpb = if op0 == op1 {
                        tmpa
                    } else {
                        self.vec_op(op1, &mut tmp2)
                    };

                    (*self.build).vsubps(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(tmpa),
                        OperandX64::reg(tmpb),
                    );
                }
                IrCmd::MUL_VEC => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[
                            (*get_op_mut(inst, (((0) as u32) as u32))),
                            (*get_op_mut(inst, (((1) as u32) as u32))),
                        ],
                    );

                    let mut tmp1 = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };
                    let mut tmp2 = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };

                    let op0 = *get_op_mut(inst, 0);
                    let op1 = *get_op_mut(inst, 1);
                    let mut tmpa = self.vec_op(op0, &mut tmp1);
                    let mut tmpb = if op0 == op1 {
                        tmpa
                    } else {
                        self.vec_op(op1, &mut tmp2)
                    };

                    (*self.build).vmulps(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(tmpa),
                        OperandX64::reg(tmpb),
                    );
                }
                IrCmd::DIV_VEC => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[
                            (*get_op_mut(inst, (((0) as u32) as u32))),
                            (*get_op_mut(inst, (((1) as u32) as u32))),
                        ],
                    );

                    let mut tmp1 = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };
                    let mut tmp2 = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };

                    let op0 = *get_op_mut(inst, 0);
                    let op1 = *get_op_mut(inst, 1);
                    let mut tmpa = self.vec_op(op0, &mut tmp1);
                    let mut tmpb = if op0 == op1 {
                        tmpa
                    } else {
                        self.vec_op(op1, &mut tmp2)
                    };

                    (*self.build).vdivps(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(tmpa),
                        OperandX64::reg(tmpb),
                    );
                }
                IrCmd::IDIV_VEC => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[
                            (*get_op_mut(inst, (((0) as u32) as u32))),
                            (*get_op_mut(inst, (((1) as u32) as u32))),
                        ],
                    );

                    let mut tmp1 = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };
                    let mut tmp2 = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };

                    let op0 = *get_op_mut(inst, 0);
                    let op1 = *get_op_mut(inst, 1);
                    let mut tmpa = self.vec_op(op0, &mut tmp1);
                    let mut tmpb = if op0 == op1 {
                        tmpa
                    } else {
                        self.vec_op(op1, &mut tmp2)
                    };

                    (*self.build).vdivps(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(tmpa),
                        OperandX64::reg(tmpb),
                    );
                    (*self.build).vroundps(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(inst.reg_x64),
                        RoundingModeX64::RoundToNegativeInfinity,
                    );
                }
                IrCmd::MULADD_VEC => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );
                    let mut tmp1 = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };
                    let mut tmp2 = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };
                    let mut tmp3 = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };

                    let mut tmpa = self.vec_op((*get_op_mut(inst, 0)), &mut tmp1);
                    let mut tmpb = self.vec_op((*get_op_mut(inst, 1)), &mut tmp2);
                    let mut tmpc = self.vec_op((*get_op_mut(inst, 2)), &mut tmp3);

                    if ((*self.build).features & FeaturesX64::Feature_FMA3 as u32) != 0 {
                        if inst.reg_x64 != tmpa {
                            (*self.build)
                                .vmovups(OperandX64::reg(inst.reg_x64), OperandX64::reg(tmpa));
                        }

                        (*self.build).vfmadd213ps(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(tmpb),
                            OperandX64::reg(tmpc),
                        );
                    } else {
                        (*self.build).vmulps(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(tmpa),
                            OperandX64::reg(tmpb),
                        );
                        (*self.build).vaddps(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(tmpc),
                        );
                    }
                }
                IrCmd::UNM_VEC => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );

                    (*self.build).vxorpd(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                        (*self.build).f32x4(-0.0, -0.0, -0.0, -0.0),
                    );
                }
                IrCmd::MIN_VEC => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[
                            (*get_op_mut(inst, (((0) as u32) as u32))),
                            (*get_op_mut(inst, (((1) as u32) as u32))),
                        ],
                    );

                    let mut tmp1 = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };
                    let mut tmp2 = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };

                    let op0 = *get_op_mut(inst, 0);
                    let op1 = *get_op_mut(inst, 1);
                    let mut tmpa = self.vec_op(op0, &mut tmp1);
                    let mut tmpb = if op0 == op1 {
                        tmpa
                    } else {
                        self.vec_op(op1, &mut tmp2)
                    };

                    (*self.build).vminps(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(tmpa),
                        OperandX64::reg(tmpb),
                    );
                }
                IrCmd::MAX_VEC => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[
                            (*get_op_mut(inst, (((0) as u32) as u32))),
                            (*get_op_mut(inst, (((1) as u32) as u32))),
                        ],
                    );

                    let mut tmp1 = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };
                    let mut tmp2 = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };

                    let op0 = *get_op_mut(inst, 0);
                    let op1 = *get_op_mut(inst, 1);
                    let mut tmpa = self.vec_op(op0, &mut tmp1);
                    let mut tmpb = if op0 == op1 {
                        tmpa
                    } else {
                        self.vec_op(op1, &mut tmp2)
                    };

                    (*self.build).vmaxps(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(tmpa),
                        OperandX64::reg(tmpb),
                    );
                }
                IrCmd::FLOOR_VEC => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );

                    let mut tmp1 = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };
                    let mut tmpa = self.vec_op((*get_op_mut(inst, 0)), &mut tmp1);

                    (*self.build).vroundps(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(tmpa),
                        RoundingModeX64::RoundToNegativeInfinity,
                    );
                }
                IrCmd::CEIL_VEC => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );

                    let mut tmp1 = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };
                    let mut tmpa = self.vec_op((*get_op_mut(inst, 0)), &mut tmp1);

                    (*self.build).vroundps(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(tmpa),
                        RoundingModeX64::RoundToPositiveInfinity,
                    );
                }
                IrCmd::ABS_VEC => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );

                    let mut tmp1 = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };
                    let mut tmpa = self.vec_op((*get_op_mut(inst, 0)), &mut tmp1);

                    (*self.build).vandps(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(tmpa),
                        (*self.build).u32x4(0x7fffffff, 0x7fffffff, 0x7fffffff, 0x7fffffff),
                    );
                }
                IrCmd::DOT_VEC => {
                    {
                        inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                            SizeX64::xmmword,
                            index,
                            &[
                                (*get_op_mut(inst, (((0) as u32) as u32))),
                                (*get_op_mut(inst, (((1) as u32) as u32))),
                            ],
                        );

                        let mut tmp1 = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        let mut tmp2 = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };

                        let op0 = *get_op_mut(inst, 0);
                        let op1 = *get_op_mut(inst, 1);
                        let mut tmpa = self.vec_op(op0, &mut tmp1);
                        let mut tmpb = if op0 == op1 {
                            tmpa
                        } else {
                            self.vec_op(op1, &mut tmp2)
                        };

                        (*self.build).vdpps(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(tmpa),
                            OperandX64::reg(tmpb),
                            0x71,
                        ); // 7 = 0b0111, sum first 3 products into first float
                    }
                }
                IrCmd::EXTRACT_VEC => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );

                    (*self.build).vpshufps(
                        inst.reg_x64,
                        self.reg_op((*get_op_mut(inst, 0))),
                        OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                        (((self.int_op((*get_op_mut(inst, 1)))) as u8) as u8),
                    );
                }
                IrCmd::NOT_ANY => {
                    {
                        // TODO: if we have a single user which is a STORE_INT, we are missing the opportunity to write directly to target
                        inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                            SizeX64::dword,
                            index,
                            &[
                                (*get_op_mut(inst, (((0) as u32) as u32))),
                                (*get_op_mut(inst, (((1) as u32) as u32))),
                            ],
                        );

                        let mut saveOne = Label::default();
                        let mut saveZero = Label::default();
                        let mut exit = Label::default();

                        if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Constant {
                            // Other cases should've been constant folded
                            CODEGEN_ASSERT!(
                                self.tag_op((*get_op_mut(inst, 0))) == lua_Type::LUA_TBOOLEAN as u8
                            );
                        } else {
                            (*self.build).cmp(
                                OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                                OperandX64::imm((lua_Type::LUA_TNIL as u8) as i32),
                            );
                            (*self.build).jcc(ConditionX64::Equal, &mut saveOne);

                            (*self.build).cmp(
                                OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                                OperandX64::imm((lua_Type::LUA_TBOOLEAN as u8) as i32),
                            );
                            (*self.build).jcc(ConditionX64::NotEqual, &mut saveZero);
                        }

                        if (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Constant {
                            // If value is 1, we fallthrough to storing 0
                            if self.int_op((*get_op_mut(inst, 1))) == 0 {
                                (*self.build).jmp_label(&mut saveOne);
                            }
                        } else {
                            (*self.build).cmp(
                                OperandX64::reg(self.reg_op((*get_op_mut(inst, 1)))),
                                OperandX64::imm((0) as i32),
                            );
                            (*self.build).jcc(ConditionX64::Equal, &mut saveOne);
                        }

                        (*self.build).set_label(&mut saveZero);
                        (*self.build)
                            .mov(OperandX64::reg(inst.reg_x64), OperandX64::imm((0) as i32));
                        (*self.build).jmp_label(&mut exit);

                        (*self.build).set_label(&mut saveOne);
                        (*self.build)
                            .mov(OperandX64::reg(inst.reg_x64), OperandX64::imm((1) as i32));

                        (*self.build).set_label(&mut exit);
                    }
                }
                IrCmd::CMP_INT => {
                    {
                        // Cannot reuse operand registers as a target because we have to modify it before the comparison
                        inst.reg_x64 = self.regs.alloc_reg(SizeX64::dword, index);

                        // We are going to operate on byte register, those do not clear high bits on write
                        (*self.build)
                            .xor_(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));

                        let mut cond = condition_op((*get_op_mut(inst, 2)));

                        if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Constant {
                            (*self.build).cmp(
                                OperandX64::reg(self.reg_op((*get_op_mut(inst, 1)))),
                                OperandX64::imm((self.int_op((*get_op_mut(inst, 0)))) as i32),
                            );
                            (*self.build).setcc(
                                get_inverse_condition(get_condition_int(cond)),
                                OperandX64::reg(byte_reg(inst.reg_x64)),
                            );
                        } else if (*get_op_mut(inst, (((0) as u32) as u32))).kind()
                            == IrOpKind::Inst
                        {
                            (*self.build).cmp(
                                OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                                OperandX64::imm((self.int_op((*get_op_mut(inst, 1)))) as i32),
                            );
                            (*self.build).setcc(
                                get_condition_int(cond),
                                OperandX64::reg(byte_reg(inst.reg_x64)),
                            );
                        } else {
                            CODEGEN_ASSERT!(false, "Unsupported instruction form");
                        }
                    }
                }
                IrCmd::CMP_ANY => {
                    {
                        CODEGEN_ASSERT!(
                            (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::VmReg
                                && (*get_op_mut(inst, (((1) as u32) as u32))).kind()
                                    == IrOpKind::VmReg
                        );
                        let mut cond = condition_op((*get_op_mut(inst, 2)));

                        inst.reg_x64 = self.regs.alloc_reg(SizeX64::dword, index);

                        let mut skip = Label::default();
                        let mut exit = Label::default();

                        // For equality comparison, 'luaV_equalval' expects tag to be equal before the call
                        if cond == IrCondition::Equal {
                            let mut tmp = ScopedRegX64 {
                                owner: &mut self.regs,
                                reg: RegisterX64::noreg,
                            };
                            tmp.alloc(SizeX64::dword);

                            (*self.build).mov(
                                OperandX64::reg(tmp.reg),
                                self.mem_reg_tag_op((*get_op_mut(inst, 0))),
                            );
                            (*self.build).cmp(
                                self.mem_reg_tag_op((*get_op_mut(inst, 1))),
                                OperandX64::reg(tmp.reg),
                            );

                            // If the tags are not equal, skip the call and set result to 0
                            (*self.build).jcc(ConditionX64::NotEqual, &mut skip);
                        }

                        {
                            let mut spillGuard = ScopedSpills {
                                owner: core::ptr::null_mut(),
                                start_spill_id: 0,
                            };
                            spillGuard
                                .scoped_spills_scoped_spills_ir_reg_alloc_x_64(&mut self.regs);

                            let mut callWrap =
                                IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
                                    &mut self.regs,
                                    &mut *self.build,
                                    index,
                                );
                            callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                                SizeX64::qword,
                                OperandX64::reg(r_state()),
                                IrOp::default(),
                            );
                            callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                                SizeX64::qword,
                                luau_reg_address(vm_reg_op((*get_op_mut(inst, 0)))),
                                IrOp::default(),
                            );
                            callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                                SizeX64::qword,
                                luau_reg_address(vm_reg_op((*get_op_mut(inst, 1)))),
                                IrOp::default(),
                            );
                            callWrap.set_result_register(inst.reg_x64, index);

                            if cond == IrCondition::LessEqual {
                                callWrap.call(&OperandX64::mem(
                                    SizeX64::qword,
                                    RegisterX64::noreg,
                                    1,
                                    r_native_context(),
                                    (core::mem::offset_of!(NativeContext, luaV_lessequal) as i32),
                                ));
                            } else if cond == IrCondition::Less {
                                callWrap.call(&OperandX64::mem(
                                    SizeX64::qword,
                                    RegisterX64::noreg,
                                    1,
                                    r_native_context(),
                                    (core::mem::offset_of!(NativeContext, luaV_lessthan) as i32),
                                ));
                            } else if cond == IrCondition::Equal {
                                callWrap.call(&OperandX64::mem(
                                    SizeX64::qword,
                                    RegisterX64::noreg,
                                    1,
                                    r_native_context(),
                                    (core::mem::offset_of!(NativeContext, luaV_equalval) as i32),
                                ));
                            } else {
                                CODEGEN_ASSERT!(false, "Unsupported condition");
                            }

                            emit_update_base(&mut *self.build);
                        }

                        if cond == IrCondition::Equal {
                            (*self.build).jmp_label(&mut exit);
                            (*self.build).set_label(&mut skip);

                            (*self.build)
                                .xor_(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));
                            (*self.build).set_label(&mut exit);
                        }

                        // If case we made a call, skip high register bits clear, only consumer is JUMP_CMP_INT which doesn't read them
                    }
                }
                IrCmd::CMP_TAG => {
                    {
                        // Cannot reuse operand registers as a target because we have to modify it before the comparison
                        inst.reg_x64 = self.regs.alloc_reg(SizeX64::dword, index);

                        // We are going to operate on byte register, those do not clear high bits on write
                        (*self.build)
                            .xor_(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));

                        let mut cond = condition_op((*get_op_mut(inst, 2)));
                        CODEGEN_ASSERT!(
                            cond == IrCondition::Equal || cond == IrCondition::NotEqual
                        );
                        let mut condX64 = get_condition_int(cond);

                        if self.tag_op((*get_op_mut(inst, 1))) == lua_Type::LUA_TNIL as u8
                            && (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Inst
                        {
                            (*self.build).test(
                                OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                                OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                            );
                        } else {
                            (*self.build).cmp(
                                self.mem_reg_tag_op((*get_op_mut(inst, 0))),
                                OperandX64::imm((self.tag_op((*get_op_mut(inst, 1)))) as i32),
                            );
                        }

                        (*self.build).setcc(condX64, OperandX64::reg(byte_reg(inst.reg_x64)));
                    }
                }
                IrCmd::CMP_SPLIT_TVALUE => {
                    {
                        // Cannot reuse operand registers as a target because we have to modify it before the comparison
                        inst.reg_x64 = self.regs.alloc_reg(SizeX64::dword, index);

                        // Second operand of this instruction must be a constant
                        // Without a constant type, we wouldn't know the correct way to compare the values at lowering time
                        CODEGEN_ASSERT!(
                            (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Constant
                        );

                        // We are going to operate on byte registers, those do not clear high bits on write
                        (*self.build)
                            .xor_(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));

                        let mut cond = condition_op((*get_op_mut(inst, 4)));
                        CODEGEN_ASSERT!(
                            cond == IrCondition::Equal || cond == IrCondition::NotEqual
                        );

                        // Check tag equality first
                        let mut tmp1 = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp1.alloc(SizeX64::byte);

                        if (*get_op_mut(inst, (((0) as u32) as u32))).kind() != IrOpKind::Constant {
                            (*self.build).cmp(
                                OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                                OperandX64::imm((self.tag_op((*get_op_mut(inst, 1)))) as i32),
                            );
                            (*self.build).setcc(
                                get_condition_int(cond),
                                OperandX64::reg(byte_reg(tmp1.reg)),
                            );
                        } else {
                            // Constant folding had to handle different constant tags
                            CODEGEN_ASSERT!(
                                self.tag_op((*get_op_mut(inst, 0)))
                                    == self.tag_op((*get_op_mut(inst, 1)))
                            );
                        }

                        if self.tag_op((*get_op_mut(inst, 1))) == lua_Type::LUA_TBOOLEAN as u8 {
                            if (*get_op_mut(inst, (((2) as u32) as u32))).kind()
                                == IrOpKind::Constant
                            {
                                (*self.build).cmp(
                                    OperandX64::reg(self.reg_op((*get_op_mut(inst, 3)))),
                                    OperandX64::imm((self.int_op((*get_op_mut(inst, 2)))) as i32),
                                );
                            }
                            // swapped arguments
                            else if (*get_op_mut(inst, (((3) as u32) as u32))).kind()
                                == IrOpKind::Constant
                            {
                                (*self.build).cmp(
                                    OperandX64::reg(self.reg_op((*get_op_mut(inst, 2)))),
                                    OperandX64::imm((self.int_op((*get_op_mut(inst, 3)))) as i32),
                                );
                            } else {
                                (*self.build).cmp(
                                    OperandX64::reg(self.reg_op((*get_op_mut(inst, 2)))),
                                    OperandX64::reg(self.reg_op((*get_op_mut(inst, 3)))),
                                );
                            }

                            (*self.build).setcc(
                                get_condition_int(cond),
                                OperandX64::reg(byte_reg(inst.reg_x64)),
                            );
                        } else if self.tag_op((*get_op_mut(inst, 1))) == lua_Type::LUA_TSTRING as u8
                        {
                            (*self.build).cmp(
                                OperandX64::reg(self.reg_op((*get_op_mut(inst, 2)))),
                                OperandX64::reg(self.reg_op((*get_op_mut(inst, 3)))),
                            );
                            (*self.build).setcc(
                                get_condition_int(cond),
                                OperandX64::reg(byte_reg(inst.reg_x64)),
                            );
                        } else if self.tag_op((*get_op_mut(inst, 1))) == lua_Type::LUA_TNUMBER as u8
                        {
                            if (*get_op_mut(inst, (((2) as u32) as u32))).kind()
                                == IrOpKind::Constant
                            {
                                (*self.build).vucomisd(
                                    OperandX64::reg(self.reg_op((*get_op_mut(inst, 3)))),
                                    self.mem_reg_double_op((*get_op_mut(inst, 2))),
                                );
                            }
                            // swapped arguments
                            else if (*get_op_mut(inst, (((3) as u32) as u32))).kind()
                                == IrOpKind::Constant
                            {
                                (*self.build).vucomisd(
                                    OperandX64::reg(self.reg_op((*get_op_mut(inst, 2)))),
                                    self.mem_reg_double_op((*get_op_mut(inst, 3))),
                                );
                            } else {
                                (*self.build).vucomisd(
                                    OperandX64::reg(self.reg_op((*get_op_mut(inst, 2)))),
                                    OperandX64::reg(self.reg_op((*get_op_mut(inst, 3)))),
                                );
                            }

                            let op2 = *get_op_mut(inst, 2);
                            let op3 = *get_op_mut(inst, 3);
                            if op2 == op3 {
                                // When numbers are the same, we only need to check parity to detect NaN
                                if cond == IrCondition::Equal {
                                    (*self.build).setcc(
                                        ConditionX64::NotParity,
                                        OperandX64::reg(byte_reg(inst.reg_x64)),
                                    );
                                } else {
                                    (*self.build).setcc(
                                        ConditionX64::Parity,
                                        OperandX64::reg(byte_reg(inst.reg_x64)),
                                    );
                                }
                            } else {
                                let mut tmp2 = ScopedRegX64 {
                                    owner: &mut self.regs,
                                    reg: RegisterX64::noreg,
                                };
                                tmp2.alloc(SizeX64::dword);

                                if cond == IrCondition::Equal {
                                    (*self.build).mov(
                                        OperandX64::reg(tmp2.reg),
                                        OperandX64::imm((0) as i32),
                                    );
                                    (*self.build).setcc(
                                        ConditionX64::NotParity,
                                        OperandX64::reg(byte_reg(inst.reg_x64)),
                                    );
                                    (*self.build).cmov(
                                        ConditionX64::NotEqual,
                                        inst.reg_x64,
                                        OperandX64::reg(tmp2.reg),
                                    );
                                } else {
                                    (*self.build).mov(
                                        OperandX64::reg(tmp2.reg),
                                        OperandX64::imm((1) as i32),
                                    );
                                    (*self.build).setcc(
                                        ConditionX64::Parity,
                                        OperandX64::reg(byte_reg(inst.reg_x64)),
                                    );
                                    (*self.build).cmov(
                                        ConditionX64::NotEqual,
                                        inst.reg_x64,
                                        OperandX64::reg(tmp2.reg),
                                    );
                                }
                            }
                        } else if self.tag_op((*get_op_mut(inst, 1)))
                            == lua_Type::LUA_TINTEGER as u8
                        {
                            if (*get_op_mut(inst, (((2) as u32) as u32))).kind()
                                == IrOpKind::Constant
                            {
                                (*self.build).cmp(
                                    OperandX64::reg(self.reg_op((*get_op_mut(inst, 3)))),
                                    self.mem_reg_int_64_op((*get_op_mut(inst, 2))),
                                );
                            }
                            // swapped arguments
                            else if (*get_op_mut(inst, (((3) as u32) as u32))).kind()
                                == IrOpKind::Constant
                            {
                                (*self.build).cmp(
                                    OperandX64::reg(self.reg_op((*get_op_mut(inst, 2)))),
                                    self.mem_reg_int_64_op((*get_op_mut(inst, 3))),
                                );
                            } else {
                                (*self.build).cmp(
                                    OperandX64::reg(self.reg_op((*get_op_mut(inst, 2)))),
                                    OperandX64::reg(self.reg_op((*get_op_mut(inst, 3)))),
                                );
                            }

                            (*self.build).setcc(
                                get_condition_int(cond),
                                OperandX64::reg(byte_reg(inst.reg_x64)),
                            );
                        } else {
                            CODEGEN_ASSERT!(false, "unsupported type tag in CMP_SPLIT_TVALUE");
                        }

                        if (*get_op_mut(inst, (((0) as u32) as u32))).kind() != IrOpKind::Constant {
                            if cond == IrCondition::Equal {
                                (*self.build).and_(
                                    OperandX64::reg(byte_reg(inst.reg_x64)),
                                    OperandX64::reg(byte_reg(tmp1.reg)),
                                );
                            } else {
                                (*self.build).or_(
                                    OperandX64::reg(byte_reg(inst.reg_x64)),
                                    OperandX64::reg(byte_reg(tmp1.reg)),
                                );
                            }
                        }
                    }
                }
                IrCmd::JUMP => {
                    self.jump_or_abort_on_undef_ir_op_u32_ir_block(
                        (*get_op_mut(inst, 0)),
                        index,
                        next,
                    );
                }
                IrCmd::JUMP_IF_TRUTHY => {
                    jump_if_truthy(
                        &mut *self.build,
                        vm_reg_op((*get_op_mut(inst, 0))),
                        self.label_op((*get_op_mut(inst, 1))),
                        self.label_op((*get_op_mut(inst, 2))),
                    );
                    let target_block = self.block_op((*get_op_mut(inst, 2))) as *mut IrBlock;
                    self.jump_or_fallthrough(&mut *target_block, next);
                }
                IrCmd::JUMP_IF_FALSY => {
                    jump_if_falsy(
                        &mut *self.build,
                        vm_reg_op((*get_op_mut(inst, 0))),
                        self.label_op((*get_op_mut(inst, 1))),
                        self.label_op((*get_op_mut(inst, 2))),
                    );
                    let target_block = self.block_op((*get_op_mut(inst, 2))) as *mut IrBlock;
                    self.jump_or_fallthrough(&mut *target_block, next);
                }
                IrCmd::JUMP_EQ_TAG => {
                    CODEGEN_ASSERT!(
                        (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Inst
                            || (*get_op_mut(inst, (((1) as u32) as u32))).kind()
                                == IrOpKind::Constant
                    );
                    let mut opb =
                        if (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Inst {
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 1))))
                        } else {
                            OperandX64::imm(self.tag_op((*get_op_mut(inst, 1))) as i32)
                        };

                    if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Constant {
                        (*self.build).cmp(
                            opb,
                            OperandX64::imm((self.tag_op((*get_op_mut(inst, 0)))) as i32),
                        );
                    } else {
                        (*self.build).cmp(self.mem_reg_tag_op((*get_op_mut(inst, 0))), opb);
                    }

                    if self.is_fallthrough_block(self.block_op((*get_op_mut(inst, 3))), next) {
                        (*self.build)
                            .jcc(ConditionX64::Equal, self.label_op((*get_op_mut(inst, 2))));
                        let target_block = self.block_op((*get_op_mut(inst, 3))) as *mut IrBlock;
                        self.jump_or_fallthrough(&mut *target_block, next);
                    } else {
                        (*self.build).jcc(
                            ConditionX64::NotEqual,
                            self.label_op((*get_op_mut(inst, 3))),
                        );
                        let target_block = self.block_op((*get_op_mut(inst, 2))) as *mut IrBlock;
                        self.jump_or_fallthrough(&mut *target_block, next);
                    }
                }
                IrCmd::JUMP_CMP_INT => {
                    let mut cond = condition_op((*get_op_mut(inst, 2)));

                    if (cond == IrCondition::Equal || cond == IrCondition::NotEqual)
                        && self.int_op((*get_op_mut(inst, 1))) == 0
                    {
                        let mut invert = cond == IrCondition::NotEqual;

                        (*self.build).test(
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                        );

                        if self.is_fallthrough_block(self.block_op((*get_op_mut(inst, 3))), next) {
                            (*self.build).jcc(
                                if invert {
                                    ConditionX64::Zero
                                } else {
                                    ConditionX64::NotZero
                                },
                                self.label_op((*get_op_mut(inst, 4))),
                            );
                            let target_block =
                                self.block_op((*get_op_mut(inst, 3))) as *mut IrBlock;
                            self.jump_or_fallthrough(&mut *target_block, next);
                        } else {
                            (*self.build).jcc(
                                if invert {
                                    ConditionX64::NotZero
                                } else {
                                    ConditionX64::Zero
                                },
                                self.label_op((*get_op_mut(inst, 3))),
                            );
                            let target_block =
                                self.block_op((*get_op_mut(inst, 4))) as *mut IrBlock;
                            self.jump_or_fallthrough(&mut *target_block, next);
                        }
                    } else {
                        (*self.build).cmp(
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                            OperandX64::imm((self.int_op((*get_op_mut(inst, 1)))) as i32),
                        );

                        (*self.build).jcc(
                            get_condition_int(cond),
                            self.label_op((*get_op_mut(inst, 3))),
                        );
                        let target_block = self.block_op((*get_op_mut(inst, 4))) as *mut IrBlock;
                        self.jump_or_fallthrough(&mut *target_block, next);
                    }
                }
                IrCmd::JUMP_EQ_POINTER => {
                    (*self.build).cmp(
                        OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                        OperandX64::reg(self.reg_op((*get_op_mut(inst, 1)))),
                    );

                    (*self.build).jcc(ConditionX64::Equal, self.label_op((*get_op_mut(inst, 2))));
                    let target_block = self.block_op((*get_op_mut(inst, 3))) as *mut IrBlock;
                    self.jump_or_fallthrough(&mut *target_block, next);
                }
                IrCmd::JUMP_CMP_NUM => {
                    {
                        let mut cond = condition_op((*get_op_mut(inst, 2)));

                        let mut tmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp.alloc(SizeX64::xmmword);

                        jump_on_number_cmp(
                            &mut *self.build,
                            tmp.reg,
                            self.mem_reg_double_op((*get_op_mut(inst, 0))),
                            self.mem_reg_double_op((*get_op_mut(inst, 1))),
                            cond,
                            self.label_op((*get_op_mut(inst, 3))),
                            /* floatPrecision */ false,
                        );
                        let target_block = self.block_op((*get_op_mut(inst, 4))) as *mut IrBlock;
                        self.jump_or_fallthrough(&mut *target_block, next);
                    }
                }
                IrCmd::JUMP_CMP_FLOAT => {
                    {
                        let mut cond = condition_op((*get_op_mut(inst, 2)));

                        let mut tmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp.alloc(SizeX64::xmmword);

                        jump_on_number_cmp(
                            &mut *self.build,
                            tmp.reg,
                            self.mem_reg_float_op((*get_op_mut(inst, 0))),
                            self.mem_reg_float_op((*get_op_mut(inst, 1))),
                            cond,
                            self.label_op((*get_op_mut(inst, 3))),
                            /* floatPrecision */ true,
                        );
                        let target_block = self.block_op((*get_op_mut(inst, 4))) as *mut IrBlock;
                        self.jump_or_fallthrough(&mut *target_block, next);
                    }
                }
                IrCmd::JUMP_FORN_LOOP_COND => {
                    {
                        let mut tmp1 = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp1.alloc(SizeX64::xmmword);
                        let mut tmp2 = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp2.alloc(SizeX64::xmmword);
                        let mut tmp3 = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp3.alloc(SizeX64::xmmword);

                        let mut index = if (*get_op_mut(inst, (((0) as u32) as u32))).kind()
                            == IrOpKind::Inst
                        {
                            self.reg_op((*get_op_mut(inst, 0)))
                        } else {
                            tmp1.reg
                        };
                        let mut limit = if (*get_op_mut(inst, (((1) as u32) as u32))).kind()
                            == IrOpKind::Inst
                        {
                            self.reg_op((*get_op_mut(inst, 1)))
                        } else {
                            tmp2.reg
                        };

                        if (*get_op_mut(inst, (((0) as u32) as u32))).kind() != IrOpKind::Inst {
                            (*self.build).vmovsd_operand_x_64_operand_x_64(
                                OperandX64::reg(tmp1.reg),
                                self.mem_reg_double_op((*get_op_mut(inst, 0))),
                            );
                        }

                        if (*get_op_mut(inst, (((1) as u32) as u32))).kind() != IrOpKind::Inst {
                            (*self.build).vmovsd_operand_x_64_operand_x_64(
                                OperandX64::reg(tmp2.reg),
                                self.mem_reg_double_op((*get_op_mut(inst, 1))),
                            );
                        }

                        let mut direct = Label::default();

                        // step > 0
                        jump_on_number_cmp(
                            &mut *self.build,
                            tmp3.reg,
                            self.mem_reg_double_op((*get_op_mut(inst, 2))),
                            (*self.build).f64(0.0),
                            IrCondition::Greater,
                            &mut direct,
                            /* floatPrecision */ false,
                        );

                        // !(limit <= index)
                        jump_on_number_cmp(
                            &mut *self.build,
                            RegisterX64::noreg,
                            OperandX64::reg(limit),
                            OperandX64::reg(index),
                            IrCondition::NotLessEqual,
                            self.label_op((*get_op_mut(inst, 4))),
                            /* floatPrecision */ false,
                        );
                        (*self.build).jmp_label(self.label_op((*get_op_mut(inst, 3))));

                        // !(index <= limit)
                        (*self.build).set_label(&mut direct);
                        jump_on_number_cmp(
                            &mut *self.build,
                            RegisterX64::noreg,
                            OperandX64::reg(index),
                            OperandX64::reg(limit),
                            IrCondition::NotLessEqual,
                            self.label_op((*get_op_mut(inst, 4))),
                            /* floatPrecision */ false,
                        );
                        let target_block = self.block_op((*get_op_mut(inst, 3))) as *mut IrBlock;
                        self.jump_or_fallthrough(&mut *target_block, next);
                    }
                }
                IrCmd::TABLE_LEN => {
                    {
                        let mut callWrap =
                            IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
                                &mut self.regs,
                                &mut *self.build,
                                index,
                            );
                        callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                            SizeX64::qword,
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                            (*get_op_mut(inst, (((0) as u32) as u32))),
                        );
                        callWrap.call(&OperandX64::mem(
                            SizeX64::qword,
                            RegisterX64::noreg,
                            1,
                            r_native_context(),
                            (core::mem::offset_of!(NativeContext, luaH_getn) as i32),
                        ));

                        inst.reg_x64 = self.regs.take_reg(dword_reg(RegisterX64::rax), index);

                        (*self.build)
                            .mov(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));
                        // Ensure high register bits are cleared
                    }
                }
                IrCmd::TABLE_SETNUM => {
                    let mut callWrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
                        &mut self.regs,
                        &mut *self.build,
                        index,
                    );
                    callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                        SizeX64::qword,
                        OperandX64::reg(r_state()),
                        IrOp::default(),
                    );
                    callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                        SizeX64::qword,
                        OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                        (*get_op_mut(inst, (((0) as u32) as u32))),
                    );
                    callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                        SizeX64::dword,
                        OperandX64::reg(self.reg_op((*get_op_mut(inst, 1)))),
                        (*get_op_mut(inst, (((1) as u32) as u32))),
                    );
                    callWrap.call(&OperandX64::mem(
                        SizeX64::qword,
                        RegisterX64::noreg,
                        1,
                        r_native_context(),
                        (core::mem::offset_of!(NativeContext, luaH_setnum) as i32),
                    ));
                    inst.reg_x64 = self.regs.take_reg(RegisterX64::rax, index);
                }
                IrCmd::STRING_LEN => {
                    let mut ptr = self.reg_op((*get_op_mut(inst, 0)));
                    inst.reg_x64 = self.regs.alloc_reg(SizeX64::dword, index);
                    (*self.build).mov(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::mem(
                            SizeX64::dword,
                            RegisterX64::noreg,
                            1,
                            ptr,
                            kTStringLenOffset,
                        ),
                    );
                }
                IrCmd::NEW_TABLE => {
                    let mut callWrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
                        &mut self.regs,
                        &mut *self.build,
                        index,
                    );
                    callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                        SizeX64::qword,
                        OperandX64::reg(r_state()),
                        IrOp::default(),
                    );
                    callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                        SizeX64::dword,
                        OperandX64::imm(self.uint_op((*get_op_mut(inst, 0))) as i32),
                        IrOp::default(),
                    );
                    callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                        SizeX64::dword,
                        OperandX64::imm(self.uint_op((*get_op_mut(inst, 1))) as i32),
                        IrOp::default(),
                    );
                    callWrap.call(&OperandX64::mem(
                        SizeX64::qword,
                        RegisterX64::noreg,
                        1,
                        r_native_context(),
                        (core::mem::offset_of!(NativeContext, luaH_new) as i32),
                    ));
                    inst.reg_x64 = self.regs.take_reg(RegisterX64::rax, index);
                }
                IrCmd::DUP_TABLE => {
                    let mut callWrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
                        &mut self.regs,
                        &mut *self.build,
                        index,
                    );
                    callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                        SizeX64::qword,
                        OperandX64::reg(r_state()),
                        IrOp::default(),
                    );
                    callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                        SizeX64::qword,
                        OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                        (*get_op_mut(inst, (((0) as u32) as u32))),
                    );
                    callWrap.call(&OperandX64::mem(
                        SizeX64::qword,
                        RegisterX64::noreg,
                        1,
                        r_native_context(),
                        (core::mem::offset_of!(NativeContext, luaH_clone) as i32),
                    ));
                    inst.reg_x64 = self.regs.take_reg(RegisterX64::rax, index);
                }
                IrCmd::TRY_NUM_TO_INDEX => {
                    inst.reg_x64 = self.regs.alloc_reg(SizeX64::dword, index);

                    let mut tmp = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };
                    tmp.alloc(SizeX64::xmmword);

                    convert_number_to_index_or_jump(
                        &mut *self.build,
                        tmp.reg,
                        self.reg_op((*get_op_mut(inst, 0))),
                        inst.reg_x64,
                        self.label_op((*get_op_mut(inst, 1))),
                    );
                }
                IrCmd::TRY_CALL_FASTGETTM => {
                    {
                        inst.reg_x64 = self.regs.alloc_reg(SizeX64::qword, index);

                        let mut tmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp.alloc(SizeX64::qword);

                        (*self.build).mov(
                            OperandX64::reg(tmp.reg),
                            OperandX64::mem(
                                SizeX64::qword,
                                RegisterX64::noreg,
                                1,
                                self.reg_op((*get_op_mut(inst, 0))),
                                (core::mem::offset_of!(LuaTable, metatable) as i32),
                            ),
                        );
                        self.regs.free_last_use_reg(
                            (*self.function).inst_op((*get_op_mut(inst, (((0) as u32) as u32)))),
                            index,
                        ); // Release before the call if it's the last use

                        (*self.build).test(OperandX64::reg(tmp.reg), OperandX64::reg(tmp.reg));
                        (*self.build)
                            .jcc(ConditionX64::Zero, self.label_op((*get_op_mut(inst, 2)))); // No metatable

                        (*self.build).test(
                            OperandX64::mem(
                                SizeX64::byte,
                                RegisterX64::noreg,
                                1,
                                tmp.reg,
                                (core::mem::offset_of!(LuaTable, tmcache) as i32),
                            ),
                            OperandX64::imm(1 << self.int_op((*get_op_mut(inst, 1)))),
                        );
                        (*self.build)
                            .jcc(ConditionX64::NotZero, self.label_op((*get_op_mut(inst, 2)))); // No tag method

                        let mut tmp2 = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp2.alloc(SizeX64::qword);
                        (*self.build).mov(
                            OperandX64::reg(tmp2.reg),
                            OperandX64::mem(
                                SizeX64::qword,
                                RegisterX64::noreg,
                                1,
                                r_state(),
                                (core::mem::offset_of!(lua_State, global) as i32),
                            ),
                        );

                        {
                            let mut spillGuard = ScopedSpills {
                                owner: core::ptr::null_mut(),
                                start_spill_id: 0,
                            };
                            spillGuard
                                .scoped_spills_scoped_spills_ir_reg_alloc_x_64(&mut self.regs);

                            let mut callWrap =
                                IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
                                    &mut self.regs,
                                    &mut *self.build,
                                    index,
                                );
                            callWrap
                                .add_argument_size_x_64_scoped_reg_x_64(SizeX64::qword, &mut tmp);
                            callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                                SizeX64::qword,
                                OperandX64::imm(self.int_op((*get_op_mut(inst, 1)))),
                                IrOp::default(),
                            );
                            callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                                SizeX64::qword,
                                OperandX64::mem(
                                    SizeX64::qword,
                                    RegisterX64::noreg,
                                    1,
                                    tmp2.release(),
                                    (core::mem::offset_of!(global_State, tmname) as i32)
                                        + self.int_op((*get_op_mut(inst, 1)))
                                            * (core::mem::size_of::<*mut TString>() as i32),
                                ),
                                IrOp::default(),
                            );
                            callWrap.set_result_register(inst.reg_x64, index);
                            callWrap.call(&OperandX64::mem(
                                SizeX64::qword,
                                RegisterX64::noreg,
                                1,
                                r_native_context(),
                                (core::mem::offset_of!(NativeContext, luaT_gettm) as i32),
                            ));
                        }

                        (*self.build)
                            .test(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));
                        (*self.build)
                            .jcc(ConditionX64::Zero, self.label_op((*get_op_mut(inst, 2))));
                        // No tag method
                    }
                }
                IrCmd::NEW_USERDATA => {
                    let mut callWrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
                        &mut self.regs,
                        &mut *self.build,
                        index,
                    );
                    callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                        SizeX64::qword,
                        OperandX64::reg(r_state()),
                        IrOp::default(),
                    );
                    callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                        SizeX64::qword,
                        OperandX64::imm(self.int_op((*get_op_mut(inst, 0)))),
                        IrOp::default(),
                    );
                    callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                        SizeX64::dword,
                        OperandX64::imm(self.int_op((*get_op_mut(inst, 1)))),
                        IrOp::default(),
                    );
                    callWrap.call(&OperandX64::mem(
                        SizeX64::qword,
                        RegisterX64::noreg,
                        1,
                        r_native_context(),
                        (core::mem::offset_of!(NativeContext, newUserdata) as i32),
                    ));
                    inst.reg_x64 = self.regs.take_reg(RegisterX64::rax, index);
                }
                IrCmd::INT_TO_NUM => {
                    inst.reg_x64 = self.regs.alloc_reg(SizeX64::xmmword, index);

                    (*self.build).vcvtsi2sd(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                    );
                }
                IrCmd::UINT_TO_NUM => {
                    inst.reg_x64 = self.regs.alloc_reg(SizeX64::xmmword, index);

                    // AVX has no uint->double conversion; the source must come from UINT op and they all should clear top 32 bits so we can usually
                    // use 64-bit reg; the one exception is NUM_TO_UINT which doesn't clear top bits
                    let source = (*self.function)
                        .inst_op((*get_op_mut(inst, (((0) as u32) as u32))))
                        .cmd;
                    if source == IrCmd::NUM_TO_UINT {
                        let mut tmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp.alloc(SizeX64::dword);
                        (*self.build).mov(
                            OperandX64::reg(tmp.reg),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                        );
                        (*self.build).vcvtsi2sd(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(qword_reg(tmp.reg)),
                        );
                    } else {
                        CODEGEN_ASSERT!(source != IrCmd::SUBSTITUTE); // we don't process substitutions
                        (*self.build).vcvtsi2sd(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(qword_reg(self.reg_op((*get_op_mut(inst, 0))))),
                        );
                    }
                }
                IrCmd::UINT_TO_FLOAT => {
                    inst.reg_x64 = self.regs.alloc_reg(SizeX64::xmmword, index);

                    // AVX has no uint->float conversion; the source must come from UINT op and they all should clear top 32 bits so we can usually
                    // use 64-bit reg; the one exception is NUM_TO_UINT which doesn't clear top bits
                    let source = (*self.function)
                        .inst_op((*get_op_mut(inst, (((0) as u32) as u32))))
                        .cmd;
                    if source == IrCmd::NUM_TO_UINT {
                        let mut tmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp.alloc(SizeX64::dword);
                        (*self.build).mov(
                            OperandX64::reg(tmp.reg),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                        );
                        (*self.build).vcvtsi2ss(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(qword_reg(tmp.reg)),
                        );
                    } else {
                        CODEGEN_ASSERT!(source != IrCmd::SUBSTITUTE); // we don't process substitutions
                        (*self.build).vcvtsi2ss(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(qword_reg(self.reg_op((*get_op_mut(inst, 0))))),
                        );
                    }
                }
                IrCmd::NUM_TO_INT => {
                    inst.reg_x64 = self.regs.alloc_reg(SizeX64::dword, index);

                    (*self.build).vcvttsd2si(
                        OperandX64::reg(inst.reg_x64),
                        self.mem_reg_double_op((*get_op_mut(inst, 0))),
                    );
                }
                IrCmd::NUM_TO_UINT => {
                    inst.reg_x64 = self.regs.alloc_reg(SizeX64::dword, index);

                    // Note: we perform 'uint64_t = (long long)double' for consistency with C++ code
                    (*self.build).vcvttsd2si(
                        OperandX64::reg(qword_reg(inst.reg_x64)),
                        self.mem_reg_double_op((*get_op_mut(inst, 0))),
                    );
                }
                IrCmd::FLOAT_TO_NUM => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );

                    (*self.build).vcvtss2sd(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(inst.reg_x64),
                        self.mem_reg_double_op((*get_op_mut(inst, 0))),
                    );
                }
                IrCmd::NUM_TO_FLOAT => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );

                    (*self.build).vcvtsd2ss(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(inst.reg_x64),
                        self.mem_reg_double_op((*get_op_mut(inst, 0))),
                    );
                }
                IrCmd::FLOAT_TO_VEC => {
                    inst.reg_x64 = self.regs.alloc_reg(SizeX64::xmmword, index);

                    if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Constant {
                        let mut value = ((self.double_op((*get_op_mut(inst, 0)))) as f32);
                        let mut asU32: u32 = 0;
                        const _: () =
                            assert!(core::mem::size_of::<u32>() == core::mem::size_of::<f32>());
                        core::ptr::copy_nonoverlapping(
                            &value as *const f32 as *const u32,
                            &mut asU32 as *mut u32,
                            1,
                        );

                        (*self.build).vmovaps(
                            OperandX64::reg(inst.reg_x64),
                            (*self.build).u32x4(asU32, asU32, asU32, 0),
                        );
                    } else {
                        (*self.build).vpshufps(
                            inst.reg_x64,
                            self.reg_op((*get_op_mut(inst, 0))),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                            0b00_00_00_00,
                        );
                    }
                }
                IrCmd::TAG_VECTOR => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::xmmword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );

                    (*self.build).vpinsrd(
                        inst.reg_x64,
                        self.reg_op((*get_op_mut(inst, 0))),
                        (*self.build).i32(lua_Type::LUA_TVECTOR as i32),
                        (((3) as u8) as u8),
                    );
                }
                IrCmd::TRUNCATE_UINT => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::dword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );

                    // Might generate mov with the same source and destination register which is not a no-op
                    (*self.build).mov(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                    );
                }
                IrCmd::ADJUST_STACK_TO_REG => {
                    let mut tmp = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };
                    tmp.alloc(SizeX64::qword);

                    if (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Constant {
                        (*self.build).lea_operand_x_64_operand_x_64(
                            OperandX64::reg(tmp.reg),
                            OperandX64::mem(
                                SizeX64::none,
                                RegisterX64::noreg,
                                1,
                                r_base(),
                                (vm_reg_op((*get_op_mut(inst, 0)))
                                    + self.int_op((*get_op_mut(inst, 1))))
                                    * (core::mem::size_of::<TValue>() as i32),
                            ),
                        );
                        (*self.build).mov(
                            OperandX64::mem(
                                SizeX64::qword,
                                RegisterX64::noreg,
                                1,
                                r_state(),
                                (core::mem::offset_of!(lua_State, top) as i32),
                            ),
                            OperandX64::reg(tmp.reg),
                        );
                    } else if (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Inst {
                        (*self.build).mov(
                            OperandX64::reg(dword_reg(tmp.reg)),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 1)))),
                        );
                        (*self.build)
                            .shl(OperandX64::reg(tmp.reg), OperandX64::imm(kTValueSizeLog2));
                        (*self.build).lea_operand_x_64_operand_x_64(
                            OperandX64::reg(tmp.reg),
                            OperandX64::mem(
                                SizeX64::none,
                                tmp.reg,
                                1,
                                r_base(),
                                vm_reg_op((*get_op_mut(inst, 0)))
                                    * (core::mem::size_of::<TValue>() as i32),
                            ),
                        );
                        (*self.build).mov(
                            OperandX64::mem(
                                SizeX64::qword,
                                RegisterX64::noreg,
                                1,
                                r_state(),
                                (core::mem::offset_of!(lua_State, top) as i32),
                            ),
                            OperandX64::reg(tmp.reg),
                        );
                    } else {
                        CODEGEN_ASSERT!(false, "Unsupported instruction form");
                    }
                }
                IrCmd::ADJUST_STACK_TO_TOP => {
                    let mut tmp = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };
                    tmp.alloc(SizeX64::qword);
                    (*self.build).mov(
                        OperandX64::reg(tmp.reg),
                        OperandX64::mem(
                            SizeX64::qword,
                            RegisterX64::noreg,
                            1,
                            r_state(),
                            (core::mem::offset_of!(lua_State, ci) as i32),
                        ),
                    );
                    (*self.build).mov(
                        OperandX64::reg(tmp.reg),
                        OperandX64::mem(
                            SizeX64::qword,
                            RegisterX64::noreg,
                            1,
                            tmp.reg,
                            (core::mem::offset_of!(CallInfo, top) as i32),
                        ),
                    );
                    (*self.build).mov(
                        OperandX64::mem(
                            SizeX64::qword,
                            RegisterX64::noreg,
                            1,
                            r_state(),
                            (core::mem::offset_of!(lua_State, top) as i32),
                        ),
                        OperandX64::reg(tmp.reg),
                    );
                }
                IrCmd::FASTCALL => {
                    let bfid = self.uint_op((*get_op_mut(inst, 0))) as i32;
                    let ra = vm_reg_op((*get_op_mut(inst, 1)));
                    let arg = vm_reg_op((*get_op_mut(inst, 2)));
                    let nparams = self.int_op((*get_op_mut(inst, 3)));
                    emit_builtin(&mut self.regs, &mut *self.build, bfid, ra, arg, nparams);
                }
                IrCmd::INVOKE_FASTCALL => {
                    {
                        let mut bfid = self.uint_op((*get_op_mut(inst, 0)));

                        let mut args = OperandX64::imm(0);
                        let mut argsAlt = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };

                        // 'E' argument can only be produced by LOP_FASTCALL3
                        if (*get_op_mut(inst, (((4) as u32) as u32))).kind() != IrOpKind::Undef {
                            CODEGEN_ASSERT!(self.int_op((*get_op_mut(inst, 5))) == 3);

                            let mut tmp = ScopedRegX64 {
                                owner: &mut self.regs,
                                reg: RegisterX64::noreg,
                            };
                            tmp.alloc(SizeX64::xmmword);
                            argsAlt.alloc(SizeX64::qword);

                            (*self.build).mov(
                                OperandX64::reg(argsAlt.reg),
                                OperandX64::mem(
                                    SizeX64::qword,
                                    RegisterX64::noreg,
                                    1,
                                    r_state(),
                                    (core::mem::offset_of!(lua_State, top) as i32),
                                ),
                            );

                            (*self.build).vmovups(
                                OperandX64::reg(tmp.reg),
                                luau_reg(vm_reg_op((*get_op_mut(inst, 3)))),
                            );
                            (*self.build).vmovups(
                                OperandX64::mem(
                                    SizeX64::xmmword,
                                    RegisterX64::noreg,
                                    1,
                                    argsAlt.reg,
                                    0,
                                ),
                                OperandX64::reg(tmp.reg),
                            );

                            (*self.build).vmovups(
                                OperandX64::reg(tmp.reg),
                                luau_reg(vm_reg_op((*get_op_mut(inst, 4)))),
                            );
                            (*self.build).vmovups(
                                OperandX64::mem(
                                    SizeX64::xmmword,
                                    RegisterX64::noreg,
                                    1,
                                    argsAlt.reg,
                                    (core::mem::size_of::<TValue>() as i32),
                                ),
                                OperandX64::reg(tmp.reg),
                            );
                        } else {
                            if (*get_op_mut(inst, (((3) as u32) as u32))).kind() == IrOpKind::VmReg
                            {
                                args = luau_reg_address(vm_reg_op((*get_op_mut(inst, 3))));
                            } else if (*get_op_mut(inst, (((3) as u32) as u32))).kind()
                                == IrOpKind::VmConst
                            {
                                args = luau_constant_address(vm_const_op((*get_op_mut(inst, 3))));
                            } else {
                                CODEGEN_ASSERT!(
                                    (*get_op_mut(inst, (((3) as u32) as u32))).kind()
                                        == IrOpKind::Undef
                                );
                            }
                        }

                        let mut ra = vm_reg_op((*get_op_mut(inst, 1)));
                        let mut arg = vm_reg_op((*get_op_mut(inst, 2)));
                        let mut nparams = self.int_op((*get_op_mut(inst, 5)));
                        let mut nresults = self.int_op((*get_op_mut(inst, 6)));

                        let mut callWrap =
                            IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
                                &mut self.regs,
                                &mut *self.build,
                                index,
                            );
                        callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                            SizeX64::qword,
                            OperandX64::reg(r_state()),
                            IrOp::default(),
                        );
                        callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                            SizeX64::qword,
                            luau_reg_address(ra),
                            IrOp::default(),
                        );
                        callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                            SizeX64::qword,
                            luau_reg_address(arg),
                            IrOp::default(),
                        );
                        callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                            SizeX64::dword,
                            OperandX64::imm(nresults),
                            IrOp::default(),
                        );

                        if (*get_op_mut(inst, (((4) as u32) as u32))).kind() != IrOpKind::Undef {
                            callWrap.add_argument_size_x_64_scoped_reg_x_64(
                                SizeX64::qword,
                                &mut argsAlt,
                            );
                        } else {
                            callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                                SizeX64::qword,
                                args,
                                IrOp::default(),
                            );
                        }

                        if nparams == LUA_MULTRET {
                            let mut reg = callWrap.suggest_next_argument_register(SizeX64::qword);
                            let mut tmp = ScopedRegX64 {
                                owner: &mut self.regs,
                                reg: RegisterX64::noreg,
                            };
                            tmp.alloc(SizeX64::qword);

                            // L->top - (ra + 1)
                            (*self.build).mov(
                                OperandX64::reg(reg),
                                OperandX64::mem(
                                    SizeX64::qword,
                                    RegisterX64::noreg,
                                    1,
                                    r_state(),
                                    (core::mem::offset_of!(lua_State, top) as i32),
                                ),
                            );
                            (*self.build).lea_operand_x_64_operand_x_64(
                                OperandX64::reg(tmp.reg),
                                OperandX64::mem(
                                    SizeX64::none,
                                    RegisterX64::noreg,
                                    1,
                                    r_base(),
                                    (ra + 1) * (core::mem::size_of::<TValue>() as i32),
                                ),
                            );
                            (*self.build).sub(OperandX64::reg(reg), OperandX64::reg(tmp.reg));
                            (*self.build)
                                .shr(OperandX64::reg(reg), OperandX64::imm(kTValueSizeLog2));

                            callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                                SizeX64::dword,
                                OperandX64::reg(dword_reg(reg)),
                                IrOp::default(),
                            );
                        } else {
                            callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                                SizeX64::dword,
                                OperandX64::imm(nparams),
                                IrOp::default(),
                            );
                        }

                        let mut func = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        func.alloc(SizeX64::qword);
                        (*self.build).mov(
                            OperandX64::reg(func.reg),
                            OperandX64::mem(
                                SizeX64::qword,
                                RegisterX64::noreg,
                                1,
                                r_native_context(),
                                (core::mem::offset_of!(NativeContext, luauF_table) as i32)
                                    + (bfid as i32)
                                        * (core::mem::size_of::<luau_FastFunction>() as i32),
                            ),
                        );

                        callWrap.call(&OperandX64::reg(func.release()));
                        inst.reg_x64 = self.regs.take_reg(dword_reg(RegisterX64::rax), index);
                        // Result of a builtin call is returned in eax
                        // Skipping high register bits clear, only consumer is CHECK_FASTCALL_RES which doesn't read them
                    }
                }
                IrCmd::CHECK_FASTCALL_RES => {
                    {
                        let mut res = self.reg_op((*get_op_mut(inst, 0)));

                        (*self.build).test(OperandX64::reg(res), OperandX64::reg(res)); // test here will set SF=1 for a negative number and it always sets OF to 0
                        (*self.build)
                            .jcc(ConditionX64::Less, self.label_op((*get_op_mut(inst, 1))));
                        // jl jumps if SF != OF
                    }
                }
                IrCmd::DO_ARITH => {
                    let mut opb =
                        if (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::VmReg {
                            luau_reg_address(vm_reg_op((*get_op_mut(inst, 1))))
                        } else {
                            luau_constant_address(vm_const_op((*get_op_mut(inst, 1))))
                        };
                    let mut opc =
                        if (*get_op_mut(inst, (((2) as u32) as u32))).kind() == IrOpKind::VmReg {
                            luau_reg_address(vm_reg_op((*get_op_mut(inst, 2))))
                        } else {
                            luau_constant_address(vm_const_op((*get_op_mut(inst, 2))))
                        };
                    let ra = vm_reg_op((*get_op_mut(inst, 0)));
                    let tm = core::mem::transmute::<u32, TMS>(
                        self.int_op((*get_op_mut(inst, 3))) as u32
                    );
                    call_arith_helper(&mut self.regs, &mut *self.build, ra, opb, opc, tm);
                }
                IrCmd::DO_LEN => {
                    call_length_helper(
                        &mut self.regs,
                        &mut *self.build,
                        vm_reg_op((*get_op_mut(inst, 0))),
                        vm_reg_op((*get_op_mut(inst, 1))),
                    );
                }
                IrCmd::GET_TABLE => {
                    if (*get_op_mut(inst, (((2) as u32) as u32))).kind() == IrOpKind::VmReg {
                        call_get_table(
                            &mut self.regs,
                            &mut *self.build,
                            vm_reg_op((*get_op_mut(inst, 1))),
                            luau_reg_address(vm_reg_op((*get_op_mut(inst, 2)))),
                            vm_reg_op((*get_op_mut(inst, 0))),
                        );
                    } else if (*get_op_mut(inst, (((2) as u32) as u32))).kind()
                        == IrOpKind::Constant
                    {
                        let mut n = TValue::default();
                        setnvalue!(
                            &mut n as *mut TValue,
                            self.uint_op((*get_op_mut(inst, 2))) as f64
                        );
                        call_get_table(
                            &mut self.regs,
                            &mut *self.build,
                            vm_reg_op((*get_op_mut(inst, 1))),
                            (*self.build).bytes(
                                &n as *const TValue as *const core::ffi::c_void,
                                core::mem::size_of::<TValue>(),
                                8,
                            ),
                            vm_reg_op((*get_op_mut(inst, 0))),
                        );
                    } else {
                        CODEGEN_ASSERT!(false, "Unsupported instruction form");
                    }
                }
                IrCmd::SET_TABLE => {
                    if (*get_op_mut(inst, (((2) as u32) as u32))).kind() == IrOpKind::VmReg {
                        call_set_table(
                            &mut self.regs,
                            &mut *self.build,
                            vm_reg_op((*get_op_mut(inst, 1))),
                            luau_reg_address(vm_reg_op((*get_op_mut(inst, 2)))),
                            vm_reg_op((*get_op_mut(inst, 0))),
                        );
                    } else if (*get_op_mut(inst, (((2) as u32) as u32))).kind()
                        == IrOpKind::Constant
                    {
                        let mut n = TValue::default();
                        setnvalue!(
                            &mut n as *mut TValue,
                            self.uint_op((*get_op_mut(inst, 2))) as f64
                        );
                        call_set_table(
                            &mut self.regs,
                            &mut *self.build,
                            vm_reg_op((*get_op_mut(inst, 1))),
                            (*self.build).bytes(
                                &n as *const TValue as *const core::ffi::c_void,
                                core::mem::size_of::<TValue>(),
                                8,
                            ),
                            vm_reg_op((*get_op_mut(inst, 0))),
                        );
                    } else {
                        CODEGEN_ASSERT!(false, "Unsupported instruction form");
                    }
                }
                IrCmd::GET_CACHED_IMPORT => {
                    {
                        self.regs.assert_all_free();
                        self.regs.assert_no_spills();

                        let mut skip = Label::default();
                        let mut exit = Label::default();

                        // If the constant for the import is set, we will use it directly, otherwise we have to call an import path lookup function
                        (*self.build).cmp(
                            luau_constant_tag(vm_const_op((*get_op_mut(inst, 1)))),
                            OperandX64::imm((lua_Type::LUA_TNIL as u8) as i32),
                        );
                        (*self.build).jcc(ConditionX64::NotEqual, &mut skip);

                        {
                            let mut spillGuard = ScopedSpills {
                                owner: core::ptr::null_mut(),
                                start_spill_id: 0,
                            };
                            spillGuard
                                .scoped_spills_scoped_spills_ir_reg_alloc_x_64(&mut self.regs);

                            let mut callWrap =
                                IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
                                    &mut self.regs,
                                    &mut *self.build,
                                    index,
                                );
                            callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                                SizeX64::qword,
                                OperandX64::reg(r_state()),
                                IrOp::default(),
                            );
                            callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                                SizeX64::qword,
                                luau_reg_address(vm_reg_op((*get_op_mut(inst, 0)))),
                                IrOp::default(),
                            );
                            callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                                SizeX64::dword,
                                OperandX64::imm(self.import_op((*get_op_mut(inst, 2))) as i32),
                                IrOp::default(),
                            );
                            callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                                SizeX64::dword,
                                OperandX64::imm(self.uint_op((*get_op_mut(inst, 3))) as i32),
                                IrOp::default(),
                            );
                            callWrap.call(&OperandX64::mem(
                                SizeX64::qword,
                                RegisterX64::noreg,
                                1,
                                r_native_context(),
                                (core::mem::offset_of!(NativeContext, getImport) as i32),
                            ));

                            emit_update_base(&mut *self.build);
                        }

                        (*self.build).jmp_label(&mut exit);

                        (*self.build).set_label(&mut skip);

                        let mut tmp1 = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp1.alloc(SizeX64::xmmword);

                        (*self.build).vmovups(
                            OperandX64::reg(tmp1.reg),
                            luau_constant(vm_const_op((*get_op_mut(inst, 1)))),
                        );
                        (*self.build).vmovups(
                            luau_reg(vm_reg_op((*get_op_mut(inst, 0)))),
                            OperandX64::reg(tmp1.reg),
                        );
                        (*self.build).set_label(&mut exit);
                    }
                }
                IrCmd::CONCAT => {
                    let mut callWrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
                        &mut self.regs,
                        &mut *self.build,
                        index,
                    );
                    callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                        SizeX64::qword,
                        OperandX64::reg(r_state()),
                        IrOp::default(),
                    );
                    callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                        SizeX64::dword,
                        OperandX64::imm(self.uint_op((*get_op_mut(inst, 1))) as i32),
                        IrOp::default(),
                    );
                    callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                        SizeX64::dword,
                        OperandX64::imm(
                            vm_reg_op((*get_op_mut(inst, 0)))
                                + self.uint_op((*get_op_mut(inst, 1))) as i32
                                - 1,
                        ),
                        IrOp::default(),
                    );
                    callWrap.call(&OperandX64::mem(
                        SizeX64::qword,
                        RegisterX64::noreg,
                        1,
                        r_native_context(),
                        (core::mem::offset_of!(NativeContext, luaV_concat) as i32),
                    ));

                    emit_update_base(&mut *self.build);
                }
                IrCmd::GET_UPVALUE => {
                    {
                        inst.reg_x64 = self.regs.alloc_reg(SizeX64::xmmword, index);

                        let mut tmp1 = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp1.alloc(SizeX64::qword);

                        (*self.build).mov(OperandX64::reg(tmp1.reg), sClosure);
                        (*self.build).add(
                            OperandX64::reg(tmp1.reg),
                            OperandX64::imm(
                                kClosureLUprefsOffset
                                    + (core::mem::size_of::<TValue>() as i32)
                                        * vm_upvalue_op((*get_op_mut(inst, 0))) as i32,
                            ),
                        );

                        // uprefs[] is either an actual value, or it points to UpVal object which has a pointer to value
                        let mut skip = Label::default();
                        (*self.build).cmp(
                            OperandX64::mem(
                                SizeX64::dword,
                                RegisterX64::noreg,
                                1,
                                tmp1.reg,
                                (core::mem::offset_of!(TValue, tt) as i32),
                            ),
                            OperandX64::imm((lua_Type::LUA_TUPVAL as u8) as i32),
                        );
                        (*self.build).jcc(ConditionX64::NotEqual, &mut skip);

                        // UpVal.v points to the value (either on stack, or on heap inside each UpVal, but we can deref it unconditionally)
                        (*self.build).mov(
                            OperandX64::reg(tmp1.reg),
                            OperandX64::mem(
                                SizeX64::qword,
                                RegisterX64::noreg,
                                1,
                                tmp1.reg,
                                (core::mem::offset_of!(TValue, value.gc) as i32),
                            ),
                        );
                        (*self.build).mov(
                            OperandX64::reg(tmp1.reg),
                            OperandX64::mem(
                                SizeX64::qword,
                                RegisterX64::noreg,
                                1,
                                tmp1.reg,
                                (core::mem::offset_of!(UpVal, v) as i32),
                            ),
                        );

                        (*self.build).set_label(&mut skip);

                        (*self.build).vmovups(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::mem(SizeX64::xmmword, RegisterX64::noreg, 1, tmp1.reg, 0),
                        );
                    }
                }
                IrCmd::SET_UPVALUE => {
                    let mut tmp1 = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };
                    tmp1.alloc(SizeX64::qword);
                    let mut tmp2 = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };
                    tmp2.alloc(SizeX64::qword);

                    (*self.build).mov(OperandX64::reg(tmp1.reg), sClosure);
                    (*self.build).mov(
                        OperandX64::reg(tmp2.reg),
                        OperandX64::mem(
                            SizeX64::qword,
                            RegisterX64::noreg,
                            1,
                            tmp1.reg,
                            kClosureLUprefsOffset
                                + (core::mem::size_of::<TValue>() as i32)
                                    * vm_upvalue_op((*get_op_mut(inst, 0))) as i32
                                + (core::mem::offset_of!(TValue, value.gc) as i32),
                        ),
                    );

                    (*self.build).mov(
                        OperandX64::reg(tmp1.reg),
                        OperandX64::mem(
                            SizeX64::qword,
                            RegisterX64::noreg,
                            1,
                            tmp2.reg,
                            (core::mem::offset_of!(UpVal, v) as i32),
                        ),
                    );
                    (*self.build).vmovups(
                        OperandX64::mem(SizeX64::xmmword, RegisterX64::noreg, 1, tmp1.reg, 0),
                        OperandX64::reg(self.reg_op((*get_op_mut(inst, 1)))),
                    );

                    tmp1.free();

                    if (*get_op_mut(inst, (((2) as u32) as u32))).kind() == IrOpKind::Undef
                        || is_gco((((self.tag_op((*get_op_mut(inst, 2)))) as u8) as u8))
                    {
                        let object = tmp2.release();
                        let value_op = *get_op_mut(inst, 1);
                        let value = self.reg_op(value_op);
                        let tag_op = *get_op_mut(inst, 2);
                        let ratag = if tag_op.kind() == IrOpKind::Undef {
                            -1
                        } else {
                            self.tag_op(tag_op) as i32
                        };
                        call_barrier_object(
                            &mut self.regs,
                            &mut *self.build,
                            object,
                            IrOp::default(),
                            value,
                            value_op,
                            ratag,
                        );
                    }
                }
                IrCmd::CHECK_TAG => {
                    (*self.build).cmp(
                        self.mem_reg_tag_op((*get_op_mut(inst, 0))),
                        OperandX64::imm((self.tag_op((*get_op_mut(inst, 1)))) as i32),
                    );
                    self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                        ConditionX64::NotEqual,
                        (*get_op_mut(inst, 2)),
                        index,
                        next,
                    );
                }
                IrCmd::CHECK_TRUTHY => {
                    {
                        // Constant tags which don't require boolean value check should've been removed in constant folding
                        CODEGEN_ASSERT!(
                            (*get_op_mut(inst, (((0) as u32) as u32))).kind() != IrOpKind::Constant
                                || self.tag_op((*get_op_mut(inst, 0)))
                                    == lua_Type::LUA_TBOOLEAN as u8
                        );

                        let mut skip = Label::default();

                        if (*get_op_mut(inst, (((0) as u32) as u32))).kind() != IrOpKind::Constant {
                            // Fail to fallback on 'nil' (falsy)
                            (*self.build).cmp(
                                self.mem_reg_tag_op((*get_op_mut(inst, 0))),
                                OperandX64::imm((lua_Type::LUA_TNIL as u8) as i32),
                            );
                            self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                                ConditionX64::Equal,
                                (*get_op_mut(inst, 2)),
                                index,
                                next,
                            );

                            // Skip value test if it's not a boolean (truthy)
                            (*self.build).cmp(
                                self.mem_reg_tag_op((*get_op_mut(inst, 0))),
                                OperandX64::imm((lua_Type::LUA_TBOOLEAN as u8) as i32),
                            );
                            (*self.build).jcc(ConditionX64::NotEqual, &mut skip);
                        }

                        // fail to fallback on 'false' boolean value (falsy)
                        if (*get_op_mut(inst, (((1) as u32) as u32))).kind() != IrOpKind::Constant {
                            (*self.build).cmp(
                                self.mem_reg_uint_op((*get_op_mut(inst, 1))),
                                OperandX64::imm((0) as i32),
                            );
                            self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                                ConditionX64::Equal,
                                (*get_op_mut(inst, 2)),
                                index,
                                next,
                            );
                        } else {
                            if self.int_op((*get_op_mut(inst, 1))) == 0 {
                                self.jump_or_abort_on_undef_ir_op_u32_ir_block(
                                    (*get_op_mut(inst, 2)),
                                    index,
                                    next,
                                );
                            }
                        }

                        if (*get_op_mut(inst, (((0) as u32) as u32))).kind() != IrOpKind::Constant {
                            (*self.build).set_label(&mut skip);
                        }
                    }
                }
                IrCmd::CHECK_READONLY => {
                    (*self.build).cmp(
                        OperandX64::mem(
                            SizeX64::byte,
                            RegisterX64::noreg,
                            1,
                            self.reg_op((*get_op_mut(inst, 0))),
                            (core::mem::offset_of!(LuaTable, readonly) as i32),
                        ),
                        OperandX64::imm((0) as i32),
                    );
                    self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                        ConditionX64::NotEqual,
                        (*get_op_mut(inst, 1)),
                        index,
                        next,
                    );
                }
                IrCmd::CHECK_NO_METATABLE => {
                    (*self.build).cmp(
                        OperandX64::mem(
                            SizeX64::qword,
                            RegisterX64::noreg,
                            1,
                            self.reg_op((*get_op_mut(inst, 0))),
                            (core::mem::offset_of!(LuaTable, metatable) as i32),
                        ),
                        OperandX64::imm((0) as i32),
                    );
                    self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                        ConditionX64::NotEqual,
                        (*get_op_mut(inst, 1)),
                        index,
                        next,
                    );
                }
                IrCmd::CHECK_SAFE_ENV => {
                    self.check_safe_env((*get_op_mut(inst, 0)), index, next);
                }
                IrCmd::CHECK_ARRAY_SIZE => {
                    if (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Inst {
                        (*self.build).cmp(
                            OperandX64::mem(
                                SizeX64::dword,
                                RegisterX64::noreg,
                                1,
                                self.reg_op((*get_op_mut(inst, 0))),
                                (core::mem::offset_of!(LuaTable, sizearray) as i32),
                            ),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 1)))),
                        );
                    } else if (*get_op_mut(inst, (((1) as u32) as u32))).kind()
                        == IrOpKind::Constant
                    {
                        (*self.build).cmp(
                            OperandX64::mem(
                                SizeX64::dword,
                                RegisterX64::noreg,
                                1,
                                self.reg_op((*get_op_mut(inst, 0))),
                                (core::mem::offset_of!(LuaTable, sizearray) as i32),
                            ),
                            OperandX64::imm((self.int_op((*get_op_mut(inst, 1)))) as i32),
                        );
                    } else {
                        CODEGEN_ASSERT!(false, "Unsupported instruction form");
                    }

                    self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                        ConditionX64::BelowEqual,
                        (*get_op_mut(inst, 2)),
                        index,
                        next,
                    );
                }
                IrCmd::JUMP_SLOT_MATCH | IrCmd::CHECK_SLOT_MATCH => {
                    {
                        let mut abort = Label { id: 0, location: 0 }; // Used when guard aborts execution
                        let mismatchOp = if inst.cmd == IrCmd::JUMP_SLOT_MATCH {
                            (*get_op_mut(inst, (((3) as u32) as u32)))
                        } else {
                            (*get_op_mut(inst, (((2) as u32) as u32)))
                        };
                        let mismatch = if mismatchOp.kind() == IrOpKind::Undef {
                            &mut abort as *mut Label
                        } else {
                            self.label_op(mismatchOp) as *mut Label
                        };

                        let mut tmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp.alloc(SizeX64::qword);

                        // Check if node key tag is a string
                        (*self.build).mov(
                            OperandX64::reg(dword_reg(tmp.reg)),
                            luau_node_key_tag(self.reg_op((*get_op_mut(inst, 0)))),
                        );
                        (*self.build).and_(
                            OperandX64::reg(dword_reg(tmp.reg)),
                            OperandX64::imm(kTKeyTagMask),
                        );
                        (*self.build).cmp(
                            OperandX64::reg(dword_reg(tmp.reg)),
                            OperandX64::imm((lua_Type::LUA_TSTRING as u8) as i32),
                        );
                        (*self.build).jcc(ConditionX64::NotEqual, &mut *mismatch);

                        // Check that node key value matches the expected one
                        (*self.build).mov(
                            OperandX64::reg(tmp.reg),
                            luau_constant_value(vm_const_op((*get_op_mut(inst, 1)))),
                        );
                        (*self.build).cmp(
                            OperandX64::reg(tmp.reg),
                            luau_node_key_value(self.reg_op((*get_op_mut(inst, 0)))),
                        );
                        (*self.build).jcc(ConditionX64::NotEqual, &mut *mismatch);

                        // Check that node value is not nil
                        (*self.build).cmp(
                            OperandX64::mem(
                                SizeX64::dword,
                                RegisterX64::noreg,
                                1,
                                self.reg_op((*get_op_mut(inst, 0))),
                                (core::mem::offset_of!(LuaNode, val) as i32)
                                    + (core::mem::offset_of!(TValue, tt) as i32),
                            ),
                            OperandX64::imm((lua_Type::LUA_TNIL as u8) as i32),
                        );
                        (*self.build).jcc(ConditionX64::Equal, &mut *mismatch);

                        if inst.cmd == IrCmd::JUMP_SLOT_MATCH {
                            let target_block =
                                self.block_op((*get_op_mut(inst, 2))) as *mut IrBlock;
                            self.jump_or_fallthrough(&mut *target_block, next);
                        } else if mismatchOp.kind() == IrOpKind::Undef {
                            let mut skip = Label { id: 0, location: 0 };
                            (*self.build).jmp_label(&mut skip);
                            (*self.build).set_label(&mut abort);
                            (*self.build).ud_2();
                            (*self.build).set_label(&mut skip);
                        }
                    }
                }
                IrCmd::CHECK_NODE_NO_NEXT => {
                    let mut tmp = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };
                    tmp.alloc(SizeX64::dword);

                    (*self.build).mov(
                        OperandX64::reg(tmp.reg),
                        OperandX64::mem(
                            SizeX64::dword,
                            RegisterX64::noreg,
                            1,
                            self.reg_op((*get_op_mut(inst, 0))),
                            (core::mem::offset_of!(LuaNode, key) as i32) + kOffsetOfTKeyTagNext,
                        ),
                    );
                    (*self.build).shr(OperandX64::reg(tmp.reg), OperandX64::imm(kTKeyTagBits));
                    self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                        ConditionX64::NotZero,
                        (*get_op_mut(inst, 1)),
                        index,
                        next,
                    );
                }
                IrCmd::CHECK_NODE_VALUE => {
                    (*self.build).cmp(
                        OperandX64::mem(
                            SizeX64::dword,
                            RegisterX64::noreg,
                            1,
                            self.reg_op((*get_op_mut(inst, 0))),
                            (core::mem::offset_of!(LuaNode, val) as i32)
                                + (core::mem::offset_of!(TValue, tt) as i32),
                        ),
                        OperandX64::imm((lua_Type::LUA_TNIL as u8) as i32),
                    );
                    self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                        ConditionX64::Equal,
                        (*get_op_mut(inst, 1)),
                        index,
                        next,
                    );
                }
                IrCmd::CHECK_BUFFER_LEN => {
                    {
                        if FFlag::LuauCodegenVmExitSync.get() {
                            let mut minOffset = self.int_op((*get_op_mut(inst, 2)));
                            let mut maxOffset = self.int_op((*get_op_mut(inst, 3)));
                            CODEGEN_ASSERT!(minOffset < maxOffset);

                            let mut accessSize = maxOffset - minOffset;
                            CODEGEN_ASSERT!(accessSize > 0);

                            // Determine which registers we will need
                            let mut hasIntegerCheck = (*get_op_mut(inst, (((4) as u32) as u32)))
                                .kind()
                                != IrOpKind::Undef;
                            let mut needsExtendedBoundsRegs =
                                (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Inst
                                    && !(accessSize == 1 && minOffset == 0);

                            // For jumps to exit sync blocks to work, we need the same register allocation state at each potential taken branch
                            let mut regA = if (*get_op_mut(inst, (((0) as u32) as u32))).kind()
                                == IrOpKind::Inst
                            {
                                self.reg_op((*get_op_mut(inst, 0)))
                            } else {
                                RegisterX64::noreg
                            };
                            let mut regB = if (*get_op_mut(inst, (((1) as u32) as u32))).kind()
                                == IrOpKind::Inst
                            {
                                self.reg_op((*get_op_mut(inst, 1)))
                            } else {
                                RegisterX64::noreg
                            };
                            let mut regE = if hasIntegerCheck {
                                self.reg_op((*get_op_mut(inst, 4)))
                            } else {
                                RegisterX64::noreg
                            };

                            let mut tmpXmm = ScopedRegX64 {
                                owner: &mut self.regs,
                                reg: RegisterX64::noreg,
                            };
                            let mut tmp1 = ScopedRegX64 {
                                owner: &mut self.regs,
                                reg: RegisterX64::noreg,
                            };
                            let mut tmp2 = ScopedRegX64 {
                                owner: &mut self.regs,
                                reg: RegisterX64::noreg,
                            };

                            if hasIntegerCheck {
                                tmpXmm.alloc(SizeX64::xmmword);
                            }

                            if needsExtendedBoundsRegs {
                                tmp1.alloc(SizeX64::qword);
                                tmp2.alloc(SizeX64::dword);
                            }

                            let mut fresh = Label { id: 0, location: 0 };

                            // Check if we are acting not only as a guard for the size, but as a guard that offset represents an exact integer
                            if hasIntegerCheck {
                                CODEGEN_ASSERT!(
                                    get_cmd_value_kind(
                                        (*self.function).inst_op((*get_op_mut(inst, 1))).cmd
                                    ) == IrValueKind::Int
                                );
                                CODEGEN_ASSERT!(!produces_dirty_high_register_bits(
                                    (*self.function).inst_op((*get_op_mut(inst, 1))).cmd
                                )); // Ensure that high register bits are cleared

                                // Convert integer back to double
                                (*self.build).vcvtsi2sd(
                                    OperandX64::reg(tmpXmm.reg),
                                    OperandX64::reg(tmpXmm.reg),
                                    OperandX64::reg(regB),
                                );

                                (*self.build)
                                    .vucomisd(OperandX64::reg(tmpXmm.reg), OperandX64::reg(regE)); // Sets ZF=1 if equal or NaN, PF=1 on NaN

                                // We don't allow non-integer values
                                self.jump_or_abort_on_undef_no_finalize(
                                    ConditionX64::NotZero,
                                    (*get_op_mut(inst, 5)),
                                    index,
                                    next,
                                    &mut fresh,
                                ); // exit on ZF=0
                                self.jump_or_abort_on_undef_no_finalize(
                                    ConditionX64::Parity,
                                    (*get_op_mut(inst, 5)),
                                    index,
                                    next,
                                    &mut fresh,
                                ); // exit on PF=1
                            }

                            if (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Inst {
                                CODEGEN_ASSERT!(!produces_dirty_high_register_bits(
                                    (*self.function).inst_op((*get_op_mut(inst, 1))).cmd
                                )); // Ensure that high register bits are cleared

                                if accessSize == 1 && minOffset == 0 {
                                    // Simpler check for a single byte access
                                    (*self.build).cmp(
                                        OperandX64::mem(
                                            SizeX64::dword,
                                            RegisterX64::noreg,
                                            1,
                                            regA,
                                            kBufferLenOffset,
                                        ),
                                        OperandX64::reg(regB),
                                    );
                                    self.jump_or_abort_on_undef_no_finalize(
                                        ConditionX64::BelowEqual,
                                        (*get_op_mut(inst, 5)),
                                        index,
                                        next,
                                        &mut fresh,
                                    );
                                } else {
                                    // To perform the bounds check using a single branch, we take index that is limited to a 32 bit int
                                    // Max offset is then added using a 64 bit addition
                                    // This will make sure that addition will not wrap around for values like 0xffffffff

                                    if minOffset >= 0 {
                                        (*self.build).lea_operand_x_64_operand_x_64(
                                            OperandX64::reg(tmp1.reg),
                                            OperandX64::mem(
                                                SizeX64::none,
                                                RegisterX64::noreg,
                                                1,
                                                qword_reg(regB),
                                                maxOffset,
                                            ),
                                        );
                                    } else {
                                        // When the min offset is negative, we subtract it from offset first (in 32 bits)
                                        (*self.build).lea_operand_x_64_operand_x_64(
                                            OperandX64::reg(dword_reg(tmp1.reg)),
                                            OperandX64::mem(
                                                SizeX64::none,
                                                RegisterX64::noreg,
                                                1,
                                                regB,
                                                minOffset,
                                            ),
                                        );

                                        // And then add the full access size like before
                                        (*self.build).lea_operand_x_64_operand_x_64(
                                            OperandX64::reg(tmp1.reg),
                                            OperandX64::mem(
                                                SizeX64::none,
                                                RegisterX64::noreg,
                                                1,
                                                tmp1.reg,
                                                accessSize,
                                            ),
                                        );
                                    }

                                    (*self.build).mov(
                                        OperandX64::reg(tmp2.reg),
                                        OperandX64::mem(
                                            SizeX64::dword,
                                            RegisterX64::noreg,
                                            1,
                                            regA,
                                            kBufferLenOffset,
                                        ),
                                    );
                                    (*self.build).cmp(
                                        OperandX64::reg(qword_reg(tmp2.reg)),
                                        OperandX64::reg(tmp1.reg),
                                    );
                                    self.jump_or_abort_on_undef_no_finalize(
                                        ConditionX64::Below,
                                        (*get_op_mut(inst, 5)),
                                        index,
                                        next,
                                        &mut fresh,
                                    );
                                }
                            } else if (*get_op_mut(inst, (((1) as u32) as u32))).kind()
                                == IrOpKind::Constant
                            {
                                let mut offset = self.int_op((*get_op_mut(inst, 1)));

                                let mut endOffset = if FFlag::LuauCodegenFixBufferLenCheck.get() {
                                    maxOffset
                                } else {
                                    accessSize
                                };

                                // Constant folding can take care of it, but for safety we avoid overflow/underflow cases here
                                if offset < 0
                                    || ((offset) as u32) + ((endOffset) as u32)
                                        >= ((INT_MAX) as u32)
                                {
                                    self.jump_or_abort_on_undef_no_finalize(
                                        ConditionX64::Count,
                                        (*get_op_mut(inst, 5)),
                                        index,
                                        next,
                                        &mut fresh,
                                    );
                                } else {
                                    (*self.build).cmp(
                                        OperandX64::mem(
                                            SizeX64::dword,
                                            RegisterX64::noreg,
                                            1,
                                            regA,
                                            kBufferLenOffset,
                                        ),
                                        OperandX64::imm(offset + endOffset),
                                    );
                                }

                                self.jump_or_abort_on_undef_no_finalize(
                                    ConditionX64::Below,
                                    (*get_op_mut(inst, 5)),
                                    index,
                                    next,
                                    &mut fresh,
                                );
                            } else {
                                CODEGEN_ASSERT!(false, "Unsupported instruction form");
                            }

                            self.finalize_target_label((*get_op_mut(inst, 5)), index, &mut fresh);
                        } else {
                            let mut minOffset = self.int_op((*get_op_mut(inst, 2)));
                            let mut maxOffset = self.int_op((*get_op_mut(inst, 3)));
                            CODEGEN_ASSERT!(minOffset < maxOffset);

                            let mut accessSize = maxOffset - minOffset;
                            CODEGEN_ASSERT!(accessSize > 0);

                            // Check if we are acting not only as a guard for the size, but as a guard that offset represents an exact integer
                            if (*get_op_mut(inst, (((4) as u32) as u32))).kind() != IrOpKind::Undef
                            {
                                CODEGEN_ASSERT!(
                                    get_cmd_value_kind(
                                        (*self.function).inst_op((*get_op_mut(inst, 1))).cmd
                                    ) == IrValueKind::Int
                                );
                                CODEGEN_ASSERT!(!produces_dirty_high_register_bits(
                                    (*self.function).inst_op((*get_op_mut(inst, 1))).cmd
                                )); // Ensure that high register bits are cleared

                                let mut tmp = ScopedRegX64 {
                                    owner: &mut self.regs,
                                    reg: RegisterX64::noreg,
                                };
                                tmp.alloc(SizeX64::xmmword);

                                // Convert integer back to double
                                (*self.build).vcvtsi2sd(
                                    OperandX64::reg(tmp.reg),
                                    OperandX64::reg(tmp.reg),
                                    OperandX64::reg(self.reg_op((*get_op_mut(inst, 1)))),
                                );

                                (*self.build).vucomisd(
                                    OperandX64::reg(tmp.reg),
                                    OperandX64::reg(self.reg_op((*get_op_mut(inst, 4)))),
                                ); // Sets ZF=1 if equal or NaN, PF=1 on NaN

                                // We don't allow non-integer values
                                self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                                    ConditionX64::NotZero,
                                    (*get_op_mut(inst, 5)),
                                    index,
                                    next,
                                ); // exit on ZF=0
                                self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                                    ConditionX64::Parity,
                                    (*get_op_mut(inst, 5)),
                                    index,
                                    next,
                                ); // exit on PF=1
                            }

                            if (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Inst {
                                CODEGEN_ASSERT!(!produces_dirty_high_register_bits(
                                    (*self.function).inst_op((*get_op_mut(inst, 1))).cmd
                                )); // Ensure that high register bits are cleared

                                if accessSize == 1 && minOffset == 0 {
                                    // Simpler check for a single byte access
                                    (*self.build).cmp(
                                        OperandX64::mem(
                                            SizeX64::dword,
                                            RegisterX64::noreg,
                                            1,
                                            self.reg_op((*get_op_mut(inst, 0))),
                                            kBufferLenOffset,
                                        ),
                                        OperandX64::reg(self.reg_op((*get_op_mut(inst, 1)))),
                                    );
                                    self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                                        ConditionX64::BelowEqual,
                                        (*get_op_mut(inst, 5)),
                                        index,
                                        next,
                                    );
                                } else {
                                    let mut tmp1 = ScopedRegX64 {
                                        owner: &mut self.regs,
                                        reg: RegisterX64::noreg,
                                    };
                                    tmp1.alloc(SizeX64::qword);
                                    let mut tmp2 = ScopedRegX64 {
                                        owner: &mut self.regs,
                                        reg: RegisterX64::noreg,
                                    };
                                    tmp2.alloc(SizeX64::dword);

                                    // To perform the bounds check using a single branch, we take index that is limited to a 32 bit int
                                    // Max offset is then added using a 64 bit addition
                                    // This will make sure that addition will not wrap around for values like 0xffffffff

                                    if minOffset >= 0 {
                                        (*self.build).lea_operand_x_64_operand_x_64(
                                            OperandX64::reg(tmp1.reg),
                                            OperandX64::mem(
                                                SizeX64::none,
                                                RegisterX64::noreg,
                                                1,
                                                qword_reg(self.reg_op((*get_op_mut(inst, 1)))),
                                                maxOffset,
                                            ),
                                        );
                                    } else {
                                        // When the min offset is negative, we subtract it from offset first (in 32 bits)
                                        (*self.build).lea_operand_x_64_operand_x_64(
                                            OperandX64::reg(dword_reg(tmp1.reg)),
                                            OperandX64::mem(
                                                SizeX64::none,
                                                RegisterX64::noreg,
                                                1,
                                                self.reg_op((*get_op_mut(inst, 1))),
                                                minOffset,
                                            ),
                                        );

                                        // And then add the full access size like before
                                        (*self.build).lea_operand_x_64_operand_x_64(
                                            OperandX64::reg(tmp1.reg),
                                            OperandX64::mem(
                                                SizeX64::none,
                                                RegisterX64::noreg,
                                                1,
                                                tmp1.reg,
                                                accessSize,
                                            ),
                                        );
                                    }

                                    (*self.build).mov(
                                        OperandX64::reg(tmp2.reg),
                                        OperandX64::mem(
                                            SizeX64::dword,
                                            RegisterX64::noreg,
                                            1,
                                            self.reg_op((*get_op_mut(inst, 0))),
                                            kBufferLenOffset,
                                        ),
                                    );
                                    (*self.build).cmp(
                                        OperandX64::reg(qword_reg(tmp2.reg)),
                                        OperandX64::reg(tmp1.reg),
                                    );

                                    self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                                        ConditionX64::Below,
                                        (*get_op_mut(inst, 5)),
                                        index,
                                        next,
                                    );
                                }
                            } else if (*get_op_mut(inst, (((1) as u32) as u32))).kind()
                                == IrOpKind::Constant
                            {
                                let mut offset = self.int_op((*get_op_mut(inst, 1)));

                                let mut endOffset = if FFlag::LuauCodegenFixBufferLenCheck.get() {
                                    maxOffset
                                } else {
                                    accessSize
                                };

                                // Constant folding can take care of it, but for safety we avoid overflow/underflow cases here
                                if offset < 0
                                    || ((offset) as u32) + ((endOffset) as u32)
                                        >= ((INT_MAX) as u32)
                                {
                                    self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                                        ConditionX64::Count,
                                        (*get_op_mut(inst, 5)),
                                        index,
                                        next,
                                    );
                                } else {
                                    (*self.build).cmp(
                                        OperandX64::mem(
                                            SizeX64::dword,
                                            RegisterX64::noreg,
                                            1,
                                            self.reg_op((*get_op_mut(inst, 0))),
                                            kBufferLenOffset,
                                        ),
                                        OperandX64::imm(offset + endOffset),
                                    );
                                }

                                self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                                    ConditionX64::Below,
                                    (*get_op_mut(inst, 5)),
                                    index,
                                    next,
                                );
                            } else {
                                CODEGEN_ASSERT!(false, "Unsupported instruction form");
                            }
                        }
                    }
                }
                IrCmd::CHECK_USERDATA_TAG => {
                    (*self.build).cmp(
                        OperandX64::mem(
                            SizeX64::byte,
                            RegisterX64::noreg,
                            1,
                            self.reg_op((*get_op_mut(inst, 0))),
                            (core::mem::offset_of!(Udata, tag) as i32),
                        ),
                        OperandX64::imm((self.int_op((*get_op_mut(inst, 1)))) as i32),
                    );
                    self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                        ConditionX64::NotEqual,
                        (*get_op_mut(inst, 2)),
                        index,
                        next,
                    );
                }
                IrCmd::CHECK_CMP_NUM => {
                    let mut cond = condition_op((*get_op_mut(inst, 2)));

                    let mut fresh = Label { id: 0, location: 0 };
                    let fail = self.get_target_label((*get_op_mut(inst, 3)), index, &mut fresh)
                        as *mut Label;

                    let mut tmp = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };
                    tmp.alloc(SizeX64::xmmword);

                    jump_on_number_cmp(
                        &mut *self.build,
                        tmp.reg,
                        self.mem_reg_double_op((*get_op_mut(inst, 0))),
                        self.mem_reg_double_op((*get_op_mut(inst, 1))),
                        get_negated_condition_ir_condition(cond),
                        &mut *fail,
                        false,
                    );

                    self.finalize_target_label((*get_op_mut(inst, 3)), index, &mut fresh);
                }
                IrCmd::CHECK_CMP_INT => {
                    let mut cond = condition_op((*get_op_mut(inst, 2)));

                    if (cond == IrCondition::Equal || cond == IrCondition::NotEqual)
                        && (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Constant
                        && self.int_op((*get_op_mut(inst, 1))) == 0
                    {
                        (*self.build).test(
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                        );
                        self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                            if cond == IrCondition::Equal {
                                ConditionX64::NotZero
                            } else {
                                ConditionX64::Zero
                            },
                            (*get_op_mut(inst, 3)),
                            index,
                            next,
                        );
                    } else if (*get_op_mut(inst, (((0) as u32) as u32))).kind()
                        == IrOpKind::Constant
                    {
                        let mut tmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp.alloc(SizeX64::dword);
                        (*self.build).mov(
                            OperandX64::reg(tmp.reg),
                            self.mem_reg_int_op((*get_op_mut(inst, 0))),
                        );
                        (*self.build).cmp(
                            OperandX64::reg(tmp.reg),
                            self.mem_reg_int_op((*get_op_mut(inst, 1))),
                        );
                        self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                            get_condition_int(get_negated_condition_ir_condition(cond)),
                            (*get_op_mut(inst, 3)),
                            index,
                            next,
                        );
                    } else {
                        (*self.build).cmp(
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                            self.mem_reg_int_op((*get_op_mut(inst, 1))),
                        );
                        self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                            get_condition_int(get_negated_condition_ir_condition(cond)),
                            (*get_op_mut(inst, 3)),
                            index,
                            next,
                        );
                    }
                }
                IrCmd::INTERRUPT => {
                    {
                        let mut pcpos = self.uint_op((*get_op_mut(inst, 0)));

                        // We unconditionally spill values here because that allows us to ignore register state when we synthesize interrupt handler
                        // This can be changed in the future if we can somehow record interrupt handler code separately
                        // Since interrupts are loop edges or call/ret, we don't have a significant opportunity for register reuse here anyway
                        self.regs.preserve_and_free_inst_values();

                        let mut tmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp.alloc(SizeX64::qword);

                        let mut self_lbl = Label::default();

                        (*self.build).mov(
                            OperandX64::reg(tmp.reg),
                            OperandX64::mem(
                                SizeX64::qword,
                                RegisterX64::noreg,
                                1,
                                r_state(),
                                (core::mem::offset_of!(lua_State, global) as i32),
                            ),
                        );
                        (*self.build).cmp(
                            OperandX64::mem(
                                SizeX64::qword,
                                RegisterX64::noreg,
                                1,
                                tmp.reg,
                                (core::mem::offset_of!(global_State, cb.interrupt) as i32),
                            ),
                            OperandX64::imm((0) as i32),
                        );
                        (*self.build).jcc(ConditionX64::NotEqual, &mut self_lbl);

                        let mut next = Label::default();
                        (*self.build).set_label(&mut next);

                        self.interrupt_handlers.push(InterruptHandler {
                            self_: self_lbl,
                            pcpos,
                            next,
                        });
                    }
                }
                IrCmd::CHECK_GC => {
                    call_step_gc(&mut self.regs, &mut *self.build);
                }
                IrCmd::BARRIER_OBJ => {
                    let object_op = *get_op_mut(inst, 0);
                    let object = self.reg_op(object_op);
                    let value_op = *get_op_mut(inst, 1);
                    let tag_op = *get_op_mut(inst, 2);
                    let ratag = if tag_op.kind() == IrOpKind::Undef {
                        -1
                    } else {
                        self.tag_op(tag_op) as i32
                    };
                    call_barrier_object(
                        &mut self.regs,
                        &mut *self.build,
                        object,
                        object_op,
                        RegisterX64::noreg,
                        value_op,
                        ratag,
                    );
                }
                IrCmd::BARRIER_TABLE_BACK => {
                    let table_op = *get_op_mut(inst, 0);
                    let table = self.reg_op(table_op);
                    call_barrier_table_fast(&mut self.regs, &mut *self.build, table, table_op);
                }
                IrCmd::BARRIER_TABLE_FORWARD => {
                    let mut skip = Label::default();

                    let mut tmp = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };
                    tmp.alloc(SizeX64::qword);

                    check_object_barrier_conditions(
                        &mut *self.build,
                        tmp.reg,
                        self.reg_op((*get_op_mut(inst, 0))),
                        RegisterX64::noreg,
                        (*get_op_mut(inst, 1)),
                        if (*get_op_mut(inst, 2)).kind() == IrOpKind::Undef {
                            -1
                        } else {
                            self.tag_op((*get_op_mut(inst, 2))) as i32
                        },
                        &mut skip,
                    );

                    {
                        let mut spillGuard = ScopedSpills {
                            owner: core::ptr::null_mut(),
                            start_spill_id: 0,
                        };
                        spillGuard.scoped_spills_scoped_spills_ir_reg_alloc_x_64(&mut self.regs);

                        let mut callWrap =
                            IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
                                &mut self.regs,
                                &mut *self.build,
                                index,
                            );
                        callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                            SizeX64::qword,
                            OperandX64::reg(r_state()),
                            IrOp::default(),
                        );
                        callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                            SizeX64::qword,
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                            (*get_op_mut(inst, (((0) as u32) as u32))),
                        );
                        callWrap.add_argument_size_x_64_scoped_reg_x_64(SizeX64::qword, &mut tmp);
                        callWrap.call(&OperandX64::mem(
                            SizeX64::qword,
                            RegisterX64::noreg,
                            1,
                            r_native_context(),
                            (core::mem::offset_of!(NativeContext, luaC_barriertable) as i32),
                        ));
                    }

                    (*self.build).set_label(&mut skip);
                }
                IrCmd::SET_SAVEDPC => {
                    let mut tmp1 = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };
                    tmp1.alloc(SizeX64::qword);
                    let mut tmp2 = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };
                    tmp2.alloc(SizeX64::qword);

                    (*self.build).mov(OperandX64::reg(tmp2.reg), sCode);
                    (*self.build).add(
                        OperandX64::reg(tmp2.reg),
                        OperandX64::imm(
                            (self.uint_op((*get_op_mut(inst, 0))) as i32)
                                * (core::mem::size_of::<Instruction>() as i32),
                        ),
                    );
                    (*self.build).mov(
                        OperandX64::reg(tmp1.reg),
                        OperandX64::mem(
                            SizeX64::qword,
                            RegisterX64::noreg,
                            1,
                            r_state(),
                            (core::mem::offset_of!(lua_State, ci) as i32),
                        ),
                    );
                    (*self.build).mov(
                        OperandX64::mem(
                            SizeX64::qword,
                            RegisterX64::noreg,
                            1,
                            tmp1.reg,
                            (core::mem::offset_of!(CallInfo, savedpc) as i32),
                        ),
                        OperandX64::reg(tmp2.reg),
                    );
                }
                IrCmd::CLOSE_UPVALS => {
                    {
                        let mut next = Label::default();
                        let mut tmp1 = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp1.alloc(SizeX64::qword);
                        let mut tmp2 = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp2.alloc(SizeX64::qword);

                        // L->openupval != 0
                        (*self.build).mov(
                            OperandX64::reg(tmp1.reg),
                            OperandX64::mem(
                                SizeX64::qword,
                                RegisterX64::noreg,
                                1,
                                r_state(),
                                (core::mem::offset_of!(lua_State, openupval) as i32),
                            ),
                        );
                        (*self.build).test(OperandX64::reg(tmp1.reg), OperandX64::reg(tmp1.reg));
                        (*self.build).jcc(ConditionX64::Zero, &mut next);

                        // ra <= L->openupval->v
                        (*self.build).lea_operand_x_64_operand_x_64(
                            OperandX64::reg(tmp2.reg),
                            OperandX64::mem(
                                SizeX64::none,
                                RegisterX64::noreg,
                                1,
                                r_base(),
                                vm_reg_op((*get_op_mut(inst, 0)))
                                    * (core::mem::size_of::<TValue>() as i32),
                            ),
                        );
                        (*self.build).cmp(
                            OperandX64::reg(tmp2.reg),
                            OperandX64::mem(
                                SizeX64::qword,
                                RegisterX64::noreg,
                                1,
                                tmp1.reg,
                                (core::mem::offset_of!(UpVal, v) as i32),
                            ),
                        );
                        (*self.build).jcc(ConditionX64::Above, &mut next);

                        tmp1.free();

                        {
                            let mut spillGuard = ScopedSpills {
                                owner: core::ptr::null_mut(),
                                start_spill_id: 0,
                            };
                            spillGuard
                                .scoped_spills_scoped_spills_ir_reg_alloc_x_64(&mut self.regs);

                            let mut callWrap =
                                IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
                                    &mut self.regs,
                                    &mut *self.build,
                                    index,
                                );
                            callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                                SizeX64::qword,
                                OperandX64::reg(r_state()),
                                IrOp::default(),
                            );
                            callWrap
                                .add_argument_size_x_64_scoped_reg_x_64(SizeX64::qword, &mut tmp2);
                            callWrap.call(&OperandX64::mem(
                                SizeX64::qword,
                                RegisterX64::noreg,
                                1,
                                r_native_context(),
                                (core::mem::offset_of!(NativeContext, luaF_close) as i32),
                            ));
                        }

                        (*self.build).set_label(&mut next);
                    }
                }
                IrCmd::CAPTURE => {
                    // No-op right now

                    // Fallbacks to non-IR instruction implementations
                }
                IrCmd::SETLIST => {
                    self.regs.assert_all_free();
                    let ra = vm_reg_op((*get_op_mut(inst, 1)));
                    let rb = vm_reg_op((*get_op_mut(inst, 2)));
                    let count = self.int_op((*get_op_mut(inst, 3))) as i32;
                    let index = self.uint_op((*get_op_mut(inst, 4))) as u32;
                    let aux = if (*get_op_mut(inst, 5)).kind() == IrOpKind::Undef {
                        -1
                    } else {
                        self.uint_op((*get_op_mut(inst, 5))) as i32
                    };
                    emit_inst_set_list(&mut self.regs, &mut *self.build, ra, rb, count, index, aux);
                }
                IrCmd::CALL => {
                    self.regs.assert_all_free();
                    self.regs.assert_no_spills();
                    let ra = vm_reg_op((*get_op_mut(inst, 0)));
                    let nparams = self.int_op((*get_op_mut(inst, 1))) as i32;
                    let nresults = self.int_op((*get_op_mut(inst, 2))) as i32;
                    emit_inst_call(
                        &mut self.regs,
                        &mut *self.build,
                        &mut *self.helpers,
                        ra,
                        nparams,
                        nresults,
                    );
                }
                IrCmd::RETURN => {
                    self.regs.assert_all_free();
                    self.regs.assert_no_spills();
                    emit_inst_return(
                        &mut *self.build,
                        &mut *self.helpers,
                        vm_reg_op((*get_op_mut(inst, 0))),
                        (((self.int_op((*get_op_mut(inst, 1)))) as i32) as i32),
                        (*self.function).variadic,
                    );
                }
                IrCmd::FORGLOOP => {
                    self.regs.assert_all_free();
                    let ra = vm_reg_op((*get_op_mut(inst, 0)));
                    let aux = self.int_op((*get_op_mut(inst, 1))) as i32;
                    let target = self.label_op((*get_op_mut(inst, 2))) as *mut Label;
                    emit_inst_for_g_loop(&mut self.regs, &mut *self.build, ra, aux, &mut *target);
                    let target_block = self.block_op((*get_op_mut(inst, 3))) as *mut IrBlock;
                    self.jump_or_fallthrough(&mut *target_block, next);
                }
                IrCmd::FORGLOOP_FALLBACK => {
                    let mut callWrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
                        &mut self.regs,
                        &mut *self.build,
                        index,
                    );
                    callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                        SizeX64::qword,
                        OperandX64::reg(r_state()),
                        IrOp::default(),
                    );
                    callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                        SizeX64::dword,
                        OperandX64::imm(vm_reg_op((*get_op_mut(inst, 0)))),
                        IrOp::default(),
                    );
                    callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                        SizeX64::dword,
                        OperandX64::imm(self.int_op((*get_op_mut(inst, 1)))),
                        IrOp::default(),
                    );

                    if FFlag::LuauYieldIter2.get() {
                        callWrap.call(&OperandX64::mem(
                            SizeX64::qword,
                            RegisterX64::noreg,
                            1,
                            r_native_context(),
                            (core::mem::offset_of!(NativeContext, forgLoopNonTableFallback) as i32),
                        ));

                        emit_update_base(&mut *self.build);

                        (*self.build).test(
                            OperandX64::reg(dword_reg(RegisterX64::rax)),
                            OperandX64::reg(dword_reg(RegisterX64::rax)),
                        );
                        (*self.build)
                            .jcc(ConditionX64::Less, &mut (*self.helpers).exitNoContinueVm);
                        (*self.build)
                            .jcc(ConditionX64::Greater, self.label_op((*get_op_mut(inst, 2))));
                    } else {
                        callWrap.call(&OperandX64::mem(
                            SizeX64::qword,
                            RegisterX64::noreg,
                            1,
                            r_native_context(),
                            (core::mem::offset_of!(
                                NativeContext,
                                forgLoopNonTableFallback_DEPRECATED
                            ) as i32),
                        ));

                        emit_update_base(&mut *self.build);

                        (*self.build).test(
                            OperandX64::reg(byte_reg(RegisterX64::rax)),
                            OperandX64::reg(byte_reg(RegisterX64::rax)),
                        );
                        (*self.build)
                            .jcc(ConditionX64::NotZero, self.label_op((*get_op_mut(inst, 2))));
                    }

                    let target_block = self.block_op((*get_op_mut(inst, 3))) as *mut IrBlock;
                    self.jump_or_fallthrough(&mut *target_block, next);
                }
                IrCmd::FORGPREP_XNEXT_FALLBACK => {
                    let mut callWrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
                        &mut self.regs,
                        &mut *self.build,
                        index,
                    );
                    callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                        SizeX64::qword,
                        OperandX64::reg(r_state()),
                        IrOp::default(),
                    );
                    callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                        SizeX64::qword,
                        luau_reg_address(vm_reg_op((*get_op_mut(inst, 1)))),
                        IrOp::default(),
                    );
                    callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                        SizeX64::dword,
                        OperandX64::imm(self.uint_op((*get_op_mut(inst, 0))) as i32 + 1),
                        IrOp::default(),
                    );
                    callWrap.call(&OperandX64::mem(
                        SizeX64::qword,
                        RegisterX64::noreg,
                        1,
                        r_native_context(),
                        (core::mem::offset_of!(NativeContext, forgPrepXnextFallback) as i32),
                    ));
                    let target_block = self.block_op((*get_op_mut(inst, 2))) as *mut IrBlock;
                    self.jump_or_fallthrough(&mut *target_block, next);
                }
                IrCmd::COVERAGE => {
                    {
                        let mut tmp1 = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp1.alloc(SizeX64::qword);
                        let mut tmp2 = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp2.alloc(SizeX64::dword);
                        let mut tmp3 = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp3.alloc(SizeX64::dword);

                        (*self.build).mov(OperandX64::reg(tmp1.reg), sCode);
                        (*self.build).add(
                            OperandX64::reg(tmp1.reg),
                            OperandX64::imm(
                                (self.uint_op((*get_op_mut(inst, 0))) as i32)
                                    * (core::mem::size_of::<Instruction>() as i32),
                            ),
                        );

                        // hits = LUAU_INSN_E(*pc)
                        (*self.build).mov(
                            OperandX64::reg(tmp2.reg),
                            OperandX64::mem(SizeX64::dword, RegisterX64::noreg, 1, tmp1.reg, 0),
                        );
                        (*self.build).sar(OperandX64::reg(tmp2.reg), OperandX64::imm((8) as i32));

                        // hits = if (hits < (1 << 23) - 1) { hits + 1 } else { hits };
                        (*self.build).xor_(OperandX64::reg(tmp3.reg), OperandX64::reg(tmp3.reg));
                        (*self.build)
                            .cmp(OperandX64::reg(tmp2.reg), OperandX64::imm((1 << 23) - 1));
                        (*self.build)
                            .setcc(ConditionX64::NotEqual, OperandX64::reg(byte_reg(tmp3.reg)));
                        (*self.build).add(OperandX64::reg(tmp2.reg), OperandX64::reg(tmp3.reg));

                        // VM_PATCH_E(pc, hits);
                        (*self.build).sal(OperandX64::reg(tmp2.reg), OperandX64::imm((8) as i32));
                        (*self.build).movzx(
                            tmp3.reg,
                            OperandX64::mem(SizeX64::byte, RegisterX64::noreg, 1, tmp1.reg, 0),
                        );
                        (*self.build).or_(OperandX64::reg(tmp3.reg), OperandX64::reg(tmp2.reg));
                        (*self.build).mov(
                            OperandX64::mem(SizeX64::dword, RegisterX64::noreg, 1, tmp1.reg, 0),
                            OperandX64::reg(tmp3.reg),
                        );
                    }

                    // Full instruction fallbacks
                }
                IrCmd::FALLBACK_GETGLOBAL => {
                    CODEGEN_ASSERT!(
                        (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::VmReg
                    );
                    CODEGEN_ASSERT!(
                        (*get_op_mut(inst, (((2) as u32) as u32))).kind() == IrOpKind::VmConst
                    );

                    let pcpos = self.uint_op((*get_op_mut(inst, 0))) as i32;
                    emit_fallback(
                        &mut self.regs,
                        &mut *self.build,
                        core::mem::offset_of!(NativeContext, executeGETGLOBAL) as i32,
                        pcpos,
                    );
                }
                IrCmd::FALLBACK_SETGLOBAL => {
                    CODEGEN_ASSERT!(
                        (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::VmReg
                    );
                    CODEGEN_ASSERT!(
                        (*get_op_mut(inst, (((2) as u32) as u32))).kind() == IrOpKind::VmConst
                    );

                    let pcpos = self.uint_op((*get_op_mut(inst, 0))) as i32;
                    emit_fallback(
                        &mut self.regs,
                        &mut *self.build,
                        core::mem::offset_of!(NativeContext, executeSETGLOBAL) as i32,
                        pcpos,
                    );
                }
                IrCmd::FALLBACK_GETTABLEKS => {
                    CODEGEN_ASSERT!(
                        (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::VmReg
                    );
                    CODEGEN_ASSERT!(
                        (*get_op_mut(inst, (((2) as u32) as u32))).kind() == IrOpKind::VmReg
                    );
                    CODEGEN_ASSERT!(
                        (*get_op_mut(inst, (((3) as u32) as u32))).kind() == IrOpKind::VmConst
                    );

                    let pcpos = self.uint_op((*get_op_mut(inst, 0))) as i32;
                    emit_fallback(
                        &mut self.regs,
                        &mut *self.build,
                        core::mem::offset_of!(NativeContext, executeGETTABLEKS) as i32,
                        pcpos,
                    );
                }
                IrCmd::FALLBACK_SETTABLEKS => {
                    CODEGEN_ASSERT!(
                        (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::VmReg
                    );
                    CODEGEN_ASSERT!(
                        (*get_op_mut(inst, (((2) as u32) as u32))).kind() == IrOpKind::VmReg
                    );
                    CODEGEN_ASSERT!(
                        (*get_op_mut(inst, (((3) as u32) as u32))).kind() == IrOpKind::VmConst
                    );

                    let pcpos = self.uint_op((*get_op_mut(inst, 0))) as i32;
                    emit_fallback(
                        &mut self.regs,
                        &mut *self.build,
                        core::mem::offset_of!(NativeContext, executeSETTABLEKS) as i32,
                        pcpos,
                    );
                }
                IrCmd::FALLBACK_NAMECALL => {
                    CODEGEN_ASSERT!(
                        (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::VmReg
                    );
                    CODEGEN_ASSERT!(
                        (*get_op_mut(inst, (((2) as u32) as u32))).kind() == IrOpKind::VmReg
                    );
                    CODEGEN_ASSERT!(
                        (*get_op_mut(inst, (((3) as u32) as u32))).kind() == IrOpKind::VmConst
                    );

                    let pcpos = self.uint_op((*get_op_mut(inst, 0))) as i32;
                    emit_fallback(
                        &mut self.regs,
                        &mut *self.build,
                        core::mem::offset_of!(NativeContext, executeNAMECALL) as i32,
                        pcpos,
                    );
                }
                IrCmd::FALLBACK_PREPVARARGS => {
                    CODEGEN_ASSERT!(
                        (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Constant
                    );

                    let pcpos = self.uint_op((*get_op_mut(inst, 0))) as i32;
                    emit_fallback(
                        &mut self.regs,
                        &mut *self.build,
                        core::mem::offset_of!(NativeContext, executePREPVARARGS) as i32,
                        pcpos,
                    );
                }
                IrCmd::FALLBACK_GETVARARGS => {
                    CODEGEN_ASSERT!(
                        (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::VmReg
                    );
                    CODEGEN_ASSERT!(
                        (*get_op_mut(inst, (((2) as u32) as u32))).kind() == IrOpKind::Constant
                    );

                    if self.int_op((*get_op_mut(inst, 2))) == LUA_MULTRET {
                        let mut callWrap =
                            IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
                                &mut self.regs,
                                &mut *self.build,
                                index,
                            );
                        callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                            SizeX64::qword,
                            OperandX64::reg(r_state()),
                            IrOp::default(),
                        );

                        let mut reg = callWrap.suggest_next_argument_register(SizeX64::qword);
                        (*self.build).mov(OperandX64::reg(reg), sCode);
                        callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                            SizeX64::qword,
                            OperandX64::mem(
                                SizeX64::qword,
                                RegisterX64::noreg,
                                1,
                                reg,
                                (self.uint_op((*get_op_mut(inst, 0))) as i32)
                                    * (core::mem::size_of::<Instruction>() as i32),
                            ),
                            IrOp::default(),
                        );

                        callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                            SizeX64::qword,
                            OperandX64::reg(r_base()),
                            IrOp::default(),
                        );
                        callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                            SizeX64::dword,
                            OperandX64::imm(vm_reg_op((*get_op_mut(inst, 1)))),
                            IrOp::default(),
                        );
                        callWrap.call(&OperandX64::mem(
                            SizeX64::qword,
                            RegisterX64::noreg,
                            1,
                            r_native_context(),
                            (core::mem::offset_of!(NativeContext, executeGETVARARGSMultRet) as i32),
                        ));

                        emit_update_base(&mut *self.build);
                    } else {
                        let mut callWrap =
                            IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
                                &mut self.regs,
                                &mut *self.build,
                                index,
                            );
                        callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                            SizeX64::qword,
                            OperandX64::reg(r_state()),
                            IrOp::default(),
                        );
                        callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                            SizeX64::qword,
                            OperandX64::reg(r_base()),
                            IrOp::default(),
                        );
                        callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                            SizeX64::dword,
                            OperandX64::imm(vm_reg_op((*get_op_mut(inst, 1)))),
                            IrOp::default(),
                        );
                        callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                            SizeX64::dword,
                            OperandX64::imm(self.int_op((*get_op_mut(inst, 2)))),
                            IrOp::default(),
                        );
                        callWrap.call(&OperandX64::mem(
                            SizeX64::qword,
                            RegisterX64::noreg,
                            1,
                            r_native_context(),
                            (core::mem::offset_of!(NativeContext, executeGETVARARGSConst) as i32),
                        ));
                    }
                }
                IrCmd::NEWCLOSURE => {
                    let mut tmp2 = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };
                    tmp2.alloc(SizeX64::qword);
                    (*self.build).mov(OperandX64::reg(tmp2.reg), sClosure);
                    (*self.build).mov(
                        OperandX64::reg(tmp2.reg),
                        OperandX64::mem(
                            SizeX64::qword,
                            RegisterX64::noreg,
                            1,
                            tmp2.reg,
                            kClosureLPoffset,
                        ),
                    );
                    (*self.build).mov(
                        OperandX64::reg(tmp2.reg),
                        OperandX64::mem(
                            SizeX64::qword,
                            RegisterX64::noreg,
                            1,
                            tmp2.reg,
                            (core::mem::offset_of!(Proto, p) as i32),
                        ),
                    );
                    (*self.build).mov(
                        OperandX64::reg(tmp2.reg),
                        OperandX64::mem(
                            SizeX64::qword,
                            RegisterX64::noreg,
                            1,
                            tmp2.reg,
                            (self.uint_op((*get_op_mut(inst, 2))) as i32)
                                * (core::mem::size_of::<*mut Proto>() as i32),
                        ),
                    );

                    let mut callWrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
                        &mut self.regs,
                        &mut *self.build,
                        index,
                    );
                    callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                        SizeX64::qword,
                        OperandX64::reg(r_state()),
                        IrOp::default(),
                    );
                    callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                        SizeX64::dword,
                        OperandX64::imm(self.uint_op((*get_op_mut(inst, 0))) as i32),
                        (*get_op_mut(inst, (((0) as u32) as u32))),
                    );
                    callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                        SizeX64::qword,
                        OperandX64::reg(self.reg_op((*get_op_mut(inst, 1)))),
                        (*get_op_mut(inst, (((1) as u32) as u32))),
                    );
                    callWrap.add_argument_size_x_64_scoped_reg_x_64(SizeX64::qword, &mut tmp2);

                    callWrap.call(&OperandX64::mem(
                        SizeX64::qword,
                        RegisterX64::noreg,
                        1,
                        r_native_context(),
                        (core::mem::offset_of!(NativeContext, luaF_newLclosure) as i32),
                    ));

                    inst.reg_x64 = self.regs.take_reg(RegisterX64::rax, index);
                }
                IrCmd::FALLBACK_DUPCLOSURE => {
                    CODEGEN_ASSERT!(
                        (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::VmReg
                    );
                    CODEGEN_ASSERT!(
                        (*get_op_mut(inst, (((2) as u32) as u32))).kind() == IrOpKind::VmConst
                    );

                    let pcpos = self.uint_op((*get_op_mut(inst, 0))) as i32;
                    emit_fallback(
                        &mut self.regs,
                        &mut *self.build,
                        core::mem::offset_of!(NativeContext, executeDUPCLOSURE) as i32,
                        pcpos,
                    );
                }
                IrCmd::FALLBACK_FORGPREP => {
                    let pcpos = self.uint_op((*get_op_mut(inst, 0))) as i32;
                    emit_fallback(
                        &mut self.regs,
                        &mut *self.build,
                        core::mem::offset_of!(NativeContext, executeFORGPREP) as i32,
                        pcpos,
                    );
                    let target_block = self.block_op((*get_op_mut(inst, 2))) as *mut IrBlock;
                    self.jump_or_fallthrough(&mut *target_block, next);
                }
                IrCmd::BITAND_UINT => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::dword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );
                    let op0 = *get_op_mut(inst, 0);

                    if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
                        (*self.build).mov(OperandX64::reg(inst.reg_x64), self.mem_reg_uint_op(op0));
                    }

                    (*self.build).and_(
                        OperandX64::reg(inst.reg_x64),
                        self.mem_reg_uint_op((*get_op_mut(inst, 1))),
                    );
                }
                IrCmd::BITXOR_UINT => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::dword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );
                    let op0 = *get_op_mut(inst, 0);

                    if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
                        (*self.build).mov(OperandX64::reg(inst.reg_x64), self.mem_reg_uint_op(op0));
                    }

                    (*self.build).xor_(
                        OperandX64::reg(inst.reg_x64),
                        self.mem_reg_uint_op((*get_op_mut(inst, 1))),
                    );
                }
                IrCmd::BITOR_UINT => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::dword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );
                    let op0 = *get_op_mut(inst, 0);

                    if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
                        (*self.build).mov(OperandX64::reg(inst.reg_x64), self.mem_reg_uint_op(op0));
                    }

                    (*self.build).or_(
                        OperandX64::reg(inst.reg_x64),
                        self.mem_reg_uint_op((*get_op_mut(inst, 1))),
                    );
                }
                IrCmd::BITNOT_UINT => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::dword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );
                    let op0 = *get_op_mut(inst, 0);

                    if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
                        (*self.build).mov(OperandX64::reg(inst.reg_x64), self.mem_reg_uint_op(op0));
                    }

                    (*self.build).not_(OperandX64::reg(inst.reg_x64));
                }
                IrCmd::BITLSHIFT_UINT => {
                    {
                        let mut shiftTmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };

                        // Custom bit shift value can only be placed in RegisterX64::cl
                        // but we use it if the shift value is not a constant stored in b
                        if (*get_op_mut(inst, (((1) as u32) as u32))).kind() != IrOpKind::Constant {
                            shiftTmp.take(dword_reg(RegisterX64::rcx));
                        }

                        inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                            SizeX64::dword,
                            index,
                            &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                        );
                        let op0 = *get_op_mut(inst, 0);

                        if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
                            (*self.build)
                                .mov(OperandX64::reg(inst.reg_x64), self.mem_reg_uint_op(op0));
                        }

                        if (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Constant {
                            // if shift value is a constant, we extract the byte-sized shift amount
                            let mut shift = (((self.int_op((*get_op_mut(inst, 1)))) as u32) as i8);
                            (*self.build)
                                .shl(OperandX64::reg(inst.reg_x64), OperandX64::imm(shift as i32));
                        } else {
                            (*self.build).mov(
                                OperandX64::reg(shiftTmp.reg),
                                self.mem_reg_uint_op((*get_op_mut(inst, 1))),
                            );
                            (*self.build).shl(
                                OperandX64::reg(inst.reg_x64),
                                OperandX64::reg(byte_reg(shiftTmp.reg)),
                            );
                        }
                    }
                }
                IrCmd::BITRSHIFT_UINT => {
                    {
                        let mut shiftTmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };

                        // Custom bit shift value can only be placed in RegisterX64::cl
                        // but we use it if the shift value is not a constant stored in b
                        if (*get_op_mut(inst, (((1) as u32) as u32))).kind() != IrOpKind::Constant {
                            shiftTmp.take(dword_reg(RegisterX64::rcx));
                        }

                        inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                            SizeX64::dword,
                            index,
                            &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                        );
                        let op0 = *get_op_mut(inst, 0);

                        if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
                            (*self.build)
                                .mov(OperandX64::reg(inst.reg_x64), self.mem_reg_uint_op(op0));
                        }

                        if (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Constant {
                            // if shift value is a constant, we extract the byte-sized shift amount
                            let mut shift = (((self.int_op((*get_op_mut(inst, 1)))) as u32) as i8);
                            (*self.build)
                                .shr(OperandX64::reg(inst.reg_x64), OperandX64::imm(shift as i32));
                        } else {
                            (*self.build).mov(
                                OperandX64::reg(shiftTmp.reg),
                                self.mem_reg_uint_op((*get_op_mut(inst, 1))),
                            );
                            (*self.build).shr(
                                OperandX64::reg(inst.reg_x64),
                                OperandX64::reg(byte_reg(shiftTmp.reg)),
                            );
                        }
                    }
                }
                IrCmd::BITARSHIFT_UINT => {
                    {
                        let mut shiftTmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };

                        // Custom bit shift value can only be placed in RegisterX64::cl
                        // but we use it if the shift value is not a constant stored in b
                        if (*get_op_mut(inst, (((1) as u32) as u32))).kind() != IrOpKind::Constant {
                            shiftTmp.take(dword_reg(RegisterX64::rcx));
                        }

                        inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                            SizeX64::dword,
                            index,
                            &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                        );
                        let op0 = *get_op_mut(inst, 0);

                        if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
                            (*self.build)
                                .mov(OperandX64::reg(inst.reg_x64), self.mem_reg_uint_op(op0));
                        }

                        if (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Constant {
                            // if shift value is a constant, we extract the byte-sized shift amount
                            let mut shift = (((self.int_op((*get_op_mut(inst, 1)))) as u32) as i8);
                            (*self.build)
                                .sar(OperandX64::reg(inst.reg_x64), OperandX64::imm(shift as i32));
                        } else {
                            (*self.build).mov(
                                OperandX64::reg(shiftTmp.reg),
                                self.mem_reg_uint_op((*get_op_mut(inst, 1))),
                            );
                            (*self.build).sar(
                                OperandX64::reg(inst.reg_x64),
                                OperandX64::reg(byte_reg(shiftTmp.reg)),
                            );
                        }
                    }
                }
                IrCmd::BITLROTATE_UINT => {
                    {
                        let mut shiftTmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };

                        // Custom bit shift value can only be placed in RegisterX64::cl
                        // but we use it if the shift value is not a constant stored in b
                        if (*get_op_mut(inst, (((1) as u32) as u32))).kind() != IrOpKind::Constant {
                            shiftTmp.take(dword_reg(RegisterX64::rcx));
                        }

                        inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                            SizeX64::dword,
                            index,
                            &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                        );
                        let op0 = *get_op_mut(inst, 0);

                        if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
                            (*self.build)
                                .mov(OperandX64::reg(inst.reg_x64), self.mem_reg_uint_op(op0));
                        }

                        if (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Constant {
                            // if shift value is a constant, we extract the byte-sized shift amount
                            let mut shift = (((self.int_op((*get_op_mut(inst, 1)))) as u32) as i8);
                            (*self.build)
                                .rol(OperandX64::reg(inst.reg_x64), OperandX64::imm(shift as i32));
                        } else {
                            (*self.build).mov(
                                OperandX64::reg(shiftTmp.reg),
                                self.mem_reg_uint_op((*get_op_mut(inst, 1))),
                            );
                            (*self.build).rol(
                                OperandX64::reg(inst.reg_x64),
                                OperandX64::reg(byte_reg(shiftTmp.reg)),
                            );
                        }
                    }
                }
                IrCmd::BITRROTATE_UINT => {
                    {
                        let mut shiftTmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };

                        // Custom bit shift value can only be placed in RegisterX64::cl
                        // but we use it if the shift value is not a constant stored in b
                        if (*get_op_mut(inst, (((1) as u32) as u32))).kind() != IrOpKind::Constant {
                            shiftTmp.take(dword_reg(RegisterX64::rcx));
                        }

                        inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                            SizeX64::dword,
                            index,
                            &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                        );
                        let op0 = *get_op_mut(inst, 0);

                        if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
                            (*self.build)
                                .mov(OperandX64::reg(inst.reg_x64), self.mem_reg_uint_op(op0));
                        }

                        if (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Constant {
                            // if shift value is a constant, we extract the byte-sized shift amount
                            let mut shift = (((self.int_op((*get_op_mut(inst, 1)))) as u32) as i8);
                            (*self.build)
                                .ror(OperandX64::reg(inst.reg_x64), OperandX64::imm(shift as i32));
                        } else {
                            (*self.build).mov(
                                OperandX64::reg(shiftTmp.reg),
                                self.mem_reg_uint_op((*get_op_mut(inst, 1))),
                            );
                            (*self.build).ror(
                                OperandX64::reg(inst.reg_x64),
                                OperandX64::reg(byte_reg(shiftTmp.reg)),
                            );
                        }
                    }
                }
                IrCmd::BITCOUNTLZ_UINT => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::dword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );

                    let mut zero = Label::default();
                    let mut exit = Label::default();

                    (*self.build).test(
                        OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                        OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                    );
                    (*self.build).jcc(ConditionX64::Equal, &mut zero);

                    (*self.build).bsr(
                        inst.reg_x64,
                        OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                    );
                    (*self.build).xor_(OperandX64::reg(inst.reg_x64), OperandX64::imm(0x1f));
                    (*self.build).jmp_label(&mut exit);

                    (*self.build).set_label(&mut zero);
                    (*self.build).mov(OperandX64::reg(inst.reg_x64), OperandX64::imm((32) as i32));

                    (*self.build).set_label(&mut exit);
                }
                IrCmd::BITCOUNTRZ_UINT => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::dword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );

                    let mut zero = Label::default();
                    let mut exit = Label::default();

                    (*self.build).test(
                        OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                        OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                    );
                    (*self.build).jcc(ConditionX64::Equal, &mut zero);

                    (*self.build).bsf(
                        inst.reg_x64,
                        OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                    );
                    (*self.build).jmp_label(&mut exit);

                    (*self.build).set_label(&mut zero);
                    (*self.build).mov(OperandX64::reg(inst.reg_x64), OperandX64::imm((32) as i32));

                    (*self.build).set_label(&mut exit);
                }
                IrCmd::BYTESWAP_UINT => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::dword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );
                    let op0 = *get_op_mut(inst, 0);

                    if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
                        (*self.build).mov(OperandX64::reg(inst.reg_x64), self.mem_reg_uint_op(op0));
                    }

                    (*self.build).bswap(inst.reg_x64);
                }
                IrCmd::INVOKE_LIBM => {
                    let mut callWrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
                        &mut self.regs,
                        &mut *self.build,
                        index,
                    );
                    callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                        SizeX64::xmmword,
                        self.mem_reg_double_op((*get_op_mut(inst, 1))),
                        (*get_op_mut(inst, (((1) as u32) as u32))),
                    );

                    if HAS_OP_C!(inst) {
                        let mut isInt = if ((*get_op_mut(inst, (((2) as u32) as u32))).kind()
                            == IrOpKind::Constant)
                        {
                            self.ir_lowering_x_64_const_op((*get_op_mut(inst, 2))).kind
                                == IrConstKind::Int
                        } else {
                            get_cmd_value_kind((*self.function).inst_op((*get_op_mut(inst, 2))).cmd)
                                == IrValueKind::Int
                        };

                        if isInt {
                            callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                                SizeX64::dword,
                                self.mem_reg_uint_op((*get_op_mut(inst, 2))),
                                (*get_op_mut(inst, (((2) as u32) as u32))),
                            );
                        } else {
                            callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                                SizeX64::xmmword,
                                self.mem_reg_double_op((*get_op_mut(inst, 2))),
                                (*get_op_mut(inst, (((2) as u32) as u32))),
                            );
                        }
                    }

                    callWrap.call(&OperandX64::mem(
                        SizeX64::qword,
                        RegisterX64::noreg,
                        1,
                        r_native_context(),
                        get_native_context_offset(self.uint_op((*get_op_mut(inst, 0))) as i32)
                            as i32,
                    ));
                    inst.reg_x64 = self.regs.take_reg(xmm0(), index);
                }
                IrCmd::GET_TYPE => {
                    inst.reg_x64 = self.regs.alloc_reg(SizeX64::qword, index);

                    (*self.build).mov(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::mem(
                            SizeX64::qword,
                            RegisterX64::noreg,
                            1,
                            r_state(),
                            (core::mem::offset_of!(lua_State, global) as i32),
                        ),
                    );

                    if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Inst {
                        (*self.build).mov(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::mem(
                                SizeX64::qword,
                                qword_reg(self.reg_op((*get_op_mut(inst, 0)))),
                                core::mem::size_of::<*mut TString>() as u8,
                                inst.reg_x64,
                                (core::mem::offset_of!(global_State, ttname) as i32),
                            ),
                        );
                    } else if (*get_op_mut(inst, (((0) as u32) as u32))).kind()
                        == IrOpKind::Constant
                    {
                        (*self.build).mov(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::mem(
                                SizeX64::qword,
                                RegisterX64::noreg,
                                1,
                                inst.reg_x64,
                                (core::mem::offset_of!(global_State, ttname) as i32)
                                    + (self.tag_op((*get_op_mut(inst, 0))) as i32)
                                        * (core::mem::size_of::<*mut TString>() as i32),
                            ),
                        );
                    } else {
                        CODEGEN_ASSERT!(false, "Unsupported instruction form");
                    }
                }
                IrCmd::GET_TYPEOF => {
                    let mut callWrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
                        &mut self.regs,
                        &mut *self.build,
                        index,
                    );
                    callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                        SizeX64::qword,
                        OperandX64::reg(r_state()),
                        IrOp::default(),
                    );
                    callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                        SizeX64::qword,
                        luau_reg_address(vm_reg_op((*get_op_mut(inst, 0)))),
                        IrOp::default(),
                    );
                    callWrap.call(&OperandX64::mem(
                        SizeX64::qword,
                        RegisterX64::noreg,
                        1,
                        r_native_context(),
                        (core::mem::offset_of!(NativeContext, luaT_objtypenamestr) as i32),
                    ));

                    inst.reg_x64 = self.regs.take_reg(RegisterX64::rax, index);
                }
                IrCmd::FINDUPVAL => {
                    let mut callWrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
                        &mut self.regs,
                        &mut *self.build,
                        index,
                    );
                    callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                        SizeX64::qword,
                        OperandX64::reg(r_state()),
                        IrOp::default(),
                    );
                    callWrap.add_argument_size_x_64_operand_x_64_ir_op(
                        SizeX64::qword,
                        luau_reg_address(vm_reg_op((*get_op_mut(inst, 0)))),
                        IrOp::default(),
                    );
                    callWrap.call(&OperandX64::mem(
                        SizeX64::qword,
                        RegisterX64::noreg,
                        1,
                        r_native_context(),
                        (core::mem::offset_of!(NativeContext, luaF_findupval) as i32),
                    ));

                    inst.reg_x64 = self.regs.take_reg(RegisterX64::rax, index);
                }
                IrCmd::BUFFER_READI8 => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::dword,
                        index,
                        &[
                            (*get_op_mut(inst, (((0) as u32) as u32))),
                            (*get_op_mut(inst, (((1) as u32) as u32))),
                        ],
                    );

                    (*self.build).movsx(
                        inst.reg_x64,
                        sized_mem(
                            self.buffer_addr_op(
                                (*get_op_mut(inst, 0)),
                                (*get_op_mut(inst, 1)),
                                self.tag_op((*get_op_mut(inst, 2))),
                            ),
                            SizeX64::byte,
                        ),
                    );
                }
                IrCmd::BUFFER_READU8 => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::dword,
                        index,
                        &[
                            (*get_op_mut(inst, (((0) as u32) as u32))),
                            (*get_op_mut(inst, (((1) as u32) as u32))),
                        ],
                    );

                    (*self.build).movzx(
                        inst.reg_x64,
                        sized_mem(
                            self.buffer_addr_op(
                                (*get_op_mut(inst, 0)),
                                (*get_op_mut(inst, 1)),
                                self.tag_op((*get_op_mut(inst, 2))),
                            ),
                            SizeX64::byte,
                        ),
                    );
                }
                IrCmd::BUFFER_WRITEI8 => {
                    let mut value =
                        if (*get_op_mut(inst, (((2) as u32) as u32))).kind() == IrOpKind::Inst {
                            OperandX64::reg(byte_reg(self.reg_op((*get_op_mut(inst, 2)))))
                        } else {
                            OperandX64::imm(self.int_op((*get_op_mut(inst, 2))) as i8 as i32)
                        };

                    (*self.build).mov(
                        sized_mem(
                            self.buffer_addr_op(
                                (*get_op_mut(inst, 0)),
                                (*get_op_mut(inst, 1)),
                                self.tag_op((*get_op_mut(inst, 3))),
                            ),
                            SizeX64::byte,
                        ),
                        value,
                    );
                }
                IrCmd::BUFFER_READI16 => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::dword,
                        index,
                        &[
                            (*get_op_mut(inst, (((0) as u32) as u32))),
                            (*get_op_mut(inst, (((1) as u32) as u32))),
                        ],
                    );

                    (*self.build).movsx(
                        inst.reg_x64,
                        sized_mem(
                            self.buffer_addr_op(
                                (*get_op_mut(inst, 0)),
                                (*get_op_mut(inst, 1)),
                                self.tag_op((*get_op_mut(inst, 2))),
                            ),
                            SizeX64::word,
                        ),
                    );
                }
                IrCmd::BUFFER_READU16 => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::dword,
                        index,
                        &[
                            (*get_op_mut(inst, (((0) as u32) as u32))),
                            (*get_op_mut(inst, (((1) as u32) as u32))),
                        ],
                    );

                    (*self.build).movzx(
                        inst.reg_x64,
                        sized_mem(
                            self.buffer_addr_op(
                                (*get_op_mut(inst, 0)),
                                (*get_op_mut(inst, 1)),
                                self.tag_op((*get_op_mut(inst, 2))),
                            ),
                            SizeX64::word,
                        ),
                    );
                }
                IrCmd::BUFFER_WRITEI16 => {
                    let mut value =
                        if (*get_op_mut(inst, (((2) as u32) as u32))).kind() == IrOpKind::Inst {
                            OperandX64::reg(word_reg(self.reg_op((*get_op_mut(inst, 2)))))
                        } else {
                            OperandX64::imm(self.int_op((*get_op_mut(inst, 2))) as i16 as i32)
                        };

                    (*self.build).mov(
                        sized_mem(
                            self.buffer_addr_op(
                                (*get_op_mut(inst, 0)),
                                (*get_op_mut(inst, 1)),
                                self.tag_op((*get_op_mut(inst, 3))),
                            ),
                            SizeX64::word,
                        ),
                        value,
                    );
                }
                IrCmd::BUFFER_READI32 => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::dword,
                        index,
                        &[
                            (*get_op_mut(inst, (((0) as u32) as u32))),
                            (*get_op_mut(inst, (((1) as u32) as u32))),
                        ],
                    );

                    (*self.build).mov(
                        OperandX64::reg(inst.reg_x64),
                        sized_mem(
                            self.buffer_addr_op(
                                (*get_op_mut(inst, 0)),
                                (*get_op_mut(inst, 1)),
                                self.tag_op((*get_op_mut(inst, 2))),
                            ),
                            SizeX64::dword,
                        ),
                    );
                }
                IrCmd::BUFFER_WRITEI32 => {
                    let mut value =
                        if (*get_op_mut(inst, (((2) as u32) as u32))).kind() == IrOpKind::Inst {
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 2))))
                        } else {
                            OperandX64::imm(self.int_op((*get_op_mut(inst, 2))))
                        };

                    (*self.build).mov(
                        sized_mem(
                            self.buffer_addr_op(
                                (*get_op_mut(inst, 0)),
                                (*get_op_mut(inst, 1)),
                                self.tag_op((*get_op_mut(inst, 3))),
                            ),
                            SizeX64::dword,
                        ),
                        value,
                    );
                }
                IrCmd::BUFFER_READF32 => {
                    inst.reg_x64 = self.regs.alloc_reg(SizeX64::xmmword, index);

                    (*self.build).vmovss_operand_x_64_operand_x_64(
                        OperandX64::reg(inst.reg_x64),
                        sized_mem(
                            self.buffer_addr_op(
                                (*get_op_mut(inst, 0)),
                                (*get_op_mut(inst, 1)),
                                self.tag_op((*get_op_mut(inst, 2))),
                            ),
                            SizeX64::dword,
                        ),
                    );
                }
                IrCmd::BUFFER_WRITEF32 => {
                    let dst = sized_mem(
                        self.buffer_addr_op(
                            (*get_op_mut(inst, 0)),
                            (*get_op_mut(inst, 1)),
                            self.tag_op((*get_op_mut(inst, 3))),
                        ),
                        SizeX64::dword,
                    );
                    let src = *get_op_mut(inst, 2);
                    self.store_float(dst, src);
                }
                IrCmd::BUFFER_READF64 => {
                    inst.reg_x64 = self.regs.alloc_reg(SizeX64::xmmword, index);

                    (*self.build).vmovsd_operand_x_64_operand_x_64(
                        OperandX64::reg(inst.reg_x64),
                        sized_mem(
                            self.buffer_addr_op(
                                (*get_op_mut(inst, 0)),
                                (*get_op_mut(inst, 1)),
                                self.tag_op((*get_op_mut(inst, 2))),
                            ),
                            SizeX64::qword,
                        ),
                    );
                }
                IrCmd::BUFFER_WRITEF64 => {
                    if (*get_op_mut(inst, (((2) as u32) as u32))).kind() == IrOpKind::Constant {
                        let mut tmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp.alloc(SizeX64::xmmword);
                        (*self.build).vmovsd_operand_x_64_operand_x_64(
                            OperandX64::reg(tmp.reg),
                            (*self.build).f64(self.double_op((*get_op_mut(inst, 2)))),
                        );

                        (*self.build).vmovsd_operand_x_64_operand_x_64(
                            sized_mem(
                                self.buffer_addr_op(
                                    (*get_op_mut(inst, 0)),
                                    (*get_op_mut(inst, 1)),
                                    self.tag_op((*get_op_mut(inst, 3))),
                                ),
                                SizeX64::qword,
                            ),
                            OperandX64::reg(tmp.reg),
                        );
                    } else if (*get_op_mut(inst, (((2) as u32) as u32))).kind() == IrOpKind::Inst {
                        (*self.build).vmovsd_operand_x_64_operand_x_64(
                            sized_mem(
                                self.buffer_addr_op(
                                    (*get_op_mut(inst, 0)),
                                    (*get_op_mut(inst, 1)),
                                    self.tag_op((*get_op_mut(inst, 3))),
                                ),
                                SizeX64::qword,
                            ),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 2)))),
                        );
                    } else {
                        CODEGEN_ASSERT!(false, "Unsupported instruction form");
                    }
                }
                IrCmd::BUFFER_READI64 => {
                    inst.reg_x64 = self.regs.alloc_reg(SizeX64::qword, index);

                    (*self.build).mov(
                        OperandX64::reg(inst.reg_x64),
                        sized_mem(
                            self.buffer_addr_op(
                                (*get_op_mut(inst, 0)),
                                (*get_op_mut(inst, 1)),
                                self.tag_op((*get_op_mut(inst, 2))),
                            ),
                            SizeX64::qword,
                        ),
                    );
                }
                IrCmd::BUFFER_WRITEI64 => {
                    if (*get_op_mut(inst, (((2) as u32) as u32))).kind() == IrOpKind::Constant {
                        let mut tmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp.alloc(SizeX64::qword);
                        (*self.build).mov(
                            OperandX64::reg(tmp.reg),
                            (*self.build).i64(self.int64_op((*get_op_mut(inst, 2)))),
                        );

                        (*self.build).mov(
                            sized_mem(
                                self.buffer_addr_op(
                                    (*get_op_mut(inst, 0)),
                                    (*get_op_mut(inst, 1)),
                                    self.tag_op((*get_op_mut(inst, 3))),
                                ),
                                SizeX64::qword,
                            ),
                            OperandX64::reg(tmp.reg),
                        );
                    } else if (*get_op_mut(inst, (((2) as u32) as u32))).kind() == IrOpKind::Inst {
                        (*self.build).mov(
                            sized_mem(
                                self.buffer_addr_op(
                                    (*get_op_mut(inst, 0)),
                                    (*get_op_mut(inst, 1)),
                                    self.tag_op((*get_op_mut(inst, 3))),
                                ),
                                SizeX64::qword,
                            ),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 2)))),
                        );
                    } else {
                        CODEGEN_ASSERT!(false, "Unsupported instruction form");
                    }
                }
                IrCmd::CHECK_DIV_INT64 => {
                    {
                        let mut tmpA = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmpA.alloc(SizeX64::qword);
                        let mut tmpB = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmpB.alloc(SizeX64::qword);
                        (*self.build).mov(
                            OperandX64::reg(tmpA.reg),
                            self.mem_reg_int_64_op((*get_op_mut(inst, 0))),
                        );
                        (*self.build).mov(
                            OperandX64::reg(tmpB.reg),
                            self.mem_reg_int_64_op((*get_op_mut(inst, 1))),
                        );

                        // guard against division by zero
                        (*self.build).test(OperandX64::reg(tmpB.reg), OperandX64::reg(tmpB.reg));
                        self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                            ConditionX64::Equal,
                            (*get_op_mut(inst, 2)),
                            index,
                            next,
                        );

                        // guard against dividend == i64::MIN && divisor == -1 (signed overflow)
                        {
                            let mut skip = Label::default();

                            (*self.build)
                                .cmp(OperandX64::reg(tmpB.reg), OperandX64::imm((-1) as i32));
                            (*self.build).jcc(ConditionX64::NotEqual, &mut skip);

                            let mut tmpMin = ScopedRegX64 {
                                owner: &mut self.regs,
                                reg: RegisterX64::noreg,
                            };
                            tmpMin.alloc(SizeX64::qword);
                            (*self.build).mov64(tmpMin.reg, i64::MIN);
                            (*self.build)
                                .cmp(OperandX64::reg(tmpA.reg), OperandX64::reg(tmpMin.reg));
                            self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                                ConditionX64::Equal,
                                (*get_op_mut(inst, 2)),
                                index,
                                next,
                            );

                            (*self.build).set_label(&mut skip);
                        }
                    }
                }
                IrCmd::CHECK_CMP_INT64 => {
                    let mut cond = condition_op((*get_op_mut(inst, 2)));

                    if (cond == IrCondition::Equal || cond == IrCondition::NotEqual)
                        && (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Constant
                        && self.int64_op((*get_op_mut(inst, 1))) == 0
                    {
                        (*self.build).test(
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                        );
                        self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                            if cond == IrCondition::Equal {
                                ConditionX64::NotZero
                            } else {
                                ConditionX64::Zero
                            },
                            (*get_op_mut(inst, 3)),
                            index,
                            next,
                        );
                    } else if (*get_op_mut(inst, (((0) as u32) as u32))).kind()
                        == IrOpKind::Constant
                    {
                        let mut tmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };
                        tmp.alloc(SizeX64::qword);
                        (*self.build).mov(
                            OperandX64::reg(tmp.reg),
                            self.mem_reg_int_64_op((*get_op_mut(inst, 0))),
                        );
                        (*self.build).cmp(
                            OperandX64::reg(tmp.reg),
                            self.mem_reg_int_64_op((*get_op_mut(inst, 1))),
                        );
                        self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                            get_condition_int(get_negated_condition_ir_condition(cond)),
                            (*get_op_mut(inst, 3)),
                            index,
                            next,
                        );
                    } else {
                        (*self.build).cmp(
                            OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                            self.mem_reg_int_64_op((*get_op_mut(inst, 1))),
                        );
                        self.jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
                            get_condition_int(get_negated_condition_ir_condition(cond)),
                            (*get_op_mut(inst, 3)),
                            index,
                            next,
                        );
                    }
                }
                IrCmd::CMP_INT64 => {
                    {
                        // cannot reuse operand registers as a target because we have to modify it before the comparison
                        inst.reg_x64 = self.regs.alloc_reg(SizeX64::dword, index);

                        // We are going to operate on byte register, those do not clear high bits on write
                        (*self.build)
                            .xor_(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));

                        let mut cond = condition_op((*get_op_mut(inst, 2)));

                        if (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Constant {
                            (*self.build).cmp(
                                OperandX64::reg(self.reg_op((*get_op_mut(inst, 1)))),
                                self.mem_reg_int_64_op((*get_op_mut(inst, 0))),
                            );
                            (*self.build).setcc(
                                get_inverse_condition(get_condition_int(cond)),
                                OperandX64::reg(byte_reg(inst.reg_x64)),
                            );
                        } else if (*get_op_mut(inst, (((0) as u32) as u32))).kind()
                            == IrOpKind::Inst
                        {
                            (*self.build).cmp(
                                OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                                self.mem_reg_int_64_op((*get_op_mut(inst, 1))),
                            );
                            (*self.build).setcc(
                                get_condition_int(cond),
                                OperandX64::reg(byte_reg(inst.reg_x64)),
                            );
                        } else {
                            CODEGEN_ASSERT!(false, "Unsupported instruction form");
                        }
                    }
                }
                IrCmd::INT64_TO_NUM => {
                    inst.reg_x64 = self.regs.alloc_reg(SizeX64::xmmword, index);

                    (*self.build).vcvtsi2sd(
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(inst.reg_x64),
                        OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                    );
                }
                IrCmd::NUM_TO_INT64 => {
                    inst.reg_x64 = self.regs.alloc_reg(SizeX64::qword, index);

                    (*self.build).vcvttsd2si(
                        OperandX64::reg(inst.reg_x64),
                        self.mem_reg_double_op((*get_op_mut(inst, 0))),
                    );
                }
                IrCmd::BITAND_INT64 => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::qword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );
                    let op0 = *get_op_mut(inst, 0);

                    if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
                        (*self.build)
                            .mov(OperandX64::reg(inst.reg_x64), self.mem_reg_int_64_op(op0));
                    }

                    (*self.build).and_(
                        OperandX64::reg(inst.reg_x64),
                        self.mem_reg_int_64_op((*get_op_mut(inst, 1))),
                    );
                }
                IrCmd::BITXOR_INT64 => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::qword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );
                    let op0 = *get_op_mut(inst, 0);

                    if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
                        (*self.build)
                            .mov(OperandX64::reg(inst.reg_x64), self.mem_reg_int_64_op(op0));
                    }

                    (*self.build).xor_(
                        OperandX64::reg(inst.reg_x64),
                        self.mem_reg_int_64_op((*get_op_mut(inst, 1))),
                    );
                }
                IrCmd::BITOR_INT64 => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::qword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );
                    let op0 = *get_op_mut(inst, 0);

                    if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
                        (*self.build)
                            .mov(OperandX64::reg(inst.reg_x64), self.mem_reg_int_64_op(op0));
                    }

                    (*self.build).or_(
                        OperandX64::reg(inst.reg_x64),
                        self.mem_reg_int_64_op((*get_op_mut(inst, 1))),
                    );
                }
                IrCmd::BITNOT_INT64 => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::qword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );
                    let op0 = *get_op_mut(inst, 0);

                    if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
                        (*self.build)
                            .mov(OperandX64::reg(inst.reg_x64), self.mem_reg_int_64_op(op0));
                    }

                    (*self.build).not_(OperandX64::reg(inst.reg_x64));
                }
                IrCmd::BITLSHIFT_INT64 => {
                    {
                        let mut shiftTmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };

                        if (*get_op_mut(inst, (((1) as u32) as u32))).kind() != IrOpKind::Constant {
                            shiftTmp.take(RegisterX64::rcx);
                        }

                        inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                            SizeX64::qword,
                            index,
                            &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                        );
                        let op0 = *get_op_mut(inst, 0);

                        if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
                            (*self.build)
                                .mov(OperandX64::reg(inst.reg_x64), self.mem_reg_int_64_op(op0));
                        }

                        if (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Constant {
                            let mut shift = self.int64_op((*get_op_mut(inst, 1)));

                            if shift < 0 {
                                // Negative left shift = right shift by -amount
                                let mut amount = ((-shift) as u8);
                                if amount > 63 {
                                    (*self.build).xor_(
                                        OperandX64::reg(inst.reg_x64),
                                        OperandX64::reg(inst.reg_x64),
                                    );
                                } else {
                                    (*self.build).shr(
                                        OperandX64::reg(inst.reg_x64),
                                        OperandX64::imm(((amount) as i8) as i32),
                                    );
                                }
                            } else if shift > 63 {
                                (*self.build).xor_(
                                    OperandX64::reg(inst.reg_x64),
                                    OperandX64::reg(inst.reg_x64),
                                );
                            } else {
                                (*self.build).shl(
                                    OperandX64::reg(inst.reg_x64),
                                    OperandX64::imm(((shift) as i8) as i32),
                                );
                            }
                        } else {
                            let mut tmp = ScopedRegX64 {
                                owner: &mut self.regs,
                                reg: RegisterX64::noreg,
                            };
                            tmp.alloc(SizeX64::qword);

                            let mut negative = Label::default();
                            let mut outOfRange = Label::default();
                            let mut done = Label::default();

                            (*self.build).mov(
                                OperandX64::reg(shiftTmp.reg),
                                self.mem_reg_int_64_op((*get_op_mut(inst, 1))),
                            );

                            // Check |amount| > 63: (amount + 63) unsigned > 126
                            (*self.build).lea_operand_x_64_operand_x_64(
                                OperandX64::reg(tmp.reg),
                                OperandX64::mem(
                                    SizeX64::none,
                                    RegisterX64::noreg,
                                    1,
                                    shiftTmp.reg,
                                    63,
                                ),
                            );
                            (*self.build)
                                .cmp(OperandX64::reg(tmp.reg), OperandX64::imm((126) as i32));
                            (*self.build).jcc(ConditionX64::Above, &mut outOfRange);

                            // Check sign of amount
                            (*self.build)
                                .test(OperandX64::reg(shiftTmp.reg), OperandX64::reg(shiftTmp.reg));
                            (*self.build).jcc(ConditionX64::Less, &mut negative);

                            // Left shift
                            (*self.build).shl(
                                OperandX64::reg(inst.reg_x64),
                                OperandX64::reg(byte_reg(shiftTmp.reg)),
                            );
                            (*self.build).jmp_label(&mut done);

                            // Right shift by -amount
                            (*self.build).set_label(&mut negative);
                            (*self.build).neg(OperandX64::reg(shiftTmp.reg));
                            (*self.build).shr(
                                OperandX64::reg(inst.reg_x64),
                                OperandX64::reg(byte_reg(shiftTmp.reg)),
                            );
                            (*self.build).jmp_label(&mut done);

                            (*self.build).set_label(&mut outOfRange);
                            (*self.build)
                                .xor_(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));

                            (*self.build).set_label(&mut done);
                        }
                    }
                }
                IrCmd::BITRSHIFT_INT64 => {
                    {
                        let mut shiftTmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };

                        if (*get_op_mut(inst, (((1) as u32) as u32))).kind() != IrOpKind::Constant {
                            shiftTmp.take(RegisterX64::rcx);
                        }

                        inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                            SizeX64::qword,
                            index,
                            &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                        );
                        let op0 = *get_op_mut(inst, 0);

                        if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
                            (*self.build)
                                .mov(OperandX64::reg(inst.reg_x64), self.mem_reg_int_64_op(op0));
                        }

                        if (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Constant {
                            let mut shift = self.int64_op((*get_op_mut(inst, 1)));

                            if shift < 0 {
                                // Negative right shift = left shift by -amount
                                let mut amount = ((-shift) as u8);
                                if amount > 63 {
                                    (*self.build).xor_(
                                        OperandX64::reg(inst.reg_x64),
                                        OperandX64::reg(inst.reg_x64),
                                    );
                                } else {
                                    (*self.build).shl(
                                        OperandX64::reg(inst.reg_x64),
                                        OperandX64::imm(((amount) as i8) as i32),
                                    );
                                }
                            } else if shift > 63 {
                                (*self.build).xor_(
                                    OperandX64::reg(inst.reg_x64),
                                    OperandX64::reg(inst.reg_x64),
                                );
                            } else {
                                (*self.build).shr(
                                    OperandX64::reg(inst.reg_x64),
                                    OperandX64::imm(((shift) as i8) as i32),
                                );
                            }
                        } else {
                            let mut tmp = ScopedRegX64 {
                                owner: &mut self.regs,
                                reg: RegisterX64::noreg,
                            };
                            tmp.alloc(SizeX64::qword);

                            let mut negative = Label::default();
                            let mut outOfRange = Label::default();
                            let mut done = Label::default();

                            (*self.build).mov(
                                OperandX64::reg(shiftTmp.reg),
                                self.mem_reg_int_64_op((*get_op_mut(inst, 1))),
                            );

                            // Check |amount| > 63: (amount + 63) unsigned > 126
                            (*self.build).lea_operand_x_64_operand_x_64(
                                OperandX64::reg(tmp.reg),
                                OperandX64::mem(
                                    SizeX64::none,
                                    RegisterX64::noreg,
                                    1,
                                    shiftTmp.reg,
                                    63,
                                ),
                            );
                            (*self.build)
                                .cmp(OperandX64::reg(tmp.reg), OperandX64::imm((126) as i32));
                            (*self.build).jcc(ConditionX64::Above, &mut outOfRange);

                            // Check sign of amount
                            (*self.build)
                                .test(OperandX64::reg(shiftTmp.reg), OperandX64::reg(shiftTmp.reg));
                            (*self.build).jcc(ConditionX64::Less, &mut negative);

                            // Unsigned right shift
                            (*self.build).shr(
                                OperandX64::reg(inst.reg_x64),
                                OperandX64::reg(byte_reg(shiftTmp.reg)),
                            );
                            (*self.build).jmp_label(&mut done);

                            // Left shift by -amount
                            (*self.build).set_label(&mut negative);
                            (*self.build).neg(OperandX64::reg(shiftTmp.reg));
                            (*self.build).shl(
                                OperandX64::reg(inst.reg_x64),
                                OperandX64::reg(byte_reg(shiftTmp.reg)),
                            );
                            (*self.build).jmp_label(&mut done);

                            (*self.build).set_label(&mut outOfRange);
                            (*self.build)
                                .xor_(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));

                            (*self.build).set_label(&mut done);
                        }
                    }
                }
                IrCmd::BITARSHIFT_INT64 => {
                    {
                        let mut shiftTmp = ScopedRegX64 {
                            owner: &mut self.regs,
                            reg: RegisterX64::noreg,
                        };

                        if (*get_op_mut(inst, (((1) as u32) as u32))).kind() != IrOpKind::Constant {
                            shiftTmp.take(RegisterX64::rcx);
                        }

                        inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                            SizeX64::qword,
                            index,
                            &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                        );
                        let op0 = *get_op_mut(inst, 0);

                        if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
                            (*self.build)
                                .mov(OperandX64::reg(inst.reg_x64), self.mem_reg_int_64_op(op0));
                        }

                        if (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Constant {
                            let mut shift = self.int64_op((*get_op_mut(inst, 1)));

                            if shift < -63 {
                                // Left shift by > 63 = 0
                                (*self.build).xor_(
                                    OperandX64::reg(inst.reg_x64),
                                    OperandX64::reg(inst.reg_x64),
                                );
                            } else if shift < 0 {
                                // Negative arshift = left shift by -amount
                                (*self.build).shl(
                                    OperandX64::reg(inst.reg_x64),
                                    OperandX64::imm(((-shift) as i8) as i32),
                                );
                            } else if shift > 63 {
                                // Arithmetic right shift by > 63 = sign-fill
                                (*self.build).sar(
                                    OperandX64::reg(inst.reg_x64),
                                    OperandX64::imm(((63) as i8) as i32),
                                );
                            } else {
                                (*self.build).sar(
                                    OperandX64::reg(inst.reg_x64),
                                    OperandX64::imm(((shift) as i8) as i32),
                                );
                            }
                        } else {
                            let mut tmp = ScopedRegX64 {
                                owner: &mut self.regs,
                                reg: RegisterX64::noreg,
                            };
                            tmp.alloc(SizeX64::qword);

                            let mut negative = Label::default();
                            let mut outOfRangePositive = Label::default();
                            let mut outOfRangeNegative = Label::default();
                            let mut done = Label::default();

                            (*self.build).mov(
                                OperandX64::reg(shiftTmp.reg),
                                self.mem_reg_int_64_op((*get_op_mut(inst, 1))),
                            );

                            // amount > 63: sign-fill
                            (*self.build)
                                .cmp(OperandX64::reg(shiftTmp.reg), OperandX64::imm((63) as i32));
                            (*self.build).jcc(ConditionX64::Greater, &mut outOfRangePositive);

                            // Check amount < -63: (amount + 63) < 0
                            (*self.build).lea_operand_x_64_operand_x_64(
                                OperandX64::reg(tmp.reg),
                                OperandX64::mem(
                                    SizeX64::none,
                                    RegisterX64::noreg,
                                    1,
                                    shiftTmp.reg,
                                    63,
                                ),
                            );
                            (*self.build).test(OperandX64::reg(tmp.reg), OperandX64::reg(tmp.reg));
                            (*self.build).jcc(ConditionX64::Less, &mut outOfRangeNegative);

                            // Check sign of amount
                            (*self.build)
                                .test(OperandX64::reg(shiftTmp.reg), OperandX64::reg(shiftTmp.reg));
                            (*self.build).jcc(ConditionX64::Less, &mut negative);

                            // Arithmetic right shift
                            (*self.build).sar(
                                OperandX64::reg(inst.reg_x64),
                                OperandX64::reg(byte_reg(shiftTmp.reg)),
                            );
                            (*self.build).jmp_label(&mut done);

                            // Left shift by -amount
                            (*self.build).set_label(&mut negative);
                            (*self.build).neg(OperandX64::reg(shiftTmp.reg));
                            (*self.build).shl(
                                OperandX64::reg(inst.reg_x64),
                                OperandX64::reg(byte_reg(shiftTmp.reg)),
                            );
                            (*self.build).jmp_label(&mut done);

                            // amount > 63: sign-fill ( if n < 0 { -1 } else { 0 })
                            (*self.build).set_label(&mut outOfRangePositive);
                            (*self.build).sar(
                                OperandX64::reg(inst.reg_x64),
                                OperandX64::imm(((63) as i8) as i32),
                            );
                            (*self.build).jmp_label(&mut done);

                            // amount < -63: result is 0
                            (*self.build).set_label(&mut outOfRangeNegative);
                            (*self.build)
                                .xor_(OperandX64::reg(inst.reg_x64), OperandX64::reg(inst.reg_x64));

                            (*self.build).set_label(&mut done);
                        }
                    }
                }
                IrCmd::BITLROTATE_INT64 => {
                    let mut shiftTmp = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };

                    if (*get_op_mut(inst, (((1) as u32) as u32))).kind() != IrOpKind::Constant {
                        shiftTmp.take(RegisterX64::rcx);
                    }

                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::qword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );
                    let op0 = *get_op_mut(inst, 0);

                    if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
                        (*self.build)
                            .mov(OperandX64::reg(inst.reg_x64), self.mem_reg_int_64_op(op0));
                    }

                    if (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Constant {
                        let mut shift = (((self.int64_op((*get_op_mut(inst, 1)))) as u32) as i8);
                        (*self.build)
                            .rol(OperandX64::reg(inst.reg_x64), OperandX64::imm(shift as i32));
                    } else {
                        (*self.build).mov(
                            OperandX64::reg(shiftTmp.reg),
                            self.mem_reg_int_64_op((*get_op_mut(inst, 1))),
                        );
                        (*self.build).rol(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(byte_reg(shiftTmp.reg)),
                        );
                    }
                }
                IrCmd::BITRROTATE_INT64 => {
                    let mut shiftTmp = ScopedRegX64 {
                        owner: &mut self.regs,
                        reg: RegisterX64::noreg,
                    };

                    if (*get_op_mut(inst, (((1) as u32) as u32))).kind() != IrOpKind::Constant {
                        shiftTmp.take(RegisterX64::rcx);
                    }

                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::qword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );
                    let op0 = *get_op_mut(inst, 0);

                    if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
                        (*self.build)
                            .mov(OperandX64::reg(inst.reg_x64), self.mem_reg_int_64_op(op0));
                    }

                    if (*get_op_mut(inst, (((1) as u32) as u32))).kind() == IrOpKind::Constant {
                        let mut shift = (((self.int64_op((*get_op_mut(inst, 1)))) as u32) as i8);
                        (*self.build)
                            .ror(OperandX64::reg(inst.reg_x64), OperandX64::imm(shift as i32));
                    } else {
                        (*self.build).mov(
                            OperandX64::reg(shiftTmp.reg),
                            self.mem_reg_int_64_op((*get_op_mut(inst, 1))),
                        );
                        (*self.build).ror(
                            OperandX64::reg(inst.reg_x64),
                            OperandX64::reg(byte_reg(shiftTmp.reg)),
                        );
                    }
                }
                IrCmd::BITCOUNTLZ_INT64 => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::qword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );

                    let mut zero = Label::default();
                    let mut exit = Label::default();

                    (*self.build).test(
                        OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                        OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                    );
                    (*self.build).jcc(ConditionX64::Equal, &mut zero);

                    (*self.build).bsr(
                        inst.reg_x64,
                        OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                    );
                    (*self.build).xor_(OperandX64::reg(inst.reg_x64), OperandX64::imm(0x3f));
                    (*self.build).jmp_label(&mut exit);

                    (*self.build).set_label(&mut zero);
                    (*self.build).mov(OperandX64::reg(inst.reg_x64), OperandX64::imm((64) as i32));

                    (*self.build).set_label(&mut exit);
                }
                IrCmd::BITCOUNTRZ_INT64 => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::qword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );

                    let mut zero = Label::default();
                    let mut exit = Label::default();

                    (*self.build).test(
                        OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                        OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                    );
                    (*self.build).jcc(ConditionX64::Equal, &mut zero);

                    (*self.build).bsf(
                        inst.reg_x64,
                        OperandX64::reg(self.reg_op((*get_op_mut(inst, 0)))),
                    );
                    (*self.build).jmp_label(&mut exit);

                    (*self.build).set_label(&mut zero);
                    (*self.build).mov(OperandX64::reg(inst.reg_x64), OperandX64::imm((64) as i32));

                    (*self.build).set_label(&mut exit);
                }
                IrCmd::BYTESWAP_INT64 => {
                    inst.reg_x64 = self.regs.alloc_reg_or_reuse(
                        SizeX64::qword,
                        index,
                        &[(*get_op_mut(inst, (((0) as u32) as u32)))],
                    );
                    let op0 = *get_op_mut(inst, 0);

                    if op0.kind() != IrOpKind::Inst || inst.reg_x64 != self.reg_op(op0) {
                        (*self.build)
                            .mov(OperandX64::reg(inst.reg_x64), self.mem_reg_int_64_op(op0));
                    }

                    (*self.build).bswap(inst.reg_x64);
                }
                IrCmd::JUMP_CMP_PROTOID => {
                    {
                        CODEGEN_ASSERT!(
                            (*get_op_mut(inst, (((0) as u32) as u32))).kind() == IrOpKind::Inst
                        );
                        (*self.build).cmp(
                            OperandX64::mem(
                                SizeX64::byte,
                                RegisterX64::noreg,
                                1,
                                self.reg_op((*get_op_mut(inst, 0))),
                                (core::mem::offset_of!(Closure, isC) as i32),
                            ),
                            OperandX64::imm((1) as i32),
                        );
                        (*self.build)
                            .jcc(ConditionX64::Equal, self.label_op((*get_op_mut(inst, 3))));
                        {
                            let mut tmp = ScopedRegX64 {
                                owner: &mut self.regs,
                                reg: RegisterX64::noreg,
                            };
                            tmp.alloc(SizeX64::qword);
                            (*self.build).mov(
                                OperandX64::reg(tmp.reg),
                                OperandX64::mem(
                                    SizeX64::qword,
                                    RegisterX64::noreg,
                                    1,
                                    self.reg_op((*get_op_mut(inst, 0))),
                                    kClosureLPoffset,
                                ),
                            );
                            (*self.build).cmp(
                                OperandX64::mem(
                                    SizeX64::dword,
                                    RegisterX64::noreg,
                                    1,
                                    tmp.reg,
                                    (core::mem::offset_of!(Proto, funid) as i32),
                                ),
                                OperandX64::imm((self.uint_op((*get_op_mut(inst, 1)))) as i32),
                            );
                            (*self.build).jcc(
                                ConditionX64::NotEqual,
                                self.label_op((*get_op_mut(inst, 3))),
                            );
                        }
                        let target_block = self.block_op((*get_op_mut(inst, 2))) as *mut IrBlock;
                        self.jump_or_fallthrough(&mut *target_block, next);
                    }

                    // Pseudo instructions
                }
                IrCmd::NOP | IrCmd::SUBSTITUTE | IrCmd::MARK_USED | IrCmd::MARK_DEAD => {
                    CODEGEN_ASSERT!(false, "Pseudo instructions should not be lowered");
                }
                _ => {
                    CODEGEN_ASSERT!(false, "unhandled IrCmd");
                }
            }
            self.value_tracker.after_inst_lowering(inst, index);

            self.regs.curr_inst_idx = kInvalidInstIdx;

            self.regs.free_last_use_regs(inst, index);
        }
    }
}
