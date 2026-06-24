use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_const_kind::IrConstKind;
use crate::enums::ir_op_kind::IrOpKind;
use crate::functions::compare_ir_utils::compare_f64_f64_ir_condition;
use crate::functions::compare_ir_utils_alt_b::compare_i32_i32_ir_condition;
use crate::functions::condition_op::condition_op;
use crate::functions::fold_constants::fold_constants;
use crate::functions::handle_builtin_effects::handle_builtin_effects;
use crate::functions::is_gco::is_gco;
use crate::functions::produces_dirty_high_register_bits::produces_dirty_high_register_bits;
use crate::functions::replace_ir_utils::replace_ir_function_ir_op_ir_op;
use crate::functions::replace_ir_utils_alt_b::replace_ir_function_ir_block_u32_ir_inst;
use crate::functions::safe_integer_constant::safe_integer_constant;
use crate::functions::substitute::substitute;
use crate::functions::substitute_with_truncated_uint::substitute_with_truncated_uint;
use crate::functions::try_get_operand_tag::try_get_operand_tag;
use crate::functions::try_get_tag_for_typename::try_get_tag_for_typename;
use crate::functions::vm_const_op::vm_const_op;
use crate::macros::has_op_c::HAS_OP_C;
use crate::macros::op_a::op_a;
use crate::macros::op_b::op_b;
use crate::macros::op_c::op_c;
use crate::macros::op_d::op_d;
use crate::macros::op_e::op_e;
use crate::macros::op_f::op_f;
use crate::macros::op_g::op_g;
use crate::macros::opt_op_b::OPT_OP_B;
use crate::macros::opt_op_c::OPT_OP_C;
use crate::macros::opt_op_d::OPT_OP_D;
use crate::records::array_value_entry::ArrayValueEntry;
use crate::records::const_prop_state::ConstPropState;
use crate::records::ir_block::IrBlock;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_function::IrFunction;
use crate::records::ir_inst::IrInst;
use crate::records::ir_op::IrOp;
use crate::records::node_slot_state::NodeSlotState;
use crate::records::numbered_instruction::NumberedInstruction;
use crate::type_aliases::ir_ops::IrOps;
use luaur_common::enums::luau_builtin_function::LuauBuiltinFunction;
use luaur_vm::enums::lua_type::lua_Type;
use luaur_vm::macros::getstr::getstr;
use luaur_vm::macros::tsvalue::tsvalue;
use luaur_vm::macros::ttisvector::ttisvector;
use luaur_vm::macros::vvalue::vvalue;
use luaur_vm::type_aliases::t_value::TValue;

fn const_prop_make_inst(cmd: IrCmd, ops: &[IrOp]) -> IrInst {
    let mut ir_ops = IrOps::new();
    for &op in ops {
        ir_ops.push(op);
    }

    IrInst {
        cmd,
        ops: ir_ops,
        ..IrInst::default()
    }
}

fn tag_for_vm_const_typename(function: &IrFunction, source: IrOp, for_typeof: bool) -> Option<u8> {
    if source.kind() != IrOpKind::VmConst || function.proto.is_null() {
        return None;
    }

    unsafe {
        let constants = (*function.proto).k;
        if constants.is_null() {
            return None;
        }

        let value = constants.add(vm_const_op(source) as usize);
        if (*value).tt != lua_Type::LUA_TSTRING as core::ffi::c_int {
            return None;
        }

        let string = tsvalue!(value as *const TValue);
        let name = core::ffi::CStr::from_ptr(getstr(string)).to_str().ok()?;
        let tag = try_get_tag_for_typename(name, for_typeof);

        if tag == 0xff {
            None
        } else {
            Some(tag)
        }
    }
}

fn type_name_tag_comparison(
    function: &IrFunction,
    lhs_op: IrOp,
    rhs_op: IrOp,
) -> Option<(IrOp, u8)> {
    if lhs_op.kind() != IrOpKind::Inst || rhs_op.kind() != IrOpKind::Inst {
        return None;
    }

    let mut lhs = function.instructions.get(lhs_op.index() as usize)?.clone();
    let mut rhs = function.instructions.get(rhs_op.index() as usize)?.clone();

    if lhs.cmd != IrCmd::GET_TYPE && lhs.cmd != IrCmd::GET_TYPEOF {
        return None;
    }

    if rhs.cmd != IrCmd::LOAD_POINTER {
        return None;
    }

    let source = op_a(&mut lhs);
    let tag = tag_for_vm_const_typename(function, op_a(&mut rhs), lhs.cmd == IrCmd::GET_TYPEOF)?;

    Some((source, tag))
}

pub fn const_prop_in_inst(
    state: &mut ConstPropState,
    build: &mut IrBuilder,
    function: &mut IrFunction,
    block: &mut IrBlock,
    inst: &mut IrInst,
    index: u32,
) {
    match inst.cmd {
        IrCmd::LOAD_TAG => {
            let source = op_a(inst);
            let tag = state.try_get_tag(source);
            if tag != 0xff {
                let tag_op = build.const_tag(tag);
                substitute(function, inst, tag_op);
            } else if source.kind() == IrOpKind::VmReg {
                if state.substitute_tag_load_with_t_value_data(build, inst) {
                    return;
                }

                if luaur_common::FFlag::LuauCodegenLoadPropagateOrigin.get() {
                    state.try_redirect_vm_reg_load_to_t_value_origin(inst);
                }

                state.substitute_or_record_vm_reg_load(inst);
            }
        }
        IrCmd::LOAD_POINTER => {
            let source = op_a(inst);
            if source.kind() == IrOpKind::VmReg {
                if state.substitute_or_record_value_load_with_t_value_data(build, inst) {
                    return;
                }

                if luaur_common::FFlag::LuauCodegenLoadPropagateOrigin.get() {
                    state.try_redirect_vm_reg_load_to_t_value_origin(inst);
                }

                state.substitute_or_record_vm_reg_load(inst);
            }
        }
        IrCmd::LOAD_DOUBLE => {
            let source = op_a(inst);
            let value = state.try_get_value(source);

            if function.as_double_op(value).is_some() {
                substitute(function, inst, value);
            } else if source.kind() == IrOpKind::VmReg {
                if state.substitute_or_record_value_load_with_t_value_data(build, inst) {
                    return;
                }

                if luaur_common::FFlag::LuauCodegenLoadPropagateOrigin.get() {
                    state.try_redirect_vm_reg_load_to_t_value_origin(inst);
                }

                state.substitute_or_record_vm_reg_load(inst);
            } else {
                state.substitute_or_record(inst, index);
            }
        }
        IrCmd::LOAD_INT => {
            let source = op_a(inst);
            let value = state.try_get_value(source);

            if function.as_int_op(value).is_some() {
                substitute(function, inst, value);
            } else if source.kind() == IrOpKind::VmReg {
                if state.substitute_or_record_value_load_with_t_value_data(build, inst) {
                    return;
                }

                if luaur_common::FFlag::LuauCodegenLoadPropagateOrigin.get() {
                    state.try_redirect_vm_reg_load_to_t_value_origin(inst);
                }

                state.substitute_or_record_vm_reg_load(inst);
            } else {
                state.substitute_or_record(inst, index);
            }
        }
        IrCmd::LOAD_INT64 => {
            let source = op_a(inst);
            let value = state.try_get_value(source);

            if function.as_int_64_op(value).is_some() {
                substitute(function, inst, value);
            } else if source.kind() == IrOpKind::VmReg {
                if state.substitute_or_record_value_load_with_t_value_data(build, inst) {
                    return;
                }

                if luaur_common::FFlag::LuauCodegenLoadPropagateOrigin.get() {
                    state.try_redirect_vm_reg_load_to_t_value_origin(inst);
                }

                state.substitute_or_record_vm_reg_load(inst);
            } else {
                state.substitute_or_record(inst, index);
            }
        }
        IrCmd::LOAD_TVALUE => {
            let source = op_a(inst);
            if source.kind() == IrOpKind::VmReg {
                if !state.substitute_or_record_vm_reg_load(inst) && !HAS_OP_C!(inst) {
                    let tag = state.try_get_tag(source);
                    if tag != 0xff {
                        let offset = build.const_int(0);
                        let tag_op = build.const_tag(tag);
                        let mut ops = crate::type_aliases::ir_ops::IrOps::new();
                        ops.push(source);
                        ops.push(offset);
                        ops.push(tag_op);
                        replace_ir_function_ir_block_u32_ir_inst(
                            function,
                            block,
                            index,
                            IrInst {
                                cmd: IrCmd::LOAD_TVALUE,
                                ops,
                                ..IrInst::default()
                            },
                        );
                    }
                }
            } else if source.kind() == IrOpKind::Inst {
                let source_inst = function.instructions[source.index() as usize].clone();

                if source_inst.cmd == IrCmd::GET_SLOT_NODE_ADDR {
                    if let Some(prev_idx) = state.hash_value_cache.find(&source.index()).copied() {
                        if prev_idx != crate::records::ir_data::k_invalid_inst_idx {
                            let prev = function.instructions[prev_idx as usize].clone();

                            if prev.cmd == IrCmd::LOAD_TVALUE {
                                if prev.use_count != 0 {
                                    substitute(
                                        function,
                                        inst,
                                        IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, prev_idx),
                                    );
                                }
                            } else if prev.cmd == IrCmd::STORE_SPLIT_TVALUE {
                                state
                                    .inst_tag
                                    .try_insert(index, function.tag_op(op_b(prev.clone())));
                                state.inst_value.try_insert(index, op_c(prev));
                            } else if luaur_common::FFlag::LuauCodegenExtraTableOpts.get()
                                && prev.cmd == IrCmd::STORE_TVALUE
                            {
                                let arg = function.as_inst_op(op_b(prev.clone()));
                                if !arg.is_null() && unsafe { (*arg).use_count } != 0 {
                                    substitute(function, inst, op_b(prev));
                                }
                            }

                            return;
                        }
                    }

                    *state.hash_value_cache.get_or_insert(source.index()) = index;
                } else if source_inst.cmd == IrCmd::GET_ARR_ADDR {
                    let mut source_addr = source_inst;
                    let offset_op = state.get_combined_array_load_offset_op(
                        &mut source_addr,
                        OPT_OP_B(inst.clone()),
                    );

                    if let Some(entry) = state
                        .array_value_cache
                        .iter()
                        .find(|entry| entry.pointer == source.index() && entry.offset == offset_op)
                        .copied()
                    {
                        if entry.value != crate::records::ir_data::k_invalid_inst_idx {
                            let prev = function.instructions[entry.value as usize].clone();

                            if prev.cmd == IrCmd::LOAD_TVALUE {
                                if prev.use_count != 0 {
                                    substitute(
                                        function,
                                        inst,
                                        IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, entry.value),
                                    );
                                }
                            } else if prev.cmd == IrCmd::STORE_SPLIT_TVALUE {
                                state
                                    .inst_tag
                                    .try_insert(index, function.tag_op(op_b(prev.clone())));
                                state.inst_value.try_insert(index, op_c(prev));
                            } else if luaur_common::FFlag::LuauCodegenExtraTableOpts.get()
                                && prev.cmd == IrCmd::STORE_TVALUE
                            {
                                let arg = function.as_inst_op(op_b(prev.clone()));
                                if !arg.is_null() && unsafe { (*arg).use_count } != 0 {
                                    substitute(function, inst, op_b(prev));
                                }
                            }

                            return;
                        }
                    }

                    state.array_value_cache.push(ArrayValueEntry {
                        pointer: source.index(),
                        offset: offset_op,
                        value: index,
                    });
                } else {
                    state.substitute_or_record(inst, index);
                }
            }
        }
        IrCmd::LOAD_FLOAT => {
            let source = op_a(inst);
            if source.kind() == IrOpKind::VmReg {
                let offset = function.int_op(op_b(inst.clone()));

                if let Some(subst) =
                    state.find_substitute_component_load_from_store_vector(build, source, offset)
                {
                    substitute(function, inst, subst);
                    return;
                }

                if let Some(prev_idx_ptr) =
                    state.get_previous_versioned_load_index(IrCmd::LOAD_TVALUE, source)
                {
                    let mut prev_idx = unsafe { *prev_idx_ptr };
                    let prev = function.instructions[prev_idx as usize].clone();

                    if prev.cmd == IrCmd::TAG_VECTOR {
                        let mut prev_for_arg = prev.clone();
                        let prev_arg = op_a(&mut prev_for_arg);
                        if !function.as_inst_op(prev_arg).is_null() {
                            prev_idx = prev_arg.index();
                        }
                    }

                    let value = function.instructions[prev_idx as usize].clone();
                    let byte_offset = offset as u32;
                    crate::macros::codegen_assert::CODEGEN_ASSERT!(byte_offset % 4 == 0);

                    let component = byte_offset / 4;
                    crate::macros::codegen_assert::CODEGEN_ASSERT!(component <= 3);

                    if value.cmd == IrCmd::LOAD_TVALUE {
                        let mut value_for_source = value.clone();
                        let value_source = op_a(&mut value_for_source);

                        if value_source.kind() == IrOpKind::VmConst && !function.proto.is_null() {
                            let tv = unsafe {
                                (*function.proto).k.add(vm_const_op(value_source) as usize)
                            };

                            if ttisvector!(tv as *const TValue) {
                                let v = unsafe { vvalue!(tv as *const TValue) };
                                let subst = build.const_double(v[component as usize] as f64);
                                substitute(function, inst, subst);
                                return;
                            }
                        } else if value_source.kind() == IrOpKind::VmReg {
                            let prev_op = IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, prev_idx);
                            if state.try_get_reg_link(prev_op).is_some() {
                                if let Some(subst) = state
                                    .find_substitute_component_load_from_store_vector(
                                        build,
                                        value_source,
                                        offset,
                                    )
                                {
                                    substitute(function, inst, subst);
                                    return;
                                }
                            }
                        }
                    }

                    let mut ops = crate::type_aliases::ir_ops::IrOps::new();
                    ops.push(IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, prev_idx));
                    ops.push(build.const_int(component as i32));

                    replace_ir_function_ir_block_u32_ir_inst(
                        function,
                        block,
                        index,
                        IrInst {
                            cmd: IrCmd::EXTRACT_VEC,
                            ops,
                            ..IrInst::default()
                        },
                    );

                    state.substitute_or_record(&mut function.instructions[index as usize], index);
                    return;
                }

                state.substitute_or_record_vm_reg_load(inst);
            } else {
                state.substitute_or_record(inst, index);
            }
        }
        IrCmd::STORE_TAG => {
            let target = op_a(inst);
            if target.kind() == IrOpKind::VmReg {
                let mut active_load_cmd = IrCmd::NOP;
                let mut active_load_value = !0u32;
                let value = op_b(inst.clone());
                if value.kind() == IrOpKind::Constant {
                    let tag = function.tag_op(value);
                    (active_load_cmd, active_load_value) =
                        state.get_previous_versioned_load_for_tag(tag, target);

                    if state.try_get_tag(target) == tag {
                        crate::functions::kill_ir_utils::kill_ir_function_ir_inst(function, inst);
                    } else {
                        state.save_tag(target, tag);
                        if tag == lua_Type::LUA_TNIL as u8 {
                            state.invalidate_value(target);
                        }
                    }
                } else {
                    state.invalidate_tag(target);
                }

                if active_load_value != !0u32 {
                    let key = state.versioned_vm_reg_load_ir_cmd_ir_op(active_load_cmd, target);
                    *state.value_map.get_or_insert(key) = active_load_value;
                }
            }
        }
        IrCmd::STORE_POINTER => {
            let target = op_a(inst);
            if target.kind() == IrOpKind::VmReg {
                let value = op_b(inst.clone());
                if value.kind() == IrOpKind::Inst {
                    if let Some(prev_idx) =
                        state.get_previous_versioned_load_index(IrCmd::LOAD_POINTER, target)
                    {
                        if unsafe { *prev_idx } == value.index() {
                            crate::functions::kill_ir_utils::kill_ir_function_ir_inst(
                                function, inst,
                            );
                            return;
                        }
                    }
                }

                state.invalidate_value(target);
                if value.kind() == IrOpKind::Inst {
                    state.forward_vm_reg_store_to_load(inst, IrCmd::LOAD_POINTER);

                    let value_ptr = function.as_inst_op(value);
                    if !luaur_common::FFlag::LuauCodegenExtraTableOpts.get()
                        && !value_ptr.is_null()
                        && unsafe { (*value_ptr).cmd } == IrCmd::NEW_TABLE
                    {
                        if let Some(info) = state.try_get_register_info(target) {
                            unsafe {
                                let array_size_op = (&(*value_ptr).ops)[0];
                                (*info).known_not_readonly_deprecated = true;
                                (*info).known_no_metatable_deprecated = true;
                                (*info).known_table_array_size_deprecated =
                                    function.uint_op(array_size_op) as i32;
                            }
                        }
                    }
                }
            }
        }
        IrCmd::STORE_DOUBLE => {
            let target = op_a(inst);
            if target.kind() == IrOpKind::VmReg {
                let value = op_b(inst.clone());
                if value.kind() == IrOpKind::Constant {
                    if state.try_get_value(target) == value {
                        crate::functions::kill_ir_utils::kill_ir_function_ir_inst(function, inst);
                    } else {
                        state.save_value(target, value);
                    }
                } else {
                    if value.kind() == IrOpKind::Inst {
                        if let Some(prev_idx) =
                            state.get_previous_versioned_load_index(IrCmd::LOAD_DOUBLE, target)
                        {
                            if unsafe { *prev_idx } == value.index() {
                                crate::functions::kill_ir_utils::kill_ir_function_ir_inst(
                                    function, inst,
                                );
                                return;
                            }
                        }
                    }

                    state.invalidate_value(target);
                    state.forward_vm_reg_store_to_load(inst, IrCmd::LOAD_DOUBLE);
                }
            }
        }
        IrCmd::STORE_INT => {
            let stored = op_b(inst.clone());
            let stored_inst = function.as_inst_op(stored);
            if !stored_inst.is_null() && unsafe { (*stored_inst).cmd } == IrCmd::TRUNCATE_UINT {
                let mut stored_inst_clone = unsafe { (*stored_inst).clone() };
                let replacement = op_a(&mut stored_inst_clone);
                replace_ir_function_ir_op_ir_op(function, &mut inst.ops[1], replacement);
            }

            let target = op_a(inst);
            if target.kind() == IrOpKind::VmReg {
                let value = op_b(inst.clone());
                if value.kind() == IrOpKind::Constant {
                    if state.try_get_value(target) == value {
                        crate::functions::kill_ir_utils::kill_ir_function_ir_inst(function, inst);
                    } else {
                        state.save_value(target, value);
                    }
                } else {
                    if value.kind() == IrOpKind::Inst {
                        if let Some(prev_idx) =
                            state.get_previous_versioned_load_index(IrCmd::LOAD_INT, target)
                        {
                            if unsafe { *prev_idx } == value.index() {
                                crate::functions::kill_ir_utils::kill_ir_function_ir_inst(
                                    function, inst,
                                );
                                return;
                            }
                        }
                    }

                    state.invalidate_value(target);
                    state.forward_vm_reg_store_to_load(inst, IrCmd::LOAD_INT);
                }
            }
        }
        IrCmd::STORE_INT64 => {
            let target = op_a(inst);
            if target.kind() == IrOpKind::VmReg {
                let value = op_b(inst.clone());
                if value.kind() == IrOpKind::Constant {
                    if state.try_get_value(target) == value {
                        crate::functions::kill_ir_utils::kill_ir_function_ir_inst(function, inst);
                    } else {
                        state.save_value(target, value);
                    }
                } else {
                    if value.kind() == IrOpKind::Inst {
                        if let Some(prev_idx) =
                            state.get_previous_versioned_load_index(IrCmd::LOAD_INT64, target)
                        {
                            if unsafe { *prev_idx } == value.index() {
                                crate::functions::kill_ir_utils::kill_ir_function_ir_inst(
                                    function, inst,
                                );
                                return;
                            }
                        }
                    }

                    state.invalidate_value(target);
                    state.forward_vm_reg_store_to_load(inst, IrCmd::LOAD_INT64);
                }
            }
        }
        IrCmd::STORE_VECTOR => {
            let target = op_a(inst);
            if target.kind() == IrOpKind::VmReg {
                state.invalidate_value(target);

                let reg = crate::functions::vm_reg_op::vm_reg_op(target) as usize;
                let captured_regs = unsafe { &(*state.function).cfg.captured.regs };
                if (captured_regs[reg / 64] & (1u64 << (reg % 64))) == 0 {
                    let key = state.versioned_vm_reg_load_ir_cmd_ir_op(IrCmd::LOAD_FLOAT, target);
                    *state.value_map.get_or_insert(key) = index;
                }
            }
        }
        IrCmd::STORE_TVALUE => {
            let target = op_a(inst);
            if target.kind() == IrOpKind::VmReg || target.kind() == IrOpKind::Inst {
                let stored = op_b(inst.clone());

                if target.kind() == IrOpKind::VmReg {
                    if stored.kind() == IrOpKind::Inst {
                        if let Some(prev_idx) =
                            state.get_previous_versioned_load_index(IrCmd::LOAD_TVALUE, target)
                        {
                            if unsafe { *prev_idx } == stored.index() {
                                crate::functions::kill_ir_utils::kill_ir_function_ir_inst(
                                    function, inst,
                                );
                                return;
                            }
                        }
                    }

                    state.invalidate_ir_op(target);
                }

                let mut tag = state.try_get_tag(stored);
                if tag == 0xff {
                    tag = try_get_operand_tag(function, stored).unwrap_or(0xff);
                }

                let mut value = state.try_get_value(stored);

                if target.kind() == IrOpKind::VmReg {
                    if tag != 0xff {
                        state.save_tag(target, tag);
                    }

                    if value.kind() != IrOpKind::None {
                        state.save_value(target, value);
                    }
                }

                if target.kind() == IrOpKind::Inst {
                    let target_inst = function.instructions[target.index() as usize].clone();
                    state.invalidate_table_store_location(target_inst, OPT_OP_C(inst.clone()), tag);
                }

                let mut active_load_cmd = IrCmd::NOP;
                let mut active_load_value = !0u32;

                if tag != 0xff
                    && value.kind() == IrOpKind::None
                    && state.try_get_reg_link(stored).is_some()
                {
                    let arg_ptr = function.as_inst_op(stored);
                    if !arg_ptr.is_null() {
                        let mut arg = unsafe { (*arg_ptr).clone() };
                        let arg_source = op_a(&mut arg);
                        if arg.cmd == IrCmd::LOAD_TVALUE && arg_source.kind() == IrOpKind::VmReg {
                            let (cmd, idx) =
                                state.get_previous_versioned_load_for_tag(tag, arg_source);
                            if idx != !0u32 {
                                active_load_cmd = cmd;
                                active_load_value = idx;
                                value = crate::records::ir_op::IrOp::ir_op_ir_op_kind_u32(
                                    IrOpKind::Inst,
                                    idx,
                                );
                            }
                        }
                    }
                }

                let can_split_tvalue_store = if tag == lua_Type::LUA_TBOOLEAN as u8 {
                    value.kind() == IrOpKind::Inst || function.as_int_op(value).is_some()
                } else if tag == lua_Type::LUA_TNUMBER as u8 {
                    value.kind() == IrOpKind::Inst || function.as_double_op(value).is_some()
                } else if tag == lua_Type::LUA_TINTEGER as u8 {
                    value.kind() == IrOpKind::Inst || function.as_int_64_op(value).is_some()
                } else {
                    tag != 0xff && is_gco(tag) && value.kind() == IrOpKind::Inst
                };

                if can_split_tvalue_store {
                    let tag_op = build.const_tag(tag);
                    let mut ops = crate::type_aliases::ir_ops::IrOps::new();
                    ops.push(target);
                    ops.push(tag_op);
                    ops.push(value);
                    if HAS_OP_C!(inst) {
                        ops.push(op_c(inst.clone()));
                    }

                    let replacement = IrInst {
                        cmd: IrCmd::STORE_SPLIT_TVALUE,
                        ops,
                        ..IrInst::default()
                    };
                    let replacement_offset = OPT_OP_D(replacement.clone());

                    replace_ir_function_ir_block_u32_ir_inst(function, block, index, replacement);

                    if target.kind() == IrOpKind::VmReg && active_load_value != !0u32 {
                        let reg = crate::functions::vm_reg_op::vm_reg_op(target) as usize;
                        let versioned_reg = crate::records::ir_op::IrOp::ir_op_ir_op_kind_u32(
                            IrOpKind::VmReg,
                            (reg as u32) | (state.regs[reg].version << 8),
                        );
                        let mut ops = crate::type_aliases::ir_ops::IrOps::new();
                        ops.push(versioned_reg);
                        let key = IrInst {
                            cmd: active_load_cmd,
                            ops,
                            ..IrInst::default()
                        };
                        *state.value_map.get_or_insert(key) = active_load_value;
                    }

                    if target.kind() == IrOpKind::Inst {
                        let target_ptr =
                            &mut function.instructions[target.index() as usize] as *mut IrInst;
                        unsafe {
                            state.forward_table_store_to_load(
                                &mut *target_ptr,
                                replacement_offset,
                                index,
                            );
                        }
                    }
                } else if target.kind() == IrOpKind::VmReg {
                    state.forward_vm_reg_store_to_load(inst, IrCmd::LOAD_TVALUE);
                } else if luaur_common::FFlag::LuauCodegenExtraTableOpts.get()
                    && target.kind() == IrOpKind::Inst
                {
                    let offset = OPT_OP_C(inst.clone());
                    let target_ptr =
                        &mut function.instructions[target.index() as usize] as *mut IrInst;
                    unsafe {
                        state.forward_table_store_to_load(&mut *target_ptr, offset, index);
                    }
                }
            }
        }
        IrCmd::STORE_SPLIT_TVALUE => {
            let target = op_a(inst);
            if target.kind() == IrOpKind::VmReg {
                state.invalidate_ir_op(target);

                let tag = function.tag_op(op_b(inst.clone()));
                state.save_tag(target, tag);

                let value = op_c(inst.clone());
                if value.kind() == IrOpKind::Constant {
                    state.save_value(target, value);
                }
            } else if target.kind() == IrOpKind::Inst {
                let tag = function.tag_op(op_b(inst.clone()));
                let offset = OPT_OP_D(inst.clone());
                let target_inst = function.instructions[target.index() as usize].clone();
                state.invalidate_table_store_location(target_inst, offset, tag);

                let target_ptr = &mut function.instructions[target.index() as usize] as *mut IrInst;
                unsafe {
                    state.forward_table_store_to_load(&mut *target_ptr, offset, index);
                }
            }
        }
        IrCmd::GET_UPVALUE => {
            state.substitute_or_record_vm_upvalue_load(inst);
        }
        IrCmd::SET_UPVALUE => {
            state.forward_vm_upvalue_store_to_load(inst);

            let source = op_b(inst.clone());
            let tag = state.try_get_tag(source);
            if tag != 0xff {
                let tag_op = build.const_tag(tag);
                replace_ir_function_ir_op_ir_op(function, &mut inst.ops[2], tag_op);
            }
        }
        IrCmd::INT64_TO_NUM | IrCmd::INT_TO_NUM => {
            state.substitute_or_record(inst, index);
        }
        IrCmd::ADD_NUM | IrCmd::SUB_NUM => {
            let rhs = op_b(inst.clone());
            let rhs = if rhs.kind() == IrOpKind::Constant {
                rhs
            } else {
                state.try_get_value(rhs)
            };

            if let Some(k) = function.as_double_op(rhs) {
                if k == 0.0 && k.is_sign_negative() == (inst.cmd == IrCmd::ADD_NUM) {
                    let lhs = op_a(inst);
                    substitute(function, inst, lhs);
                } else {
                    state.substitute_or_record(inst, index);
                }
            } else {
                state.substitute_or_record(inst, index);
            }
        }
        IrCmd::MUL_NUM => {
            let rhs = op_b(inst.clone());
            let rhs = if rhs.kind() == IrOpKind::Constant {
                rhs
            } else {
                state.try_get_value(rhs)
            };

            if let Some(k) = function.as_double_op(rhs) {
                if k == 1.0 {
                    let lhs = op_a(inst);
                    substitute(function, inst, lhs);
                } else if k == 2.0 {
                    let lhs = op_a(inst);
                    let mut ops = crate::type_aliases::ir_ops::IrOps::new();
                    ops.push(lhs);
                    ops.push(lhs);
                    replace_ir_function_ir_block_u32_ir_inst(
                        function,
                        block,
                        index,
                        IrInst {
                            cmd: IrCmd::ADD_NUM,
                            ops,
                            ..IrInst::default()
                        },
                    );
                } else if k == -1.0 {
                    let lhs = op_a(inst);
                    let mut ops = crate::type_aliases::ir_ops::IrOps::new();
                    ops.push(lhs);
                    replace_ir_function_ir_block_u32_ir_inst(
                        function,
                        block,
                        index,
                        IrInst {
                            cmd: IrCmd::UNM_NUM,
                            ops,
                            ..IrInst::default()
                        },
                    );
                } else {
                    state.substitute_or_record(inst, index);
                }
            } else {
                state.substitute_or_record(inst, index);
            }
        }
        IrCmd::DIV_NUM => {
            let rhs = op_b(inst.clone());
            let rhs = if rhs.kind() == IrOpKind::Constant {
                rhs
            } else {
                state.try_get_value(rhs)
            };

            if let Some(k) = function.as_double_op(rhs) {
                if k == 1.0 {
                    let lhs = op_a(inst);
                    substitute(function, inst, lhs);
                } else if k == -1.0 {
                    let lhs = op_a(inst);
                    let mut ops = crate::type_aliases::ir_ops::IrOps::new();
                    ops.push(lhs);
                    replace_ir_function_ir_block_u32_ir_inst(
                        function,
                        block,
                        index,
                        IrInst {
                            cmd: IrCmd::UNM_NUM,
                            ops,
                            ..IrInst::default()
                        },
                    );
                } else {
                    let exp = k.log2();
                    if k > 0.0
                        && k.is_finite()
                        && exp.fract() == 0.0
                        && (-1000.0..=1000.0).contains(&exp)
                    {
                        let lhs = op_a(inst);
                        let reciprocal = build.const_double(1.0 / k);
                        let mut ops = crate::type_aliases::ir_ops::IrOps::new();
                        ops.push(lhs);
                        ops.push(reciprocal);
                        replace_ir_function_ir_block_u32_ir_inst(
                            function,
                            block,
                            index,
                            IrInst {
                                cmd: IrCmd::MUL_NUM,
                                ops,
                                ..IrInst::default()
                            },
                        );
                    } else {
                        state.substitute_or_record(inst, index);
                    }
                }
            } else {
                state.substitute_or_record(inst, index);
            }
        }
        IrCmd::ADD_FLOAT | IrCmd::SUB_FLOAT => {
            let rhs = op_b(inst.clone());
            let rhs = if rhs.kind() == IrOpKind::Constant {
                rhs
            } else {
                state.try_get_value(rhs)
            };

            if let Some(k) = function.as_double_op(rhs) {
                let kf = k as f32;
                if kf == 0.0 && kf.is_sign_negative() == (inst.cmd == IrCmd::ADD_FLOAT) {
                    let lhs = op_a(inst);
                    substitute(function, inst, lhs);
                } else {
                    state.substitute_or_record(inst, index);
                }
            } else {
                state.substitute_or_record(inst, index);
            }
        }
        IrCmd::MUL_FLOAT => {
            let rhs = op_b(inst.clone());
            let rhs = if rhs.kind() == IrOpKind::Constant {
                rhs
            } else {
                state.try_get_value(rhs)
            };

            if let Some(k) = function.as_double_op(rhs) {
                let kf = k as f32;
                if kf == 1.0 {
                    let lhs = op_a(inst);
                    substitute(function, inst, lhs);
                } else if kf == 2.0 {
                    let lhs = op_a(inst);
                    let mut ops = IrOps::new();
                    ops.push(lhs);
                    ops.push(lhs);
                    replace_ir_function_ir_block_u32_ir_inst(
                        function,
                        block,
                        index,
                        IrInst {
                            cmd: IrCmd::ADD_FLOAT,
                            ops,
                            ..IrInst::default()
                        },
                    );
                } else if kf == -1.0 {
                    let lhs = op_a(inst);
                    let mut ops = IrOps::new();
                    ops.push(lhs);
                    replace_ir_function_ir_block_u32_ir_inst(
                        function,
                        block,
                        index,
                        IrInst {
                            cmd: IrCmd::UNM_FLOAT,
                            ops,
                            ..IrInst::default()
                        },
                    );
                } else {
                    state.substitute_or_record(inst, index);
                }
            } else {
                state.substitute_or_record(inst, index);
            }
        }
        IrCmd::DIV_FLOAT => {
            let rhs = op_b(inst.clone());
            let rhs = if rhs.kind() == IrOpKind::Constant {
                rhs
            } else {
                state.try_get_value(rhs)
            };

            if let Some(k) = function.as_double_op(rhs) {
                let kf = k as f32;
                if kf == 1.0 {
                    let lhs = op_a(inst);
                    substitute(function, inst, lhs);
                } else if kf == -1.0 {
                    let lhs = op_a(inst);
                    let mut ops = IrOps::new();
                    ops.push(lhs);
                    replace_ir_function_ir_block_u32_ir_inst(
                        function,
                        block,
                        index,
                        IrInst {
                            cmd: IrCmd::UNM_FLOAT,
                            ops,
                            ..IrInst::default()
                        },
                    );
                } else {
                    let exp = kf.log2();
                    if kf > 0.0
                        && kf.is_finite()
                        && exp.fract() == 0.0
                        && (-1000.0..=1000.0).contains(&exp)
                    {
                        let lhs = op_a(inst);
                        let reciprocal = build.const_double((1.0f32 / kf) as f64);
                        let mut ops = IrOps::new();
                        ops.push(lhs);
                        ops.push(reciprocal);
                        replace_ir_function_ir_block_u32_ir_inst(
                            function,
                            block,
                            index,
                            IrInst {
                                cmd: IrCmd::MUL_FLOAT,
                                ops,
                                ..IrInst::default()
                            },
                        );
                    } else {
                        state.substitute_or_record(inst, index);
                    }
                }
            } else {
                state.substitute_or_record(inst, index);
            }
        }
        IrCmd::MIN_FLOAT
        | IrCmd::MAX_FLOAT
        | IrCmd::UNM_FLOAT
        | IrCmd::FLOOR_FLOAT
        | IrCmd::CEIL_FLOAT
        | IrCmd::SQRT_FLOAT
        | IrCmd::ABS_FLOAT
        | IrCmd::SIGN_FLOAT => {
            state.substitute_or_record(inst, index);
        }
        IrCmd::IDIV_NUM
        | IrCmd::MULADD_NUM
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
        | IrCmd::SELECT_INT64
        | IrCmd::SELECT_VEC
        | IrCmd::MULADD_VEC
        | IrCmd::EXTRACT_VEC
        | IrCmd::NOT_ANY => {
            state.substitute_or_record(inst, index);
        }
        IrCmd::SELECT_IF_TRUTHY => {
            let tag = state.try_get_tag(op_a(inst));

            if tag == lua_Type::LUA_TNIL as u8 {
                let replacement = op_c(inst.clone());
                substitute(function, inst, replacement);
            } else if tag != 0xff && tag != lua_Type::LUA_TBOOLEAN as u8 {
                let replacement = op_b(inst.clone());
                substitute(function, inst, replacement);
            }
        }
        IrCmd::UINT_TO_NUM | IrCmd::UINT_TO_FLOAT => {
            let src = function.as_inst_op(op_a(inst));
            if !src.is_null() {
                let mut src_clone = unsafe { (*src).clone() };
                if src_clone.cmd == IrCmd::TRUNCATE_UINT {
                    let src_source = op_a(&mut src_clone);
                    let src_of_src = function.as_inst_op(src_source);
                    if !src_of_src.is_null() && unsafe { (*src_of_src).cmd } == IrCmd::NUM_TO_UINT {
                        replace_ir_function_ir_op_ir_op(function, &mut inst.ops[0], src_source);
                    }
                }
            }

            state.substitute_or_record(inst, index);
        }
        IrCmd::NUM_TO_INT => {
            let src = function.as_inst_op(op_a(inst));
            if !src.is_null() {
                let src_clone = unsafe { (*src).clone() };

                if src_clone.cmd == IrCmd::INT_TO_NUM {
                    let mut src_clone = src_clone;
                    substitute(function, inst, op_a(&mut src_clone));
                    return;
                }

                if src_clone.cmd == IrCmd::ADD_NUM {
                    if let Some(arg) = function.as_double_op(op_b(src_clone.clone())) {
                        if arg == 0.0 {
                            let mut src_clone = src_clone.clone();
                            replace_ir_function_ir_op_ir_op(
                                function,
                                &mut inst.ops[0],
                                op_a(&mut src_clone),
                            );
                            state.substitute_or_record(inst, index);
                            return;
                        }
                    }

                    let mut src_clone_for_a = src_clone.clone();
                    let src_op_a = op_a(&mut src_clone_for_a);
                    if let Some(arg) = function.as_double_op(src_op_a) {
                        if arg == 0.0 {
                            replace_ir_function_ir_op_ir_op(
                                function,
                                &mut inst.ops[0],
                                op_b(src_clone),
                            );
                            state.substitute_or_record(inst, index);
                            return;
                        }
                    }
                }

                if src_clone.cmd == IrCmd::UINT_TO_NUM {
                    let mut src_clone = src_clone;
                    let src_source = op_a(&mut src_clone);
                    if src_source.kind() != IrOpKind::Constant {
                        substitute_with_truncated_uint(function, block, inst, src_source);
                        return;
                    }
                }
            }

            state.substitute_or_record(inst, index);
        }
        IrCmd::NUM_TO_UINT => {
            let src = function.as_inst_op(op_a(inst));
            if !src.is_null() {
                let src_clone = unsafe { (*src).clone() };

                if src_clone.cmd == IrCmd::UINT_TO_NUM {
                    let mut src_clone = src_clone;
                    substitute_with_truncated_uint(function, block, inst, op_a(&mut src_clone));
                    return;
                }

                if src_clone.cmd == IrCmd::INT_TO_NUM {
                    let mut src_clone = src_clone.clone();
                    let src_source = op_a(&mut src_clone);
                    if src_source.kind() != IrOpKind::Constant {
                        substitute(function, inst, src_source);
                        return;
                    }
                }

                if src_clone.cmd == IrCmd::ADD_NUM || src_clone.cmd == IrCmd::SUB_NUM {
                    let mut src_clone_a = src_clone.clone();
                    let src_a = op_a(&mut src_clone_a);
                    let src_b = op_b(src_clone.clone());
                    let add_src_1 = function.as_inst_op(src_a);
                    let add_num_1 = function.as_double_op(src_a);
                    let add_src_2 = function.as_inst_op(src_b);
                    let add_num_2 = function.as_double_op(src_b);

                    let replacement_cmd = if src_clone.cmd == IrCmd::ADD_NUM {
                        IrCmd::ADD_INT
                    } else {
                        IrCmd::SUB_INT
                    };

                    if !add_src_1.is_null()
                        && unsafe { (*add_src_1).cmd } == IrCmd::UINT_TO_NUM
                        && !add_src_2.is_null()
                        && unsafe { (*add_src_2).cmd } == IrCmd::UINT_TO_NUM
                    {
                        let mut add_src_1_clone = unsafe { (*add_src_1).clone() };
                        let mut add_src_2_clone = unsafe { (*add_src_2).clone() };
                        let mut ops = crate::type_aliases::ir_ops::IrOps::new();
                        ops.push(op_a(&mut add_src_1_clone));
                        ops.push(op_a(&mut add_src_2_clone));
                        replace_ir_function_ir_block_u32_ir_inst(
                            function,
                            block,
                            index,
                            IrInst {
                                cmd: replacement_cmd,
                                ops,
                                ..IrInst::default()
                            },
                        );
                        return;
                    } else if let Some(add_num_1) = add_num_1 {
                        if safe_integer_constant(add_num_1)
                            && !add_src_2.is_null()
                            && unsafe { (*add_src_2).cmd } == IrCmd::UINT_TO_NUM
                        {
                            let mut add_src_2_clone = unsafe { (*add_src_2).clone() };
                            let mut ops = crate::type_aliases::ir_ops::IrOps::new();
                            ops.push(build.const_int((add_num_1 as i64 as u32) as i32));
                            ops.push(op_a(&mut add_src_2_clone));
                            replace_ir_function_ir_block_u32_ir_inst(
                                function,
                                block,
                                index,
                                IrInst {
                                    cmd: replacement_cmd,
                                    ops,
                                    ..IrInst::default()
                                },
                            );
                            return;
                        }
                    } else if !add_src_1.is_null()
                        && unsafe { (*add_src_1).cmd } == IrCmd::UINT_TO_NUM
                    {
                        if let Some(add_num_2) = add_num_2 {
                            if safe_integer_constant(add_num_2) {
                                let mut add_src_1_clone = unsafe { (*add_src_1).clone() };
                                let mut ops = crate::type_aliases::ir_ops::IrOps::new();
                                ops.push(op_a(&mut add_src_1_clone));
                                ops.push(build.const_int((add_num_2 as i64 as u32) as i32));
                                replace_ir_function_ir_block_u32_ir_inst(
                                    function,
                                    block,
                                    index,
                                    IrInst {
                                        cmd: replacement_cmd,
                                        ops,
                                        ..IrInst::default()
                                    },
                                );
                                return;
                            }
                        }
                    }
                }
            }

            state.substitute_or_record(inst, index);
        }
        IrCmd::TRUNCATE_UINT => {
            let src = function.as_inst_op(op_a(inst));
            if !src.is_null() && !produces_dirty_high_register_bits(unsafe { (*src).cmd }) {
                let source = op_a(inst);
                substitute(function, inst, source);
            } else {
                state.substitute_or_record(inst, index);
            }
        }
        IrCmd::FLOAT_TO_NUM => {
            state.substitute_or_record(inst, index);
        }
        IrCmd::NUM_TO_FLOAT => {
            let src = function.as_inst_op(op_a(inst));
            if !src.is_null() {
                let src_clone = unsafe { (*src).clone() };
                if src_clone.cmd == IrCmd::FLOAT_TO_NUM {
                    let mut src_clone = src_clone;
                    substitute(function, inst, op_a(&mut src_clone));
                } else if src_clone.cmd == IrCmd::UINT_TO_NUM {
                    let mut src_clone = src_clone;
                    let mut ops = IrOps::new();
                    ops.push(op_a(&mut src_clone));
                    replace_ir_function_ir_block_u32_ir_inst(
                        function,
                        block,
                        index,
                        IrInst {
                            cmd: IrCmd::UINT_TO_FLOAT,
                            ops,
                            ..IrInst::default()
                        },
                    );
                } else {
                    state.substitute_or_record(inst, index);
                }
            } else {
                state.substitute_or_record(inst, index);
            }
        }
        IrCmd::JUMP_IF_TRUTHY => {
            let tag = state.try_get_tag(op_a(inst));
            if tag != 0xff {
                if tag == lua_Type::LUA_TNIL as u8 {
                    let mut ops = crate::type_aliases::ir_ops::IrOps::new();
                    ops.push(op_c(inst.clone()));
                    replace_ir_function_ir_block_u32_ir_inst(
                        function,
                        block,
                        index,
                        IrInst {
                            cmd: IrCmd::JUMP,
                            ops,
                            ..IrInst::default()
                        },
                    );
                } else if tag != lua_Type::LUA_TBOOLEAN as u8 {
                    let mut ops = crate::type_aliases::ir_ops::IrOps::new();
                    ops.push(op_b(inst.clone()));
                    replace_ir_function_ir_block_u32_ir_inst(
                        function,
                        block,
                        index,
                        IrInst {
                            cmd: IrCmd::JUMP,
                            ops,
                            ..IrInst::default()
                        },
                    );
                }
            }
        }
        IrCmd::JUMP_IF_FALSY => {
            let tag = state.try_get_tag(op_a(inst));
            if tag != 0xff {
                if tag == lua_Type::LUA_TNIL as u8 {
                    let mut ops = crate::type_aliases::ir_ops::IrOps::new();
                    ops.push(op_b(inst.clone()));
                    replace_ir_function_ir_block_u32_ir_inst(
                        function,
                        block,
                        index,
                        IrInst {
                            cmd: IrCmd::JUMP,
                            ops,
                            ..IrInst::default()
                        },
                    );
                } else if tag != lua_Type::LUA_TBOOLEAN as u8 {
                    let mut ops = crate::type_aliases::ir_ops::IrOps::new();
                    ops.push(op_c(inst.clone()));
                    replace_ir_function_ir_block_u32_ir_inst(
                        function,
                        block,
                        index,
                        IrInst {
                            cmd: IrCmd::JUMP,
                            ops,
                            ..IrInst::default()
                        },
                    );
                }
            }
        }
        IrCmd::CMP_ANY => {
            state.invalidate_user_call();
        }
        IrCmd::CMP_SPLIT_TVALUE => {
            let tag_a_op = op_a(inst);
            let tag_b_op = op_b(inst.clone());
            let tag_a = if tag_a_op.kind() == IrOpKind::Constant {
                function.tag_op(tag_a_op)
            } else {
                state.try_get_tag(tag_a_op)
            };
            let tag_b = if tag_b_op.kind() == IrOpKind::Constant {
                function.tag_op(tag_b_op)
            } else {
                state.try_get_tag(tag_b_op)
            };

            if tag_a == lua_Type::LUA_TSTRING as u8 && tag_b == lua_Type::LUA_TSTRING as u8 {
                if let Some((source, tag)) =
                    type_name_tag_comparison(function, op_c(inst.clone()), op_d(inst.clone()))
                {
                    let tag_op = build.const_tag(tag);
                    let replacement =
                        const_prop_make_inst(IrCmd::CMP_TAG, &[source, tag_op, op_e(inst.clone())]);

                    replace_ir_function_ir_block_u32_ir_inst(function, block, index, replacement);
                    fold_constants(build, function, block, index);
                    return;
                }
            }
        }
        IrCmd::JUMP_EQ_POINTER => {
            if let Some((source, tag)) =
                type_name_tag_comparison(function, op_a(inst), op_b(inst.clone()))
            {
                let tag_op = build.const_tag(tag);
                let replacement = const_prop_make_inst(
                    IrCmd::JUMP_EQ_TAG,
                    &[source, tag_op, op_c(inst.clone()), op_d(inst.clone())],
                );

                replace_ir_function_ir_block_u32_ir_inst(function, block, index, replacement);
                fold_constants(build, function, block, index);
                return;
            }
        }
        IrCmd::JUMP_EQ_TAG => {
            let a = op_a(inst);
            let b = op_b(inst.clone());
            let tag_a = if a.kind() == IrOpKind::Constant {
                function.tag_op(a)
            } else {
                state.try_get_tag(a)
            };
            let tag_b = if b.kind() == IrOpKind::Constant {
                function.tag_op(b)
            } else {
                state.try_get_tag(b)
            };

            if tag_a != 0xff && tag_b != 0xff {
                let target = if tag_a == tag_b {
                    op_c(inst.clone())
                } else {
                    op_d(inst.clone())
                };
                let mut ops = crate::type_aliases::ir_ops::IrOps::new();
                ops.push(target);
                replace_ir_function_ir_block_u32_ir_inst(
                    function,
                    block,
                    index,
                    IrInst {
                        cmd: IrCmd::JUMP,
                        ops,
                        ..IrInst::default()
                    },
                );
            } else if a == b {
                let mut ops = crate::type_aliases::ir_ops::IrOps::new();
                ops.push(op_c(inst.clone()));
                replace_ir_function_ir_block_u32_ir_inst(
                    function,
                    block,
                    index,
                    IrInst {
                        cmd: IrCmd::JUMP,
                        ops,
                        ..IrInst::default()
                    },
                );
            }
        }
        IrCmd::JUMP_CMP_INT => {
            let a = op_a(inst);
            let b = op_b(inst.clone());
            let value_a = function.as_int_op(if a.kind() == IrOpKind::Constant {
                a
            } else {
                state.try_get_value(a)
            });
            let value_b = function.as_int_op(if b.kind() == IrOpKind::Constant {
                b
            } else {
                state.try_get_value(b)
            });

            if let (Some(value_a), Some(value_b)) = (value_a, value_b) {
                let target = if compare_i32_i32_ir_condition(
                    value_a,
                    value_b,
                    condition_op(op_c(inst.clone())),
                ) {
                    op_d(inst.clone())
                } else {
                    op_e(inst.clone())
                };
                let mut ops = crate::type_aliases::ir_ops::IrOps::new();
                ops.push(target);
                replace_ir_function_ir_block_u32_ir_inst(
                    function,
                    block,
                    index,
                    IrInst {
                        cmd: IrCmd::JUMP,
                        ops,
                        ..IrInst::default()
                    },
                );
            }
        }
        IrCmd::JUMP_CMP_NUM => {
            let a = op_a(inst);
            let b = op_b(inst.clone());
            let value_a = function.as_double_op(if a.kind() == IrOpKind::Constant {
                a
            } else {
                state.try_get_value(a)
            });
            let value_b = function.as_double_op(if b.kind() == IrOpKind::Constant {
                b
            } else {
                state.try_get_value(b)
            });

            if let (Some(value_a), Some(value_b)) = (value_a, value_b) {
                let target = if compare_f64_f64_ir_condition(
                    value_a,
                    value_b,
                    condition_op(op_c(inst.clone())),
                ) {
                    op_d(inst.clone())
                } else {
                    op_e(inst.clone())
                };
                let mut ops = crate::type_aliases::ir_ops::IrOps::new();
                ops.push(target);
                replace_ir_function_ir_block_u32_ir_inst(
                    function,
                    block,
                    index,
                    IrInst {
                        cmd: IrCmd::JUMP,
                        ops,
                        ..IrInst::default()
                    },
                );
            }
        }
        IrCmd::CHECK_TAG => {
            let target = op_a(inst);
            let expected = function.tag_op(op_b(inst.clone()));
            let mut tag = state.try_get_tag(target);

            if tag == 0xff {
                let value = state.try_get_value(target);
                if value.kind() == IrOpKind::Constant {
                    let constant = function.const_op(value);
                    if constant.kind == IrConstKind::Double {
                        tag = lua_Type::LUA_TNUMBER as u8;
                    } else if constant.kind == IrConstKind::Int64 {
                        tag = lua_Type::LUA_TINTEGER as u8;
                    }
                }
            }

            if tag != 0xff {
                if tag == expected {
                    crate::functions::kill_ir_utils::kill_ir_function_ir_inst(function, inst);
                } else {
                    let mut ops = crate::type_aliases::ir_ops::IrOps::new();
                    ops.push(op_c(inst.clone()));
                    replace_ir_function_ir_block_u32_ir_inst(
                        function,
                        block,
                        index,
                        IrInst {
                            cmd: IrCmd::JUMP,
                            ops,
                            ..IrInst::default()
                        },
                    );
                }
            } else {
                let lhs = function.as_inst_op(target);
                if !lhs.is_null() {
                    let mut lhs_inst = unsafe { (*lhs).clone() };
                    let lhs_source = op_a(&mut lhs_inst);
                    if lhs_inst.cmd == IrCmd::LOAD_TAG && lhs_source.kind() == IrOpKind::VmReg {
                        if let Some(prev_idx) =
                            state.get_previous_versioned_load_index(IrCmd::LOAD_TVALUE, lhs_source)
                        {
                            state.inst_tag.try_insert(unsafe { *prev_idx }, expected);
                        }
                    }
                }

                state.update_tag(target, expected);
            }
        }
        IrCmd::NUM_TO_INT64 => {
            // INT64_TO_NUM followed by NUM_TO_INT64 of the same source is the identity.
            let src = function.as_inst_op(op_a(inst));
            if !src.is_null() {
                let src_clone = unsafe { (*src).clone() };
                let src_cmd = src_clone.cmd;
                if src_cmd == IrCmd::INT64_TO_NUM {
                    let src_op_a = op_a(&mut { src_clone });
                    substitute(function, inst, src_op_a);
                    return;
                }
                if src_cmd == IrCmd::ADD_NUM {
                    let src_op_b = op_b(src_clone.clone());
                    if let Some(arg) = function.as_double_op(src_op_b) {
                        if arg == 0.0 {
                            let src_op_a = op_a(&mut { src_clone.clone() });
                            inst.ops[0] = src_op_a;
                            state.substitute_or_record(inst, index);
                            return;
                        }
                    }
                    let src_op_a = op_a(&mut { src_clone.clone() });
                    if let Some(arg) = function.as_double_op(src_op_a) {
                        if arg == 0.0 {
                            let src_op_b = op_b(src_clone);
                            inst.ops[0] = src_op_b;
                            state.substitute_or_record(inst, index);
                            return;
                        }
                    }
                }
            }
            state.substitute_or_record(inst, index);
        }
        IrCmd::LOAD_ENV => {
            if luaur_common::FFlag::LuauCodegenExtraTableOpts.get() {
                if state.load_env_idx != crate::records::ir_data::k_invalid_inst_idx {
                    substitute(
                        function,
                        inst,
                        IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, state.load_env_idx),
                    );
                } else {
                    state.load_env_idx = index;
                }
            }
        }
        IrCmd::GET_SLOT_NODE_ADDR => {
            for i in 0..state.get_slot_node_cache.len() {
                let prev_idx = state.get_slot_node_cache[i].inst_idx;
                let mut prev = function.instructions[prev_idx as usize].clone();

                if op_a(&mut prev) == op_a(inst) && op_c(prev.clone()) == op_c(inst.clone()) {
                    let limit = luaur_common::FInt::LuauCodeGenLiveSlotReuseLimit.get();

                    if state.get_slot_node_cache.len() as i32 > limit {
                        let mut cache = state.get_slot_node_cache.clone();
                        if state.get_max_internal_overlap(&mut cache, i) > limit {
                            return;
                        }
                    }

                    state.get_slot_node_cache[i].finish_pos = state.inst_pos;

                    substitute(
                        function,
                        inst,
                        IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, prev_idx),
                    );
                    return;
                }
            }

            if (state.get_slot_node_cache.len() as i32)
                < luaur_common::FInt::LuauCodeGenReuseSlotLimit.get()
            {
                state.get_slot_node_cache.push(NumberedInstruction {
                    inst_idx: index,
                    start_pos: state.inst_pos,
                    finish_pos: state.inst_pos,
                });
            }
        }
        IrCmd::GET_ARR_ADDR => {
            for prev_idx in state.get_arr_addr_cache.iter().copied() {
                let mut prev = function.instructions[prev_idx as usize].clone();

                if op_a(&mut prev) == op_a(inst) && op_b(prev.clone()) == op_b(inst.clone()) {
                    substitute(
                        function,
                        inst,
                        IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, prev_idx),
                    );
                    return;
                }
            }

            if (state.get_arr_addr_cache.len() as i32)
                < luaur_common::FInt::LuauCodeGenReuseSlotLimit.get()
            {
                state.get_arr_addr_cache.push(index);
            }
        }
        IrCmd::ADD_INT
        | IrCmd::SUB_INT
        | IrCmd::ADD_INT64
        | IrCmd::SUB_INT64
        | IrCmd::MUL_INT64
        | IrCmd::DIV_INT64
        | IrCmd::IDIV_INT64
        | IrCmd::CHECK_DIV_INT64
        | IrCmd::UDIV_INT64
        | IrCmd::REM_INT64
        | IrCmd::UREM_INT64
        | IrCmd::MOD_INT64
        | IrCmd::SEXTI8_INT
        | IrCmd::SEXTI16_INT => {
            state.substitute_or_record(inst, index);
        }
        IrCmd::TRY_NUM_TO_INDEX => {
            for prev_idx in state.try_num_to_index_cache.iter().copied() {
                let mut prev = function.instructions[prev_idx as usize].clone();

                if op_a(&mut prev) == op_a(inst) {
                    substitute(
                        function,
                        inst,
                        IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, prev_idx),
                    );
                    return;
                }
            }

            if (state.try_num_to_index_cache.len() as i32)
                < luaur_common::FInt::LuauCodeGenReuseSlotLimit.get()
            {
                state.try_num_to_index_cache.push(index);
            }
        }
        IrCmd::CHECK_SLOT_MATCH => {
            for el in &mut state.check_slot_match_cache {
                let mut prev = function.instructions[el.pointer as usize].clone();

                if op_a(&mut prev) == op_a(inst) && op_b(prev.clone()) == op_b(inst.clone()) {
                    if let Some(info) = state.inst_tag.find(&op_a(inst).index()) {
                        if *info != lua_Type::LUA_TNIL as u8 {
                            el.knownToNotBeNil = true;
                        }
                    }

                    if el.knownToNotBeNil {
                        crate::functions::kill_ir_utils::kill_ir_function_ir_inst(function, inst);
                    } else {
                        let mut ops = crate::type_aliases::ir_ops::IrOps::new();
                        ops.push(op_a(inst));
                        ops.push(op_c(inst.clone()));
                        replace_ir_function_ir_block_u32_ir_inst(
                            function,
                            block,
                            index,
                            IrInst {
                                cmd: IrCmd::CHECK_NODE_VALUE,
                                ops,
                                ..IrInst::default()
                            },
                        );
                    }

                    el.knownToNotBeNil = true;
                    return;
                }
            }

            if (state.check_slot_match_cache.len() as i32)
                < luaur_common::FInt::LuauCodeGenReuseSlotLimit.get()
            {
                state.check_slot_match_cache.push(NodeSlotState {
                    pointer: index,
                    knownToNotBeNil: true,
                });
            }
        }
        IrCmd::CHECK_SAFE_ENV => {
            if state.in_safe_env {
                crate::functions::kill_ir_utils::kill_ir_function_ir_inst(function, inst);
            } else {
                state.in_safe_env = true;
            }
        }
        IrCmd::CHECK_READONLY => {
            let target = op_a(inst);
            if luaur_common::FFlag::LuauCodegenExtraTableOpts.get()
                && target.kind() == IrOpKind::Inst
            {
                let target_idx = target.index();
                if state.inst_not_readonly.find(&target_idx).is_some() {
                    crate::functions::kill_ir_utils::kill_ir_function_ir_inst(function, inst);
                    return;
                }
                state.inst_not_readonly.insert(target_idx);
            } else if !luaur_common::FFlag::LuauCodegenExtraTableOpts.get() {
                if let Some(info) = state.try_get_register_info(target) {
                    unsafe {
                        if (*info).known_not_readonly_deprecated {
                            crate::functions::kill_ir_utils::kill_ir_function_ir_inst(
                                function, inst,
                            );
                        } else {
                            (*info).known_not_readonly_deprecated = true;
                        }
                    }
                }
            }
        }
        IrCmd::CHECK_NO_METATABLE => {
            let target = op_a(inst);
            if luaur_common::FFlag::LuauCodegenExtraTableOpts.get()
                && target.kind() == IrOpKind::Inst
            {
                let target_idx = target.index();
                if state.inst_no_metatable.find(&target_idx).is_some() {
                    crate::functions::kill_ir_utils::kill_ir_function_ir_inst(function, inst);
                    return;
                }
                state.inst_no_metatable.insert(target_idx);
            } else if !luaur_common::FFlag::LuauCodegenExtraTableOpts.get() {
                if let Some(info) = state.try_get_register_info(target) {
                    unsafe {
                        if (*info).known_no_metatable_deprecated {
                            crate::functions::kill_ir_utils::kill_ir_function_ir_inst(
                                function, inst,
                            );
                        } else {
                            (*info).known_no_metatable_deprecated = true;
                        }
                    }
                }
            }
        }
        IrCmd::BUFFER_READI8 => {
            state.substitute_or_record_buffer_load(block, index, inst, 1);
        }
        IrCmd::BUFFER_READU8 => {
            state.substitute_or_record_buffer_load(block, index, inst, 1);
        }
        IrCmd::BUFFER_WRITEI8 => {
            let src = function.as_inst_op(op_c(inst.clone()));
            if !src.is_null() {
                let src_inst = unsafe { (*src).clone() };
                let int_src_b = function.as_int_op(OPT_OP_B(src_inst.clone()));

                if src_inst.cmd == IrCmd::SEXTI8_INT {
                    let replacement = op_a(&mut src_inst.clone());
                    replace_ir_function_ir_op_ir_op(function, &mut inst.ops[2], replacement);
                } else if src_inst.cmd == IrCmd::BITAND_UINT && int_src_b == Some(0xff) {
                    let replacement = op_a(&mut src_inst.clone());
                    replace_ir_function_ir_op_ir_op(function, &mut inst.ops[2], replacement);
                }
            }

            state.forward_buffer_store_to_load(inst, IrCmd::BUFFER_READI8, 1);
        }
        IrCmd::BUFFER_READI16 => {
            state.substitute_or_record_buffer_load(block, index, inst, 2);
        }
        IrCmd::BUFFER_READU16 => {
            state.substitute_or_record_buffer_load(block, index, inst, 2);
        }
        IrCmd::BUFFER_WRITEI16 => {
            let src = function.as_inst_op(op_c(inst.clone()));
            if !src.is_null() {
                let src_inst = unsafe { (*src).clone() };
                let int_src_b = function.as_int_op(OPT_OP_B(src_inst.clone()));

                if src_inst.cmd == IrCmd::SEXTI16_INT {
                    let replacement = op_a(&mut src_inst.clone());
                    replace_ir_function_ir_op_ir_op(function, &mut inst.ops[2], replacement);
                } else if src_inst.cmd == IrCmd::BITAND_UINT && int_src_b == Some(0xffff) {
                    let replacement = op_a(&mut src_inst.clone());
                    replace_ir_function_ir_op_ir_op(function, &mut inst.ops[2], replacement);
                }
            }

            state.forward_buffer_store_to_load(inst, IrCmd::BUFFER_READI16, 2);
        }
        IrCmd::BUFFER_READI32 => {
            state.substitute_or_record_buffer_load(block, index, inst, 4);
        }
        IrCmd::BUFFER_WRITEI32 => {
            let src = function.as_inst_op(op_c(inst.clone()));
            if !src.is_null() {
                let src_inst = unsafe { (*src).clone() };

                if src_inst.cmd == IrCmd::TRUNCATE_UINT {
                    let replacement = op_a(&mut src_inst.clone());
                    replace_ir_function_ir_op_ir_op(function, &mut inst.ops[2], replacement);
                }
            }

            state.forward_buffer_store_to_load(inst, IrCmd::BUFFER_READI32, 4);
        }
        IrCmd::BUFFER_READF32 => {
            state.substitute_or_record_buffer_load(block, index, inst, 4);
        }
        IrCmd::BUFFER_WRITEF32 => {
            state.forward_buffer_store_to_load(inst, IrCmd::BUFFER_READF32, 4);
        }
        IrCmd::BUFFER_READF64 => {
            state.substitute_or_record_buffer_load(block, index, inst, 8);
        }
        IrCmd::BUFFER_WRITEF64 => {
            state.forward_buffer_store_to_load(inst, IrCmd::BUFFER_READF64, 8);
        }
        IrCmd::BUFFER_READI64 => {
            state.substitute_or_record_buffer_load(block, index, inst, 8);
        }
        IrCmd::BUFFER_WRITEI64 => {
            state.forward_buffer_store_to_load(inst, IrCmd::BUFFER_READI64, 8);
        }
        IrCmd::CHECK_GC => {
            if state.checked_gc {
                crate::functions::kill_ir_utils::kill_ir_function_ir_inst(function, inst);
            } else {
                state.checked_gc = true;
                state.invalidate_heap_table_data();
            }
        }
        IrCmd::BARRIER_OBJ | IrCmd::BARRIER_TABLE_FORWARD => {
            let value = op_b(inst.clone());
            if value.kind() == IrOpKind::VmReg {
                let tag = state.try_get_tag(value);
                if tag != 0xff && !is_gco(tag) {
                    crate::functions::kill_ir_utils::kill_ir_function_ir_inst(function, inst);
                }
            }
        }
        IrCmd::NEW_TABLE => {
            if luaur_common::FFlag::LuauCodegenExtraTableOpts.get() {
                let array_size = function.uint_op(op_a(inst)) as i32;
                state.inst_not_readonly.insert(index);
                state.inst_no_metatable.insert(index);
                state.inst_array_size.try_insert(index, array_size);
            }
        }
        IrCmd::CHECK_ARRAY_SIZE => {
            let target = op_a(inst);
            let boundary = op_b(inst.clone());
            let boundary_value = if boundary.kind() == IrOpKind::Constant {
                function.as_int_op(boundary)
            } else {
                function.as_int_op(state.try_get_value(boundary))
            };

            if let Some(array_index) = boundary_value {
                if array_index < 0 {
                    let mut ops = crate::type_aliases::ir_ops::IrOps::new();
                    ops.push(op_c(inst.clone()));
                    replace_ir_function_ir_block_u32_ir_inst(
                        function,
                        block,
                        index,
                        IrInst {
                            cmd: IrCmd::JUMP,
                            ops,
                            ..IrInst::default()
                        },
                    );
                    return;
                }

                if luaur_common::FFlag::LuauCodegenExtraTableOpts.get()
                    && target.kind() == IrOpKind::Inst
                {
                    if let Some(known_array_size) = state.inst_array_size.find(&target.index()) {
                        if *known_array_size >= 0 {
                            if (array_index as u32) < (*known_array_size as u32) {
                                crate::functions::kill_ir_utils::kill_ir_function_ir_inst(
                                    function, inst,
                                );
                            } else {
                                let mut ops = crate::type_aliases::ir_ops::IrOps::new();
                                ops.push(op_c(inst.clone()));
                                replace_ir_function_ir_block_u32_ir_inst(
                                    function,
                                    block,
                                    index,
                                    IrInst {
                                        cmd: IrCmd::JUMP,
                                        ops,
                                        ..IrInst::default()
                                    },
                                );
                            }
                            return;
                        }
                    }
                }

                if let Some(info) = state.try_get_register_info(target) {
                    unsafe {
                        if (*info).known_table_array_size_deprecated >= 0 {
                            if (array_index as u32)
                                < ((*info).known_table_array_size_deprecated as u32)
                            {
                                crate::functions::kill_ir_utils::kill_ir_function_ir_inst(
                                    function, inst,
                                );
                            } else {
                                let mut ops = crate::type_aliases::ir_ops::IrOps::new();
                                ops.push(op_c(inst.clone()));
                                replace_ir_function_ir_block_u32_ir_inst(
                                    function,
                                    block,
                                    index,
                                    IrInst {
                                        cmd: IrCmd::JUMP,
                                        ops,
                                        ..IrInst::default()
                                    },
                                );
                            }
                            return;
                        }
                    }
                }
            }

            for prev_idx in state.check_array_size_cache.iter().copied() {
                let mut prev = function.instructions[prev_idx as usize].clone();

                if op_a(&mut prev) != op_a(inst) {
                    continue;
                }

                let prev_boundary = op_b(prev.clone());
                let boundary = op_b(inst.clone());
                let mut same_boundary = prev_boundary == boundary;

                if !same_boundary
                    && boundary.kind() == IrOpKind::Constant
                    && prev_boundary.kind() == IrOpKind::Constant
                    && (function.int_op(boundary) as u32) < (function.int_op(prev_boundary) as u32)
                {
                    same_boundary = true;
                }

                if same_boundary {
                    crate::functions::kill_ir_utils::kill_ir_function_ir_inst(function, inst);
                    return;
                }
            }

            if (state.check_array_size_cache.len() as i32)
                < luaur_common::FInt::LuauCodeGenReuseSlotLimit.get()
            {
                state.check_array_size_cache.push(index);
            }
        }
        IrCmd::CHECK_BUFFER_LEN => {
            let buffer_offset_op = op_b(inst.clone());
            let buffer_offset = if buffer_offset_op.kind() == IrOpKind::Constant {
                function.as_int_op(buffer_offset_op)
            } else {
                function.as_int_op(state.try_get_value(buffer_offset_op))
            };

            let min_offset = function.int_op(op_c(inst.clone()));
            let max_offset = function.int_op(op_d(inst.clone()));
            crate::macros::codegen_assert::CODEGEN_ASSERT!(min_offset < max_offset);
            let access_size = max_offset - min_offset;
            crate::macros::codegen_assert::CODEGEN_ASSERT!(access_size > 0);

            if let Some(buffer_offset) = buffer_offset {
                if buffer_offset < 0
                    || (buffer_offset as u32).wrapping_add(access_size as u32) >= i32::MAX as u32
                {
                    let mut ops = crate::type_aliases::ir_ops::IrOps::new();
                    ops.push(op_f(inst.clone()));
                    replace_ir_function_ir_block_u32_ir_inst(
                        function,
                        block,
                        index,
                        IrInst {
                            cmd: IrCmd::JUMP,
                            ops,
                            ..IrInst::default()
                        },
                    );
                    return;
                }
            }

            for prev_idx in state.check_buffer_len_cache.clone() {
                let prev_ptr = &mut function.instructions[prev_idx as usize] as *mut IrInst;
                let prev = unsafe { &mut *prev_ptr };

                if prev.cmd != IrCmd::CHECK_BUFFER_LEN {
                    continue;
                }

                if op_a(prev) == op_a(inst)
                    && op_b(prev.clone()) == op_b(inst.clone())
                    && op_c(prev.clone()) == op_c(inst.clone())
                    && op_d(prev.clone()) == op_d(inst.clone())
                {
                    if luaur_common::FFlag::DebugLuauAbortingChecks.get() {
                        let replacement = build.undef();
                        replace_ir_function_ir_op_ir_op(function, &mut inst.ops[5], replacement);
                    } else {
                        crate::functions::kill_ir_utils::kill_ir_function_ir_inst(function, inst);
                    }
                    return;
                }

                if op_a(prev) == op_a(inst)
                    && op_b(inst.clone()).kind() == IrOpKind::Constant
                    && op_b(prev.clone()).kind() == IrOpKind::Constant
                {
                    let curr_bound = function.int_op(op_b(inst.clone()));
                    let prev_bound = function.int_op(op_b(prev.clone()));
                    crate::macros::codegen_assert::CODEGEN_ASSERT!(curr_bound >= 0);
                    crate::macros::codegen_assert::CODEGEN_ASSERT!(prev_bound >= 0);

                    let extra_offset = curr_bound - prev_bound;
                    if state.try_merge_and_kill_buffer_length_check(
                        build,
                        block,
                        inst,
                        prev,
                        extra_offset,
                    ) {
                        return;
                    }

                    continue;
                }

                if state.try_merge_buffer_range_check(build, block, inst, prev) {
                    return;
                }
            }

            if (state.check_buffer_len_cache.len() as i32)
                < luaur_common::FInt::LuauCodeGenReuseSlotLimit.get()
            {
                state.check_buffer_len_cache.push(index);
            }
        }
        IrCmd::ADD_VEC
        | IrCmd::SUB_VEC
        | IrCmd::MUL_VEC
        | IrCmd::DIV_VEC
        | IrCmd::IDIV_VEC
        | IrCmd::DOT_VEC
        | IrCmd::MIN_VEC
        | IrCmd::MAX_VEC => {
            let a = op_a(inst);
            let a_ptr = function.as_inst_op(a);
            if !a_ptr.is_null() {
                let a_inst = unsafe { (*a_ptr).clone() };
                if a_inst.cmd == IrCmd::TAG_VECTOR {
                    let replacement = op_a(&mut a_inst.clone());
                    replace_ir_function_ir_op_ir_op(function, &mut inst.ops[0], replacement);
                }
            }

            let b = op_b(inst.clone());
            let b_ptr = function.as_inst_op(b);
            if !b_ptr.is_null() {
                let b_inst = unsafe { (*b_ptr).clone() };
                if b_inst.cmd == IrCmd::TAG_VECTOR {
                    let replacement = op_a(&mut b_inst.clone());
                    replace_ir_function_ir_op_ir_op(function, &mut inst.ops[1], replacement);
                }
            }

            state.substitute_or_record(inst, index);
        }
        IrCmd::UNM_VEC | IrCmd::FLOOR_VEC | IrCmd::CEIL_VEC | IrCmd::ABS_VEC => {
            let a = op_a(inst);
            let a_ptr = function.as_inst_op(a);
            if !a_ptr.is_null() {
                let a_inst = unsafe { (*a_ptr).clone() };
                if a_inst.cmd == IrCmd::TAG_VECTOR {
                    let replacement = op_a(&mut a_inst.clone());
                    replace_ir_function_ir_op_ir_op(function, &mut inst.ops[0], replacement);
                }
            }

            state.substitute_or_record(inst, index);
        }
        IrCmd::FLOAT_TO_VEC | IrCmd::TAG_VECTOR => {
            state.substitute_or_record(inst, index);
        }
        IrCmd::INVOKE_LIBM => {
            state.substitute_or_record(inst, index);
        }
        IrCmd::DO_ARITH => {
            let target = op_a(inst);
            state.invalidate_ir_op(target);
            state.invalidate_user_call();
        }
        IrCmd::DO_LEN => {
            let target = op_a(inst);
            state.invalidate_ir_op(target);
            state.invalidate_user_call();
            state.save_tag(target, lua_Type::LUA_TNUMBER as u8);
        }
        IrCmd::GET_TABLE => {
            let target = op_a(inst);
            state.invalidate_ir_op(target);
            state.invalidate_user_call();
        }
        IrCmd::SET_TABLE => {
            state.invalidate_user_call();
        }
        IrCmd::GET_CACHED_IMPORT => {
            let target = op_a(inst);
            state.invalidate_ir_op(target);

            if state.in_safe_env {
                state.invalidate_value_propagation();
            } else {
                state.invalidate_user_call();
            }
        }
        IrCmd::SETLIST => {
            if luaur_common::FFlag::LuauCodegenExtraTableOpts.get() {
                if let Some(load_idx) =
                    state.get_previous_versioned_load_index(IrCmd::LOAD_POINTER, op_b(inst.clone()))
                {
                    let load_idx = unsafe { *load_idx };
                    if let Some(known_array_size) = state.inst_array_size.find(&load_idx) {
                        if *known_array_size >= 0 {
                            let replacement = build.const_uint(*known_array_size as u32);
                            replace_ir_function_ir_op_ir_op(
                                function,
                                &mut inst.ops[5],
                                replacement,
                            );
                        }
                    }
                }
            } else if let Some(info) = state.try_get_register_info(op_b(inst.clone())) {
                unsafe {
                    if (*info).known_table_array_size_deprecated >= 0 {
                        let replacement =
                            build.const_uint((*info).known_table_array_size_deprecated as u32);
                        replace_ir_function_ir_op_ir_op(function, &mut inst.ops[5], replacement);
                    }
                }
            }

            state.invalidate_value_propagation();
            state.invalidate_heap_table_data();
            state.invalidate_heap_buffer_data();
        }
        IrCmd::TABLE_SETNUM => {
            state.invalidate_table_array_size();
        }
        IrCmd::CONCAT => {
            let first_reg = crate::functions::vm_reg_op::vm_reg_op(op_a(inst));
            let count = function.uint_op(op_b(inst.clone())) as i32;
            state.invalidate_register_range(first_reg, count);
            state.invalidate_user_call();
        }
        IrCmd::FALLBACK_GETVARARGS => {
            let first_reg = crate::functions::vm_reg_op::vm_reg_op(op_b(inst.clone()));
            let count = function.int_op(op_c(inst.clone()));
            state.invalidate_register_range(first_reg, count);
        }
        IrCmd::FASTCALL => {
            let bfid = unsafe {
                core::mem::transmute::<u8, LuauBuiltinFunction>(function.uint_op(op_a(inst)) as u8)
            };
            let first_return_reg = crate::functions::vm_reg_op::vm_reg_op(op_b(inst.clone()));
            let nresults = function.int_op(op_d(inst.clone()));

            handle_builtin_effects(state, bfid, first_return_reg as u32, nresults);

            match bfid {
                LuauBuiltinFunction::LBF_MATH_MODF | LuauBuiltinFunction::LBF_MATH_FREXP => {
                    let target =
                        IrOp::ir_op_ir_op_kind_u32(IrOpKind::VmReg, first_return_reg as u32);
                    state.update_tag(target, lua_Type::LUA_TNUMBER as u8);

                    if nresults > 1 {
                        let target = IrOp::ir_op_ir_op_kind_u32(
                            IrOpKind::VmReg,
                            (first_return_reg + 1) as u32,
                        );
                        state.update_tag(target, lua_Type::LUA_TNUMBER as u8);
                    }
                }
                _ => {}
            }
        }
        IrCmd::INVOKE_FASTCALL => {
            let bfid = unsafe {
                core::mem::transmute::<u8, LuauBuiltinFunction>(function.uint_op(op_a(inst)) as u8)
            };
            let first_return_reg =
                crate::functions::vm_reg_op::vm_reg_op(op_b(inst.clone())) as u32;
            let nresults = function.int_op(op_g(inst.clone()));
            handle_builtin_effects(state, bfid, first_return_reg, nresults);
        }
        IrCmd::CALL => {
            let first_reg = crate::functions::vm_reg_op::vm_reg_op(op_a(inst));
            state.invalidate_registers_from(first_reg);
            state.invalidate_user_call();
        }
        IrCmd::FALLBACK_GETGLOBAL => {
            state.invalidate_ir_op(op_b(inst.clone()));
            state.invalidate_user_call();
        }
        IrCmd::FALLBACK_SETGLOBAL | IrCmd::FALLBACK_SETTABLEKS => {
            state.invalidate_user_call();
        }
        IrCmd::FALLBACK_GETTABLEKS => {
            state.invalidate_ir_op(op_b(inst.clone()));
            state.invalidate_user_call();
        }
        IrCmd::FALLBACK_NAMECALL => {
            let target = op_b(inst.clone());
            state.invalidate_ir_op(target);
            state.invalidate_ir_op(IrOp::ir_op_ir_op_kind_u32(
                target.kind(),
                target.index() + 1,
            ));
            state.invalidate_user_call();
        }
        IrCmd::FALLBACK_PREPVARARGS => {}
        IrCmd::FALLBACK_DUPCLOSURE => {
            state.invalidate_ir_op(op_b(inst.clone()));
            state.invalidate_heap_table_data();
        }
        IrCmd::FALLBACK_FORGPREP => {
            let target = op_b(inst.clone());
            state.invalidate_ir_op(target);
            state.invalidate_ir_op(IrOp::ir_op_ir_op_kind_u32(
                target.kind(),
                target.index() + 1,
            ));
            state.invalidate_ir_op(IrOp::ir_op_ir_op_kind_u32(
                target.kind(),
                target.index() + 2,
            ));
            state.invalidate_user_call();
        }
        _ => {}
    }
}
