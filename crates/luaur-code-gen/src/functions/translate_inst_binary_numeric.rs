use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_op_kind::IrOpKind;
use crate::enums::ir_op_kind::IrOpKind::*;
use crate::functions::get_initialized_fallback::get_initialized_fallback;
use crate::functions::is_userdata_bytecode_type::is_userdata_bytecode_type;
use crate::functions::load_double_or_constant::load_double_or_constant;
use crate::functions::translate_binary_numeric_fallback_if_required::translate_binary_numeric_fallback_if_required;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::bytecode_types::BytecodeTypes;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_function::IrFunction;
use crate::records::ir_op::IrOp;
use crate::type_aliases::instruction_ir_builder::Instruction;
use luaur_common::enums::luau_builtin_function::LuauBuiltinFunction;
use luaur_common::enums::luau_bytecode_type::LuauBytecodeType;
use luaur_vm::enums::lua_type::lua_Type;
use luaur_vm::type_aliases::tms::TMS;

pub fn translate_inst_binary_numeric(
    build: &mut IrBuilder,
    ra: i32,
    rb: i32,
    rc: i32,
    opb: IrOp,
    opc: IrOp,
    pcpos: i32,
    tm: TMS,
) {
    let mut fallback = IrOp::ir_op();

    let bc_types = build.function.get_bytecode_types_at(pcpos);

    // Special fast-paths for vectors, matching the cases we have in VM
    if bc_types.a == LuauBytecodeType::LBC_TYPE_VECTOR.0 as u8
        && bc_types.b == LuauBytecodeType::LBC_TYPE_VECTOR.0 as u8
        && (tm == TMS::TM_ADD
            || tm == TMS::TM_SUB
            || tm == TMS::TM_MUL
            || tm == TMS::TM_DIV
            || tm == TMS::TM_IDIV)
    {
        let reg_rb = build.vm_reg(rb as u8);
        let tag_b = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, reg_rb);
        let vector_tag = build.const_tag(lua_Type::LUA_TVECTOR as u8);
        let exit = build.vm_exit(pcpos as u32);
        build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TAG, tag_b, vector_tag, exit);

        let reg_rc = build.vm_reg(rc as u8);
        let tag_c = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, reg_rc);
        let vector_tag = build.const_tag(lua_Type::LUA_TVECTOR as u8);
        let exit = build.vm_exit(pcpos as u32);
        build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TAG, tag_c, vector_tag, exit);

        let vb = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_TVALUE, opb, IrOp::ir_op());
        let vb = vb; // keep naming aligned with original

        let vc = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_TVALUE, opc, IrOp::ir_op());
        let vc = vc;

        let result = match tm {
            TMS::TM_ADD => build.inst_ir_cmd_ir_op_ir_op(IrCmd::ADD_VEC, vb, vc),
            TMS::TM_SUB => build.inst_ir_cmd_ir_op_ir_op(IrCmd::SUB_VEC, vb, vc),
            TMS::TM_MUL => build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_VEC, vb, vc),
            TMS::TM_DIV => build.inst_ir_cmd_ir_op_ir_op(IrCmd::DIV_VEC, vb, vc),
            TMS::TM_IDIV => build.inst_ir_cmd_ir_op_ir_op(IrCmd::IDIV_VEC, vb, vc),
            _ => {
                CODEGEN_ASSERT!(false);
                IrOp::ir_op()
            }
        };

        let result = build.inst_ir_cmd_ir_op_ir_op(IrCmd::TAG_VECTOR, result, IrOp::ir_op());
        let reg_ra = build.vm_reg(ra as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, reg_ra, result);
        return;
    } else if !is_userdata_bytecode_type(bc_types.a)
        && bc_types.b == LuauBytecodeType::LBC_TYPE_VECTOR.0 as u8
        && (tm == TMS::TM_MUL || tm == TMS::TM_DIV || tm == TMS::TM_IDIV)
    {
        if rb != -1 {
            let fallback_exit = if bc_types.a == LuauBytecodeType::LBC_TYPE_NUMBER.0 as u8 {
                build.vm_exit(pcpos as u32)
            } else {
                get_initialized_fallback(build, &mut fallback, pcpos)
            };

            let rb_reg = build.vm_reg(rb as u8);
            let tag_load = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, rb_reg);
            let number_tag = build.const_tag(lua_Type::LUA_TNUMBER as u8);
            build.inst_ir_cmd_ir_op_ir_op_ir_op(
                IrCmd::CHECK_TAG,
                tag_load,
                number_tag,
                fallback_exit,
            );
        }

        let reg_rc = build.vm_reg(rc as u8);
        let tag_rc = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, reg_rc);
        let vector_tag = build.const_tag(lua_Type::LUA_TVECTOR as u8);
        let exit = build.vm_exit(pcpos as u32);
        build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TAG, tag_rc, vector_tag, exit);

        let load_d = load_double_or_constant(build, opb);
        let undef = IrOp::ir_op();
        let num_float = build.inst_ir_cmd_ir_op_ir_op(IrCmd::NUM_TO_FLOAT, load_d, undef);
        let vb = build.inst_ir_cmd_ir_op_ir_op(IrCmd::FLOAT_TO_VEC, num_float, IrOp::ir_op());

        let vc = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TVALUE, opc);
        let result = match tm {
            TMS::TM_MUL => build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_VEC, vb, vc),
            TMS::TM_DIV => build.inst_ir_cmd_ir_op_ir_op(IrCmd::DIV_VEC, vb, vc),
            TMS::TM_IDIV => build.inst_ir_cmd_ir_op_ir_op(IrCmd::IDIV_VEC, vb, vc),
            _ => {
                CODEGEN_ASSERT!(false);
                IrOp::ir_op()
            }
        };

        let result = build.inst_ir_cmd_ir_op_ir_op(IrCmd::TAG_VECTOR, result, IrOp::ir_op());
        let reg_ra = build.vm_reg(ra as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, reg_ra, result);

        translate_binary_numeric_fallback_if_required(build, fallback, ra, opb, opc, tm, pcpos);
        return;
    } else if bc_types.a == LuauBytecodeType::LBC_TYPE_VECTOR.0 as u8
        && !is_userdata_bytecode_type(bc_types.b)
        && (tm == TMS::TM_MUL || tm == TMS::TM_DIV || tm == TMS::TM_IDIV)
    {
        let reg_rb = build.vm_reg(rb as u8);
        let tag_rb = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, reg_rb);
        let vector_tag = build.const_tag(lua_Type::LUA_TVECTOR as u8);
        let exit = build.vm_exit(pcpos as u32);
        build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TAG, tag_rb, vector_tag, exit);

        if rc != -1 {
            let fallback_exit = if bc_types.b == LuauBytecodeType::LBC_TYPE_NUMBER.0 as u8 {
                build.vm_exit(pcpos as u32)
            } else {
                get_initialized_fallback(build, &mut fallback, pcpos)
            };

            let rc_reg = build.vm_reg(rc as u8);
            let tag_load = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, rc_reg);
            let number_tag = build.const_tag(lua_Type::LUA_TNUMBER as u8);
            build.inst_ir_cmd_ir_op_ir_op_ir_op(
                IrCmd::CHECK_TAG,
                tag_load,
                number_tag,
                fallback_exit,
            );
        }

        let vb = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TVALUE, opb);
        let load_d = load_double_or_constant(build, opc);
        let num_float = build.inst_ir_cmd_ir_op_ir_op(IrCmd::NUM_TO_FLOAT, load_d, IrOp::ir_op());
        let vc = build.inst_ir_cmd_ir_op_ir_op(IrCmd::FLOAT_TO_VEC, num_float, IrOp::ir_op());

        let result = match tm {
            TMS::TM_MUL => build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_VEC, vb, vc),
            TMS::TM_DIV => build.inst_ir_cmd_ir_op_ir_op(IrCmd::DIV_VEC, vb, vc),
            TMS::TM_IDIV => build.inst_ir_cmd_ir_op_ir_op(IrCmd::IDIV_VEC, vb, vc),
            _ => {
                CODEGEN_ASSERT!(false);
                IrOp::ir_op()
            }
        };

        let result = build.inst_ir_cmd_ir_op_ir_op(IrCmd::TAG_VECTOR, result, IrOp::ir_op());
        let reg_ra = build.vm_reg(ra as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, reg_ra, result);

        translate_binary_numeric_fallback_if_required(build, fallback, ra, opb, opc, tm, pcpos);
        return;
    }

    if is_userdata_bytecode_type(bc_types.a) || is_userdata_bytecode_type(bc_types.b) {
        let savedpc = build.const_uint((pcpos + 1) as u32);
        build.inst_ir_cmd_ir_op(IrCmd::SET_SAVEDPC, savedpc);
        let reg_ra = build.vm_reg(ra as u8);
        let tm_op = build.const_int(tm as i32);
        build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::DO_ARITH, reg_ra, opb, opc, tm_op);
        return;
    }

    // fast-path: number
    if rb != -1 {
        let reg_rb = build.vm_reg(rb as u8);
        let tb = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, reg_rb);
        let exit_or_fallback = if bc_types.a == LuauBytecodeType::LBC_TYPE_NUMBER.0 as u8 {
            build.vm_exit(pcpos as u32)
        } else {
            get_initialized_fallback(build, &mut fallback, pcpos)
        };
        let number_tag = build.const_tag(lua_Type::LUA_TNUMBER as u8);
        build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TAG, tb, number_tag, exit_or_fallback);
    }

    if rc != -1 && rc != rb {
        let reg_rc = build.vm_reg(rc as u8);
        let tc = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, reg_rc);
        let exit_or_fallback = if bc_types.b == LuauBytecodeType::LBC_TYPE_NUMBER.0 as u8 {
            build.vm_exit(pcpos as u32)
        } else {
            get_initialized_fallback(build, &mut fallback, pcpos)
        };
        let number_tag = build.const_tag(lua_Type::LUA_TNUMBER as u8);
        build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TAG, tc, number_tag, exit_or_fallback);
    }

    let vb = load_double_or_constant(build, opb);
    let mut vc = IrOp::ir_op();
    let mut result = IrOp::ir_op();

    if opc.kind() == IrOpKind::VmConst {
        let protok_index = crate::functions::vm_const_op::vm_const_op(opc);
        CODEGEN_ASSERT!(build.function.proto.is_null() == false);
        let protok = unsafe { *(*build.function.proto).k.add(protok_index as usize) };
        CODEGEN_ASSERT!(protok.tt == lua_Type::LUA_TNUMBER as i32);

        let n = unsafe { protok.value.n };
        if tm == TMS::TM_POW && n == 0.5 {
            result = build.inst_ir_cmd_ir_op(IrCmd::SQRT_NUM, vb);
        } else if tm == TMS::TM_POW && n == 2.0 {
            result = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_NUM, vb, vb);
        } else if tm == TMS::TM_POW && n == 3.0 {
            let vv = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_NUM, vb, vb);
            result = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_NUM, vb, vv);
        } else {
            vc = build.const_double(n);
        }
    } else {
        vc = build.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, opc);
    }

    // If result is None, we need to emit the generic numeric op
    if result.kind() == IrOpKind::None {
        CODEGEN_ASSERT!(vc.kind() != IrOpKind::None);
        result = match tm {
            TMS::TM_ADD => build.inst_ir_cmd_ir_op_ir_op(IrCmd::ADD_NUM, vb, vc),
            TMS::TM_SUB => build.inst_ir_cmd_ir_op_ir_op(IrCmd::SUB_NUM, vb, vc),
            TMS::TM_MUL => build.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_NUM, vb, vc),
            TMS::TM_DIV => build.inst_ir_cmd_ir_op_ir_op(IrCmd::DIV_NUM, vb, vc),
            TMS::TM_IDIV => build.inst_ir_cmd_ir_op_ir_op(IrCmd::IDIV_NUM, vb, vc),
            TMS::TM_MOD => build.inst_ir_cmd_ir_op_ir_op(IrCmd::MOD_NUM, vb, vc),
            TMS::TM_POW => {
                let pow = build.const_uint(LuauBuiltinFunction::LBF_MATH_POW as u32);
                build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::INVOKE_LIBM, pow, vb, vc)
            }
            _ => {
                CODEGEN_ASSERT!(false);
                IrOp::ir_op()
            }
        };
    }

    let reg_ra = build.vm_reg(ra as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, reg_ra, result);

    if ra != rb && ra != rc {
        let reg_ra = build.vm_reg(ra as u8);
        let number_tag = build.const_tag(lua_Type::LUA_TNUMBER as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, reg_ra, number_tag);
    }

    translate_binary_numeric_fallback_if_required(build, fallback, ra, opb, opc, tm, pcpos);
}
