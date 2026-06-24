use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_op_kind::IrOpKind;
use crate::functions::is_gco::is_gco;
use crate::functions::is_non_terminating_jump::is_non_terminating_jump;
use crate::functions::try_get_operand_tag::try_get_operand_tag;
use crate::functions::try_replace_tag_with_full_store::try_replace_tag_with_full_store;
use crate::functions::try_replace_value_with_full_store::try_replace_value_with_full_store;
use crate::functions::try_replace_vector_value_with_full_store::try_replace_vector_value_with_full_store;
use crate::functions::update_remaining_uses::update_remaining_uses;
use crate::functions::vm_reg_op::vm_reg_op;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_block::IrBlock;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_function::IrFunction;
use crate::records::ir_inst::IrInst;
use crate::records::remove_dead_store_state::RemoveDeadStoreState;
use crate::records::store_reg_info::StoreRegInfo;
use luaur_vm::enums::lua_type::lua_Type;

#[inline]
fn reg_captured(function: &IrFunction, reg: i32) -> bool {
    (function.cfg.captured.regs[reg as usize / 64] & (1u64 << (reg as usize % 64))) != 0
}

// Port of IrVisitUseDef.h `visitVmRegDefsUses<T>` specialized for the RemoveDeadStoreState visitor
fn visit_vm_reg_defs_uses(
    state: &mut RemoveDeadStoreState,
    function: &mut IrFunction,
    inst: &mut IrInst,
) {
    // For correct analysis, all instruction uses must be handled before handling the definitions
    match inst.cmd {
        IrCmd::LOAD_TAG
        | IrCmd::LOAD_POINTER
        | IrCmd::LOAD_DOUBLE
        | IrCmd::LOAD_INT
        | IrCmd::LOAD_INT64
        | IrCmd::LOAD_FLOAT
        | IrCmd::LOAD_TVALUE => {
            state.maybe_use(inst.ops[0]);
        }
        IrCmd::STORE_TAG
        | IrCmd::STORE_EXTRA
        | IrCmd::STORE_POINTER
        | IrCmd::STORE_DOUBLE
        | IrCmd::STORE_INT
        | IrCmd::STORE_INT64
        | IrCmd::STORE_VECTOR
        | IrCmd::STORE_TVALUE
        | IrCmd::STORE_SPLIT_TVALUE => {
            state.maybe_def(inst.ops[0]);
        }
        IrCmd::CMP_ANY => {
            state.use_(inst.ops[0], 0);
            state.use_(inst.ops[1], 0);
        }
        IrCmd::CMP_TAG => {
            state.maybe_use(inst.ops[0]);
        }
        IrCmd::JUMP_IF_TRUTHY | IrCmd::JUMP_IF_FALSY => {
            state.use_(inst.ops[0], 0);
        }
        IrCmd::JUMP_EQ_TAG => {
            state.maybe_use(inst.ops[0]);
        }
        IrCmd::DO_ARITH => {
            state.maybe_use(inst.ops[1]);
            state.maybe_use(inst.ops[2]);
            state.def(inst.ops[0], 0);
        }
        IrCmd::GET_TABLE => {
            state.use_(inst.ops[1], 0);
            state.maybe_use(inst.ops[2]);
            state.def(inst.ops[0], 0);
        }
        IrCmd::SET_TABLE => {
            state.use_(inst.ops[0], 0);
            state.use_(inst.ops[1], 0);
            state.maybe_use(inst.ops[2]);
        }
        IrCmd::DO_LEN => {
            state.use_(inst.ops[1], 0);
            state.def(inst.ops[0], 0);
        }
        IrCmd::GET_CACHED_IMPORT => {
            state.def(inst.ops[0], 0);
        }
        IrCmd::CONCAT => {
            state.use_range(vm_reg_op(inst.ops[0]), function.uint_op(inst.ops[1]) as i32);
            state.def_range(vm_reg_op(inst.ops[0]), function.uint_op(inst.ops[1]) as i32);
        }
        IrCmd::GET_UPVALUE => {}
        IrCmd::SET_UPVALUE => {}
        IrCmd::INTERRUPT => {}
        IrCmd::BARRIER_OBJ | IrCmd::BARRIER_TABLE_FORWARD => {
            state.maybe_use(inst.ops[1]);
        }
        IrCmd::CLOSE_UPVALS => {}
        IrCmd::CAPTURE => {
            state.maybe_use(inst.ops[0]);

            if function.uint_op(inst.ops[1]) == 1 {
                state.capture(vm_reg_op(inst.ops[0]));
            }
        }
        IrCmd::SETLIST => {
            state.use_(inst.ops[1], 0);
            state.use_range(vm_reg_op(inst.ops[2]), function.int_op(inst.ops[3]));
        }
        IrCmd::CALL => {
            state.use_(inst.ops[0], 0);
            state.use_range(vm_reg_op(inst.ops[0]) + 1, function.int_op(inst.ops[1]));
            state.def_range(vm_reg_op(inst.ops[0]), function.int_op(inst.ops[2]));
        }
        IrCmd::RETURN => {
            state.use_range(vm_reg_op(inst.ops[0]), function.int_op(inst.ops[1]));
        }
        IrCmd::FASTCALL => {
            state.use_(inst.ops[2], 0);
            state.def_range(vm_reg_op(inst.ops[1]), function.int_op(inst.ops[3]));
        }
        IrCmd::INVOKE_FASTCALL => {
            let count = function.int_op(inst.ops[5]);
            if count != -1 {
                // Only LOP_FASTCALL3 lowering is allowed to have third optional argument
                if count >= 3 && inst.ops[4].kind() == IrOpKind::Undef {
                    CODEGEN_ASSERT!(
                        inst.ops[3].kind() == IrOpKind::VmReg
                            && vm_reg_op(inst.ops[3]) == vm_reg_op(inst.ops[2]) + 1
                    );

                    state.use_range(vm_reg_op(inst.ops[2]), count);
                } else {
                    if count >= 1 {
                        state.use_(inst.ops[2], 0);
                    }

                    if count >= 2 {
                        state.maybe_use(inst.ops[3]);
                    }

                    if count >= 3 {
                        state.maybe_use(inst.ops[4]);
                    }
                }
            } else {
                state.use_varargs(vm_reg_op(inst.ops[2]) as u8);
            }

            state.def_range(vm_reg_op(inst.ops[1]), function.int_op(inst.ops[6]));
        }
        IrCmd::FORGLOOP => {
            // First register is not used by instruction, we check that it's still 'nil' with CHECK_TAG
            state.use_(inst.ops[0], 1);
            state.use_(inst.ops[0], 2);

            state.def(inst.ops[0], 2);
            state.def_range(vm_reg_op(inst.ops[0]) + 3, function.int_op(inst.ops[1]));
        }
        IrCmd::FORGLOOP_FALLBACK => {
            state.use_range(vm_reg_op(inst.ops[0]), 3);

            state.def(inst.ops[0], 2);
            state.def_range(
                vm_reg_op(inst.ops[0]) + 3,
                (function.int_op(inst.ops[1]) as u8) as i32,
            );
        }
        IrCmd::FORGPREP_XNEXT_FALLBACK => {
            state.use_(inst.ops[1], 0);
        }
        IrCmd::FALLBACK_GETGLOBAL => {
            state.def(inst.ops[1], 0);
        }
        IrCmd::FALLBACK_SETGLOBAL => {
            state.use_(inst.ops[1], 0);
        }
        IrCmd::FALLBACK_GETTABLEKS => {
            state.use_(inst.ops[2], 0);
            state.def(inst.ops[1], 0);
        }
        IrCmd::FALLBACK_SETTABLEKS => {
            state.use_(inst.ops[1], 0);
            state.use_(inst.ops[2], 0);
        }
        IrCmd::FALLBACK_NAMECALL => {
            state.use_(inst.ops[2], 0);
            state.def_range(vm_reg_op(inst.ops[1]), 2);
        }
        IrCmd::FALLBACK_PREPVARARGS => {}
        IrCmd::FALLBACK_GETVARARGS => {
            state.def_range(vm_reg_op(inst.ops[1]), function.int_op(inst.ops[2]));
        }
        IrCmd::FALLBACK_DUPCLOSURE => {
            state.def(inst.ops[1], 0);
        }
        IrCmd::FALLBACK_FORGPREP => {
            // This instruction doesn't always redefine Rn, Rn+1, Rn+2, so we have to mark it as implicit use
            state.use_range(vm_reg_op(inst.ops[1]), 3);
            state.def_range(vm_reg_op(inst.ops[1]), 3);
        }
        IrCmd::ADJUST_STACK_TO_REG => {
            state.def_range(vm_reg_op(inst.ops[0]), -1);
        }
        IrCmd::ADJUST_STACK_TO_TOP => {}
        IrCmd::GET_TYPEOF => {
            state.use_(inst.ops[0], 0);
        }
        IrCmd::FINDUPVAL => {
            state.use_(inst.ops[0], 0);
        }
        IrCmd::MARK_USED => {
            state.use_range(vm_reg_op(inst.ops[0]), function.int_op(inst.ops[1]));
        }
        IrCmd::MARK_DEAD => {}
        _ => {
            // All instructions which reference registers have to be handled explicitly
            let n = inst.ops.size();
            for idx in 0..n {
                CODEGEN_ASSERT!(inst.ops[idx as usize].kind() != IrOpKind::VmReg);
            }
        }
    }
}

pub fn mark_dead_stores_in_inst(
    state: &mut RemoveDeadStoreState,
    build: &mut IrBuilder,
    function: &mut IrFunction,
    block: &mut IrBlock,
    inst: &mut IrInst,
    index: u32,
) {
    update_remaining_uses(state, inst, index);

    let nil = lua_Type::LUA_TNIL as u8;

    match inst.cmd {
        IrCmd::STORE_TAG => {
            if inst.ops[0].kind() == IrOpKind::VmReg {
                let reg = vm_reg_op(inst.ops[0]);

                if reg_captured(function, reg) {
                    return;
                }

                let reg_info: &mut StoreRegInfo =
                    unsafe { &mut *(&mut state.info[reg as usize] as *mut StoreRegInfo) };

                reg_info.ignore_at_exit = false;

                if !try_replace_tag_with_full_store(
                    state,
                    build,
                    function,
                    block,
                    index,
                    inst.ops[0],
                    inst.ops[1],
                    reg_info,
                ) {
                    let tag = function.tag_op(inst.ops[1]);

                    reg_info.tag_inst_idx = index;

                    if state.tag_value_pair_established(reg_info) {
                        if tag == nil {
                            reg_info.value_inst_idx = !0u32;
                        }

                        reg_info.tvalue_inst_idx = !0u32;
                    }

                    reg_info.maybe_gco = is_gco(tag);
                    reg_info.known_tag = tag;
                    state.has_gco_to_clear |= reg_info.maybe_gco;
                }
            }
        }
        IrCmd::STORE_EXTRA => {
            // To simplify, extra field store is preserved along with all other stores made so far
            if inst.ops[0].kind() == IrOpKind::VmReg {
                let reg = vm_reg_op(inst.ops[0]);
                state.use_reg(reg as u8);
                state.info[reg as usize].ignore_at_exit = false;
            }
        }
        IrCmd::STORE_POINTER => {
            if inst.ops[0].kind() == IrOpKind::VmReg {
                let reg = vm_reg_op(inst.ops[0]);

                if reg_captured(function, reg) {
                    return;
                }

                let reg_info: &mut StoreRegInfo =
                    unsafe { &mut *(&mut state.info[reg as usize] as *mut StoreRegInfo) };

                reg_info.ignore_at_exit = false;

                let maybe_gco;

                if luaur_common::FFlag::LuauCodegenDsePtrStoreTagCheck.get() {
                    // If we have a known tag and it is not a pointer, we cannot generate a full store in invalid form
                    maybe_gco = reg_info.known_tag == 0xff || is_gco(reg_info.known_tag);

                    if maybe_gco
                        && try_replace_value_with_full_store(
                            state,
                            build,
                            function,
                            block,
                            index,
                            inst.ops[0],
                            inst.ops[1],
                            reg_info,
                        )
                    {
                        reg_info.maybe_gco = true;
                        state.has_gco_to_clear = true;
                        return mark_dead_stores_in_inst_tail(state, function, inst);
                    }
                } else {
                    if try_replace_value_with_full_store(
                        state,
                        build,
                        function,
                        block,
                        index,
                        inst.ops[0],
                        inst.ops[1],
                        reg_info,
                    ) {
                        reg_info.maybe_gco = true;
                        state.has_gco_to_clear |= true;
                        return mark_dead_stores_in_inst_tail(state, function, inst);
                    }
                    maybe_gco = true;
                }

                // Partial value store can be removed by a new one if the tag is known
                if reg_info.known_tag != 0xff {
                    state.kill_value_store(reg_info);
                }

                reg_info.value_inst_idx = index;

                if state.tag_value_pair_established(reg_info) {
                    reg_info.tvalue_inst_idx = !0u32;
                }

                if luaur_common::FFlag::LuauCodegenDsePtrStoreTagCheck.get() {
                    // While pointer was stored, TValue can still be under a non-GCO tag
                    reg_info.maybe_gco = maybe_gco;
                    state.has_gco_to_clear |= maybe_gco;
                } else {
                    reg_info.maybe_gco = true;
                    state.has_gco_to_clear = true;
                }
            }
        }
        IrCmd::STORE_DOUBLE | IrCmd::STORE_INT64 | IrCmd::STORE_INT => {
            if inst.ops[0].kind() == IrOpKind::VmReg {
                let reg = vm_reg_op(inst.ops[0]);

                if reg_captured(function, reg) {
                    return;
                }

                let reg_info: &mut StoreRegInfo =
                    unsafe { &mut *(&mut state.info[reg as usize] as *mut StoreRegInfo) };

                reg_info.ignore_at_exit = false;

                if !try_replace_value_with_full_store(
                    state,
                    build,
                    function,
                    block,
                    index,
                    inst.ops[0],
                    inst.ops[1],
                    reg_info,
                ) {
                    // Partial value store can be removed by a new one if the tag is known
                    if reg_info.known_tag != 0xff {
                        state.kill_value_store(reg_info);
                    }

                    reg_info.value_inst_idx = index;

                    if state.tag_value_pair_established(reg_info) {
                        reg_info.tvalue_inst_idx = !0u32;
                    }

                    reg_info.maybe_gco = false;
                }
            }
        }
        IrCmd::STORE_VECTOR => {
            if inst.ops[0].kind() == IrOpKind::VmReg {
                let reg = vm_reg_op(inst.ops[0]);

                if reg_captured(function, reg) {
                    return;
                }

                let reg_info: &mut StoreRegInfo =
                    unsafe { &mut *(&mut state.info[reg as usize] as *mut StoreRegInfo) };

                reg_info.ignore_at_exit = false;

                if !try_replace_vector_value_with_full_store(
                    state, build, function, block, index, reg_info,
                ) {
                    // Partial value store can be removed by a new one if the tag is known
                    if reg_info.known_tag != 0xff {
                        state.kill_value_store(reg_info);
                    }

                    reg_info.value_inst_idx = index;

                    if state.tag_value_pair_established(reg_info) {
                        reg_info.tvalue_inst_idx = !0u32;
                    }

                    reg_info.maybe_gco = false;
                }
            }
        }
        IrCmd::STORE_TVALUE => {
            if inst.ops[0].kind() == IrOpKind::VmReg {
                let reg = vm_reg_op(inst.ops[0]);

                if reg_captured(function, reg) {
                    return;
                }

                let reg_info: &mut StoreRegInfo =
                    unsafe { &mut *(&mut state.info[reg as usize] as *mut StoreRegInfo) };

                reg_info.ignore_at_exit = false;

                state.kill_tag_and_value_store_pair(reg_info);
                state.kill_t_value_store(reg_info);

                reg_info.tag_inst_idx = !0u32;
                reg_info.value_inst_idx = !0u32;

                reg_info.tvalue_inst_idx = index;

                reg_info.known_tag = try_get_operand_tag(function, inst.ops[1]).unwrap_or(0xff);
                reg_info.maybe_gco = reg_info.known_tag == 0xff || is_gco(reg_info.known_tag);

                state.has_gco_to_clear |= reg_info.maybe_gco;
            }
        }
        IrCmd::STORE_SPLIT_TVALUE => {
            if inst.ops[0].kind() == IrOpKind::VmReg {
                let reg = vm_reg_op(inst.ops[0]);

                if reg_captured(function, reg) {
                    return;
                }

                let reg_info: &mut StoreRegInfo =
                    unsafe { &mut *(&mut state.info[reg as usize] as *mut StoreRegInfo) };

                reg_info.ignore_at_exit = false;

                state.kill_tag_and_value_store_pair(reg_info);
                state.kill_t_value_store(reg_info);

                reg_info.tag_inst_idx = !0u32;
                reg_info.value_inst_idx = !0u32;

                reg_info.tvalue_inst_idx = index;
                let tag = function.tag_op(inst.ops[1]);
                reg_info.maybe_gco = is_gco(tag);
                reg_info.known_tag = tag;
                state.has_gco_to_clear |= reg_info.maybe_gco;
            }
        }

        // Guard checks can jump to a block which might be using some or all the values we stored
        IrCmd::CHECK_TAG => {
            state.check_live_ins(inst.ops.get(2).copied().unwrap_or_default(), index, true);

            // Tag guard establishes the tag value of the register in the current block
            let load_ptr = function.as_inst_op(inst.ops[0]);
            if !load_ptr.is_null() {
                let load = unsafe { &*load_ptr };
                if load.cmd == IrCmd::LOAD_TAG && load.ops[0].kind() == IrOpKind::VmReg {
                    let reg = vm_reg_op(load.ops[0]);
                    let tag = function.tag_op(inst.ops[1]);
                    state.info[reg as usize].known_tag = tag;
                }
            }
        }
        IrCmd::TRY_NUM_TO_INDEX => {
            state.check_live_ins(inst.ops[1], index, true);
        }
        IrCmd::TRY_CALL_FASTGETTM => {
            state.check_live_ins(inst.ops[2], index, true);
        }
        IrCmd::CHECK_FASTCALL_RES => {
            state.check_live_ins(inst.ops[1], index, true);
        }
        IrCmd::CHECK_TRUTHY => {
            // This instruction has two jumps to the exit in the lowering and that prevents exit sync record from being generated
            state.check_live_ins(inst.ops[2], index, false);
        }
        IrCmd::CHECK_READONLY => {
            state.check_live_ins(inst.ops[1], index, true);
        }
        IrCmd::CHECK_NO_METATABLE => {
            state.check_live_ins(inst.ops[1], index, true);
        }
        IrCmd::CHECK_SAFE_ENV => {
            state.check_live_ins(inst.ops[0], index, true);
        }
        IrCmd::CHECK_ARRAY_SIZE => {
            state.check_live_ins(inst.ops[2], index, true);
        }
        IrCmd::CHECK_DIV_INT64 => {
            // This instruction has two jumps to the exit in the lowering and that prevents exit sync record from being generated
            state.check_live_ins(inst.ops[2], index, false);
        }
        IrCmd::CHECK_SLOT_MATCH => {
            state.check_live_ins(inst.ops[2], index, true);
        }
        IrCmd::CHECK_NODE_NO_NEXT => {
            state.check_live_ins(inst.ops[1], index, true);
        }
        IrCmd::CHECK_NODE_VALUE => {
            state.check_live_ins(inst.ops[1], index, true);
        }
        IrCmd::CHECK_BUFFER_LEN => {
            state.check_live_ins(inst.ops[5], index, true);
        }
        IrCmd::CHECK_USERDATA_TAG => {
            state.check_live_ins(inst.ops[2], index, true);
        }
        IrCmd::CHECK_CMP_NUM | IrCmd::CHECK_CMP_INT | IrCmd::CHECK_CMP_INT64 => {
            state.check_live_ins(inst.ops[3], index, true);
        }

        IrCmd::JUMP_IF_TRUTHY
        | IrCmd::JUMP_IF_FALSY
        | IrCmd::JUMP_EQ_TAG
        | IrCmd::JUMP_CMP_INT
        | IrCmd::JUMP_EQ_POINTER
        | IrCmd::JUMP_CMP_NUM
        | IrCmd::JUMP_CMP_FLOAT
        | IrCmd::JUMP_FORN_LOOP_COND
        | IrCmd::JUMP_SLOT_MATCH
        | IrCmd::JUMP_CMP_PROTOID => {
            visit_vm_reg_defs_uses(state, function, inst);
            state.check_live_outs(block);
        }

        IrCmd::JUMP => {
            // Ideally, we would be able to remove stores to registers that are not live out from a block
            // But during chain optimizations, we rely on data stored in the predecessor even when it's not an explicit live out
        }
        IrCmd::RETURN => {
            visit_vm_reg_defs_uses(state, function, inst);

            // At the end of a function, we can kill stores to registers that are not live out
            state.check_live_outs(block);
        }
        IrCmd::ADJUST_STACK_TO_REG => {
            // visitVmRegDefsUses considers adjustment as the fast call register definition point, but for dead store removal, we count the actual writes
        }

        // This group of instructions can trigger GC assist internally
        IrCmd::CMP_ANY
        | IrCmd::DO_ARITH
        | IrCmd::DO_LEN
        | IrCmd::GET_TABLE
        | IrCmd::SET_TABLE
        | IrCmd::GET_CACHED_IMPORT
        | IrCmd::CONCAT
        | IrCmd::INTERRUPT
        | IrCmd::CHECK_GC
        | IrCmd::CALL
        | IrCmd::FORGLOOP_FALLBACK
        | IrCmd::FALLBACK_GETGLOBAL
        | IrCmd::FALLBACK_SETGLOBAL
        | IrCmd::FALLBACK_GETTABLEKS
        | IrCmd::FALLBACK_SETTABLEKS
        | IrCmd::FALLBACK_NAMECALL
        | IrCmd::FALLBACK_DUPCLOSURE
        | IrCmd::FALLBACK_FORGPREP => {
            if state.has_gco_to_clear {
                state.flush_gco_regs();
            }

            visit_vm_reg_defs_uses(state, function, inst);
        }

        IrCmd::NEW_USERDATA => {
            state.has_allocations = true;
        }

        IrCmd::MARK_DEAD => {
            state.mark_unused_at_exit(vm_reg_op(inst.ops[0]), function.int_op(inst.ops[1]));
        }

        _ => {
            // Guards have to be covered explicitly
            CODEGEN_ASSERT!(!is_non_terminating_jump(inst.cmd));
            visit_vm_reg_defs_uses(state, function, inst);
        }
    }

    mark_dead_stores_in_inst_tail(state, function, inst);
}

// The trailing `if (FFlag::LuauCodegenVmExitSync)` switch from markDeadStoresInInst
fn mark_dead_stores_in_inst_tail(
    state: &mut RemoveDeadStoreState,
    _function: &mut IrFunction,
    inst: &mut IrInst,
) {
    if luaur_common::FFlag::LuauCodegenVmExitSync.get() {
        // Pending stores with SSA operands must not be deferred to ExitSync blocks past instructions that can invalidate operand physical location
        match inst.cmd {
            IrCmd::CMP_ANY
            | IrCmd::DO_ARITH
            | IrCmd::DO_LEN
            | IrCmd::GET_TABLE
            | IrCmd::SET_TABLE
            | IrCmd::CONCAT
            | IrCmd::GET_CACHED_IMPORT
            | IrCmd::FORGLOOP_FALLBACK
            | IrCmd::FALLBACK_GETGLOBAL
            | IrCmd::FALLBACK_SETGLOBAL
            | IrCmd::FALLBACK_GETTABLEKS
            | IrCmd::FALLBACK_SETTABLEKS
            | IrCmd::FALLBACK_NAMECALL
            | IrCmd::FALLBACK_DUPCLOSURE
            | IrCmd::FALLBACK_FORGPREP
            | IrCmd::CALL
            | IrCmd::SETLIST
            | IrCmd::FORGLOOP => {
                state.invalidate_value_propagation();
            }
            _ => {}
        }
    }
}
