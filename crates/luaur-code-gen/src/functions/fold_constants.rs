use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_op_kind::IrOpKind;
use crate::functions::byteswap::byteswap;
use crate::functions::compare_ir_utils::compare_f64_f64_ir_condition;
use crate::functions::compare_ir_utils_alt_b::compare_i32_i32_ir_condition;
use crate::functions::compare_ir_utils_alt_c::compare_i64_i64_ir_condition;
use crate::functions::condition_op::condition_op;
use crate::functions::countlz_bit_utils::countlz_u32;
use crate::functions::countlz_bit_utils_alt_b::countlz_u64;
use crate::functions::countrz_bit_utils::countrz_u32;
use crate::functions::countrz_bit_utils_alt_b::countrz_u64;
use crate::functions::get_op_ir_data::get_op_mut;
use crate::functions::kill_ir_utils::kill_ir_function_ir_inst;
use crate::functions::lrotate::lrotate;
use crate::functions::replace_ir_utils::replace_ir_function_ir_op_ir_op;
use crate::functions::replace_ir_utils_alt_b::replace_ir_function_ir_block_u32_ir_inst;
use crate::functions::rrotate::rrotate;
use crate::functions::substitute::substitute;
use crate::functions::substitute_with_truncated_uint::substitute_with_truncated_uint;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_block::IrBlock;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_function::IrFunction;
use crate::records::ir_inst::IrInst;
use crate::records::ir_op::IrOp;
use crate::type_aliases::ir_ops::IrOps;
use luaur_vm::enums::lua_type::lua_Type;
use luaur_vm::functions::luai_numidiv::luai_numidiv;
use luaur_vm::functions::luai_nummod::luai_nummod;

// IrUtils.cpp: `constexpr double kDoubleMaxExactInteger = 9007199254740992.0;`
const K_DOUBLE_MAX_EXACT_INTEGER: f64 = 9007199254740992.0;

const LUA_TNIL: u8 = lua_Type::LUA_TNIL as u8;
const LUA_TBOOLEAN: u8 = lua_Type::LUA_TBOOLEAN as u8;
const LUA_TNUMBER: u8 = lua_Type::LUA_TNUMBER as u8;
const LUA_TINTEGER: u8 = lua_Type::LUA_TINTEGER as u8;

fn make_inst(cmd: IrCmd, ops: &[IrOp]) -> IrInst {
    let mut v = IrOps::new();
    for &o in ops {
        v.push(o);
    }
    IrInst {
        cmd,
        ops: v,
        ..Default::default()
    }
}

pub fn fold_constants(
    build: &mut IrBuilder,
    function: &mut IrFunction,
    block: &mut IrBlock,
    index: u32,
) {
    let inst_ptr: *mut IrInst = &mut function.instructions[index as usize];
    let cmd = unsafe { (*inst_ptr).cmd };

    // OP_A(inst)..OP_F(inst): read the n-th operand (copy; IrOp is Copy)
    let read = move |idx: usize| -> IrOp {
        let s = unsafe { (*inst_ptr).ops.as_slice() };
        if idx < s.len() {
            s[idx]
        } else {
            IrOp::default()
        }
    };
    let is_const = move |idx: usize| -> bool {
        let s = unsafe { (*inst_ptr).ops.as_slice() };
        let op = if idx < s.len() {
            s[idx]
        } else {
            IrOp::default()
        };
        op.kind() == IrOpKind::Constant
    };

    match cmd {
        IrCmd::ADD_INT => {
            if is_const(0) && is_const(1) {
                // Add as unsigned to force two's complement evaluation (avoid signed overflow UB)
                let lhs = function.int_op(read(0));
                let rhs = function.int_op(read(1));
                let sum = lhs.wrapping_add(rhs);
                let c = build.const_int(sum);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::SUB_INT => {
            if is_const(0) && is_const(1) {
                let lhs = function.int_op(read(0));
                let rhs = function.int_op(read(1));
                let sum = lhs.wrapping_sub(rhs);
                let c = build.const_int(sum);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::SEXTI8_INT => {
            if is_const(0) {
                let value = function.int_op(read(0)) as i8 as i32;
                let c = build.const_int(value);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::SEXTI16_INT => {
            if is_const(0) {
                let value = function.int_op(read(0)) as i16 as i32;
                let c = build.const_int(value);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::ADD_NUM => {
            if is_const(0) && is_const(1) {
                let v = function.double_op(read(0)) + function.double_op(read(1));
                let c = build.const_double(v);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::SUB_NUM => {
            if is_const(0) && is_const(1) {
                let v = function.double_op(read(0)) - function.double_op(read(1));
                let c = build.const_double(v);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::MUL_NUM => {
            if is_const(0) && is_const(1) {
                let v = function.double_op(read(0)) * function.double_op(read(1));
                let c = build.const_double(v);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::DIV_NUM => {
            if is_const(0) && is_const(1) {
                let v = function.double_op(read(0)) / function.double_op(read(1));
                let c = build.const_double(v);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::IDIV_NUM => {
            if is_const(0) && is_const(1) {
                let v = luai_numidiv(function.double_op(read(0)), function.double_op(read(1)));
                let c = build.const_double(v);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::MOD_NUM => {
            if is_const(0) && is_const(1) {
                let v = luai_nummod(function.double_op(read(0)), function.double_op(read(1)));
                let c = build.const_double(v);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::MIN_NUM => {
            if is_const(0) && is_const(1) {
                let a1 = function.double_op(read(0));
                let a2 = function.double_op(read(1));
                let c = build.const_double(if a1 < a2 { a1 } else { a2 });
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::MAX_NUM => {
            if is_const(0) && is_const(1) {
                let a1 = function.double_op(read(0));
                let a2 = function.double_op(read(1));
                let c = build.const_double(if a1 > a2 { a1 } else { a2 });
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::UNM_NUM => {
            if is_const(0) {
                let c = build.const_double(-function.double_op(read(0)));
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::FLOOR_NUM => {
            if is_const(0) {
                let c = build.const_double(function.double_op(read(0)).floor());
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::CEIL_NUM => {
            if is_const(0) {
                let c = build.const_double(function.double_op(read(0)).ceil());
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::ROUND_NUM => {
            if is_const(0) {
                let c = build.const_double(function.double_op(read(0)).round());
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::SQRT_NUM => {
            if is_const(0) {
                let c = build.const_double(function.double_op(read(0)).sqrt());
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::ABS_NUM => {
            if is_const(0) {
                let c = build.const_double(function.double_op(read(0)).abs());
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::SIGN_NUM => {
            if is_const(0) {
                let v = function.double_op(read(0));
                let r = if v > 0.0 {
                    1.0
                } else if v < 0.0 {
                    -1.0
                } else {
                    0.0
                };
                let c = build.const_double(r);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::ADD_FLOAT => {
            if is_const(0) && is_const(1) {
                let v = ((function.double_op(read(0)) as f32)
                    + (function.double_op(read(1)) as f32)) as f64;
                let c = build.const_double(v);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::SUB_FLOAT => {
            if is_const(0) && is_const(1) {
                let v = ((function.double_op(read(0)) as f32)
                    - (function.double_op(read(1)) as f32)) as f64;
                let c = build.const_double(v);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::MUL_FLOAT => {
            if is_const(0) && is_const(1) {
                let v = ((function.double_op(read(0)) as f32)
                    * (function.double_op(read(1)) as f32)) as f64;
                let c = build.const_double(v);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::DIV_FLOAT => {
            if is_const(0) && is_const(1) {
                let v = ((function.double_op(read(0)) as f32)
                    / (function.double_op(read(1)) as f32)) as f64;
                let c = build.const_double(v);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::MIN_FLOAT => {
            if is_const(0) && is_const(1) {
                let a1 = function.double_op(read(0)) as f32;
                let a2 = function.double_op(read(1)) as f32;
                let c = build.const_double((if a1 < a2 { a1 } else { a2 }) as f64);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::MAX_FLOAT => {
            if is_const(0) && is_const(1) {
                let a1 = function.double_op(read(0)) as f32;
                let a2 = function.double_op(read(1)) as f32;
                let c = build.const_double((if a1 > a2 { a1 } else { a2 }) as f64);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::UNM_FLOAT => {
            if is_const(0) {
                let c = build.const_double((-(function.double_op(read(0)) as f32)) as f64);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::FLOOR_FLOAT => {
            if is_const(0) {
                let c = build.const_double((function.double_op(read(0)) as f32).floor() as f64);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::CEIL_FLOAT => {
            if is_const(0) {
                let c = build.const_double((function.double_op(read(0)) as f32).ceil() as f64);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::SQRT_FLOAT => {
            if is_const(0) {
                let c = build.const_double((function.double_op(read(0)) as f32).sqrt() as f64);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::ABS_FLOAT => {
            if is_const(0) {
                let c = build.const_double((function.double_op(read(0)) as f32).abs() as f64);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::SIGN_FLOAT => {
            if is_const(0) {
                let v = function.double_op(read(0)) as f32;
                let r: f32 = if v > 0.0 {
                    1.0
                } else if v < 0.0 {
                    -1.0
                } else {
                    0.0
                };
                let c = build.const_double(r as f64);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::SELECT_NUM => {
            if is_const(2) && is_const(3) {
                let c = function.double_op(read(2));
                let d = function.double_op(read(3));
                let repl = if c == d { read(1) } else { read(0) };
                substitute(function, unsafe { &mut *inst_ptr }, repl);
            } else if read(0) == read(1) {
                // If the values are the same, no need to worry about the condition check
                let repl = read(0);
                substitute(function, unsafe { &mut *inst_ptr }, repl);
            }
        }
        IrCmd::SELECT_VEC => {
            if read(0) == read(1) {
                let repl = read(0);
                substitute(function, unsafe { &mut *inst_ptr }, repl);
            }
        }
        IrCmd::SELECT_IF_TRUTHY => {
            if read(1) == read(2) {
                let repl = read(1);
                substitute(function, unsafe { &mut *inst_ptr }, repl);
            }
        }
        IrCmd::NOT_ANY => {
            if is_const(0) {
                let a = function.tag_op(read(0));

                if a == LUA_TNIL {
                    let c = build.const_int(1);
                    substitute(function, unsafe { &mut *inst_ptr }, c);
                } else if a != LUA_TBOOLEAN {
                    let c = build.const_int(0);
                    substitute(function, unsafe { &mut *inst_ptr }, c);
                } else if is_const(1) {
                    let c = build.const_int(if function.int_op(read(1)) == 1 { 0 } else { 1 });
                    substitute(function, unsafe { &mut *inst_ptr }, c);
                }
            }
        }
        IrCmd::CMP_INT => {
            if is_const(0) && is_const(1) {
                let res = compare_i32_i32_ir_condition(
                    function.int_op(read(0)),
                    function.int_op(read(1)),
                    condition_op(read(2)),
                );
                let c = build.const_int(if res { 1 } else { 0 });
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::CMP_INT64 => {
            if is_const(0) && is_const(1) {
                let lhs = function.int64_op(read(0));
                let rhs = function.int64_op(read(1));
                let res = compare_i64_i64_ir_condition(lhs, rhs, condition_op(read(2)));
                let c = build.const_int(if res { 1 } else { 0 });
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::CMP_TAG => {
            if is_const(0) && is_const(1) {
                let cond = condition_op(read(2));
                CODEGEN_ASSERT!(
                    cond == crate::enums::ir_condition::IrCondition::Equal
                        || cond == crate::enums::ir_condition::IrCondition::NotEqual
                );

                let same = function.tag_op(read(0)) == function.tag_op(read(1));
                let val = if cond == crate::enums::ir_condition::IrCondition::Equal {
                    if same {
                        1
                    } else {
                        0
                    }
                } else if !same {
                    1
                } else {
                    0
                };
                let c = build.const_int(val);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::CMP_SPLIT_TVALUE => {
            CODEGEN_ASSERT!(is_const(1));

            let cond = condition_op(read(4));
            CODEGEN_ASSERT!(
                cond == crate::enums::ir_condition::IrCondition::Equal
                    || cond == crate::enums::ir_condition::IrCondition::NotEqual
            );

            if cond == crate::enums::ir_condition::IrCondition::Equal {
                if is_const(0) && function.tag_op(read(0)) != function.tag_op(read(1)) {
                    let c = build.const_int(0);
                    substitute(function, unsafe { &mut *inst_ptr }, c);
                } else if is_const(2) && is_const(3) {
                    // If the tag is a constant, this means previous condition has failed because tags are the same
                    let known_same_tag = is_const(0);
                    let tag_b = function.tag_op(read(1));
                    let same_value;
                    if tag_b == LUA_TBOOLEAN {
                        same_value = compare_i32_i32_ir_condition(
                            function.int_op(read(2)),
                            function.int_op(read(3)),
                            crate::enums::ir_condition::IrCondition::Equal,
                        );
                    } else if tag_b == LUA_TNUMBER {
                        same_value = compare_f64_f64_ir_condition(
                            function.double_op(read(2)),
                            function.double_op(read(3)),
                            crate::enums::ir_condition::IrCondition::Equal,
                        );
                    } else if tag_b == LUA_TINTEGER {
                        let lhs = function.int64_op(read(2));
                        let rhs = function.int64_op(read(3));
                        same_value = compare_i64_i64_ir_condition(
                            lhs,
                            rhs,
                            crate::enums::ir_condition::IrCondition::Equal,
                        );
                    } else {
                        CODEGEN_ASSERT!(false, "unsupported type");
                        same_value = false;
                    }

                    if known_same_tag && same_value {
                        let c = build.const_int(1);
                        substitute(function, unsafe { &mut *inst_ptr }, c);
                    } else if same_value {
                        let r = make_inst(IrCmd::CMP_TAG, &[read(0), read(1), read(4)]);
                        replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
                    } else {
                        let c = build.const_int(0);
                        substitute(function, unsafe { &mut *inst_ptr }, c);
                    }
                }
            } else {
                if is_const(0) && function.tag_op(read(0)) != function.tag_op(read(1)) {
                    let c = build.const_int(1);
                    substitute(function, unsafe { &mut *inst_ptr }, c);
                } else if is_const(2) && is_const(3) {
                    let known_same_tag = is_const(0);
                    let tag_b = function.tag_op(read(1));
                    let different_value;
                    if tag_b == LUA_TBOOLEAN {
                        different_value = compare_i32_i32_ir_condition(
                            function.int_op(read(2)),
                            function.int_op(read(3)),
                            crate::enums::ir_condition::IrCondition::NotEqual,
                        );
                    } else if tag_b == LUA_TNUMBER {
                        different_value = compare_f64_f64_ir_condition(
                            function.double_op(read(2)),
                            function.double_op(read(3)),
                            crate::enums::ir_condition::IrCondition::NotEqual,
                        );
                    } else if tag_b == LUA_TINTEGER {
                        let lhs = function.int64_op(read(2));
                        let rhs = function.int64_op(read(3));
                        different_value = compare_i64_i64_ir_condition(
                            lhs,
                            rhs,
                            crate::enums::ir_condition::IrCondition::NotEqual,
                        );
                    } else {
                        CODEGEN_ASSERT!(false, "unsupported type");
                        different_value = false;
                    }

                    if different_value {
                        let c = build.const_int(1);
                        substitute(function, unsafe { &mut *inst_ptr }, c);
                    } else if known_same_tag {
                        let c = build.const_int(0);
                        substitute(function, unsafe { &mut *inst_ptr }, c);
                    } else {
                        let r = make_inst(IrCmd::CMP_TAG, &[read(0), read(1), read(4)]);
                        replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
                    }
                }
            }
        }
        IrCmd::JUMP_EQ_TAG => {
            if is_const(0) && is_const(1) {
                if function.tag_op(read(0)) == function.tag_op(read(1)) {
                    let r = make_inst(IrCmd::JUMP, &[read(2)]);
                    replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
                } else {
                    let r = make_inst(IrCmd::JUMP, &[read(3)]);
                    replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
                }
            }
        }
        IrCmd::JUMP_CMP_INT => {
            if is_const(0) && is_const(1) {
                let res = compare_i32_i32_ir_condition(
                    function.int_op(read(0)),
                    function.int_op(read(1)),
                    condition_op(read(2)),
                );
                let r = if res {
                    make_inst(IrCmd::JUMP, &[read(3)])
                } else {
                    make_inst(IrCmd::JUMP, &[read(4)])
                };
                replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
            }
        }
        IrCmd::JUMP_CMP_NUM => {
            if is_const(0) && is_const(1) {
                let res = compare_f64_f64_ir_condition(
                    function.double_op(read(0)),
                    function.double_op(read(1)),
                    condition_op(read(2)),
                );
                let r = if res {
                    make_inst(IrCmd::JUMP, &[read(3)])
                } else {
                    make_inst(IrCmd::JUMP, &[read(4)])
                };
                replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
            }
        }
        IrCmd::JUMP_CMP_FLOAT => {
            if is_const(0) && is_const(1) {
                let a = function.double_op(read(0)) as f32 as f64;
                let b = function.double_op(read(1)) as f32 as f64;
                let res = compare_f64_f64_ir_condition(a, b, condition_op(read(2)));
                let r = if res {
                    make_inst(IrCmd::JUMP, &[read(3)])
                } else {
                    make_inst(IrCmd::JUMP, &[read(4)])
                };
                replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
            }
        }
        IrCmd::TRY_NUM_TO_INDEX => {
            if is_const(0) {
                let value = function.double_op(read(0));

                // To avoid undefined behavior of casting a value not representable in the target type, we check the range
                if value >= i32::MIN as f64 && value <= i32::MAX as f64 {
                    let arr_index = value as i32;

                    if arr_index as f64 == value {
                        let c = build.const_int(arr_index);
                        substitute(function, unsafe { &mut *inst_ptr }, c);
                    } else {
                        let r = make_inst(IrCmd::JUMP, &[read(1)]);
                        replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
                    }
                } else {
                    let r = make_inst(IrCmd::JUMP, &[read(1)]);
                    replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
                }
            }
        }
        IrCmd::INT_TO_NUM => {
            if is_const(0) {
                let c = build.const_double(function.int_op(read(0)) as f64);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::INT64_TO_NUM => {
            if is_const(0) {
                let v = function.int64_op(read(0)) as f64;
                let c = build.const_double(v);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::UINT_TO_NUM => {
            if is_const(0) {
                let c = build.const_double((function.int_op(read(0)) as u32) as f64);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::UINT_TO_FLOAT => {
            if is_const(0) {
                let c = build.const_double(((function.int_op(read(0)) as u32) as f32) as f64);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::NUM_TO_INT => {
            if is_const(0) {
                let value = function.double_op(read(0));

                // matches luai_num2int range check
                if value >= i32::MIN as f64 && value <= i32::MAX as f64 {
                    let c = build.const_int(value as i32);
                    substitute(function, unsafe { &mut *inst_ptr }, c);
                }
            }
        }
        IrCmd::NUM_TO_UINT => {
            if is_const(0) {
                let value = function.double_op(read(0));

                // matches luai_num2unsigned range check
                if value >= -K_DOUBLE_MAX_EXACT_INTEGER && value <= K_DOUBLE_MAX_EXACT_INTEGER {
                    let c = build.const_int((value as i64 as u32) as i32);
                    substitute(function, unsafe { &mut *inst_ptr }, c);
                }
            }
        }
        IrCmd::NUM_TO_INT64 => {
            if is_const(0) {
                let value = function.double_op(read(0));

                if value >= i64::MIN as f64 && value < i64::MAX as f64 {
                    let c = build.const_int_64(value as i64);
                    substitute(function, unsafe { &mut *inst_ptr }, c);
                }
            }
        }
        IrCmd::FLOAT_TO_NUM => {
            // float -> double for a constant is a no-op
            if is_const(0) {
                let c = build.const_double(function.double_op(read(0)));
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::NUM_TO_FLOAT => {
            // double -> float for a constant just needs to lower precision
            if is_const(0) {
                let c = build.const_double((function.double_op(read(0)) as f32) as f64);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::TRUNCATE_UINT => {
            // Truncating a constant integer is a no-op as constant integers only store 32 bits
            if is_const(0) {
                let repl = read(0);
                substitute(function, unsafe { &mut *inst_ptr }, repl);
            }
        }
        IrCmd::CHECK_TAG => {
            if is_const(0) && is_const(1) {
                if function.tag_op(read(0)) == function.tag_op(read(1)) {
                    kill_ir_function_ir_inst(function, unsafe { &mut *inst_ptr });
                } else {
                    let r = make_inst(IrCmd::JUMP, &[read(2)]); // Shows a conflict in assumptions on this path
                    replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
                }
            }
        }
        IrCmd::CHECK_TRUTHY => {
            if is_const(0) {
                if function.tag_op(read(0)) == LUA_TNIL {
                    let r = make_inst(IrCmd::JUMP, &[read(2)]); // Shows a conflict in assumptions on this path
                    replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
                } else if function.tag_op(read(0)) == LUA_TBOOLEAN {
                    if is_const(1) {
                        if function.int_op(read(1)) == 0 {
                            let r = make_inst(IrCmd::JUMP, &[read(2)]); // Shows a conflict in assumptions on this path
                            replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
                        } else {
                            kill_ir_function_ir_inst(function, unsafe { &mut *inst_ptr });
                        }
                    }
                } else {
                    kill_ir_function_ir_inst(function, unsafe { &mut *inst_ptr });
                }
            }
        }
        IrCmd::CHECK_CMP_NUM => {
            if is_const(0) && is_const(1) {
                if compare_f64_f64_ir_condition(
                    function.double_op(read(0)),
                    function.double_op(read(1)),
                    condition_op(read(2)),
                ) {
                    kill_ir_function_ir_inst(function, unsafe { &mut *inst_ptr });
                } else {
                    let r = make_inst(IrCmd::JUMP, &[read(3)]);
                    replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
                }
            }
        }
        IrCmd::CHECK_CMP_INT => {
            if is_const(0) && is_const(1) {
                if compare_i32_i32_ir_condition(
                    function.int_op(read(0)),
                    function.int_op(read(1)),
                    condition_op(read(2)),
                ) {
                    kill_ir_function_ir_inst(function, unsafe { &mut *inst_ptr });
                } else {
                    let r = make_inst(IrCmd::JUMP, &[read(3)]); // Shows a conflict in assumptions on this path
                    replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
                }
            }
        }
        IrCmd::ADD_INT64 => {
            if is_const(0) && is_const(1) {
                let lhs = function.int64_op(read(0));
                let rhs = function.int64_op(read(1));
                let c = build.const_int_64(lhs.wrapping_add(rhs));
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::SUB_INT64 => {
            if is_const(0) && is_const(1) {
                let lhs = function.int64_op(read(0));
                let rhs = function.int64_op(read(1));
                let c = build.const_int_64(lhs.wrapping_sub(rhs));
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::MUL_INT64 => {
            if is_const(0) && is_const(1) {
                let lhs = function.int64_op(read(0));
                let rhs = function.int64_op(read(1));
                let c = build.const_int_64(lhs.wrapping_mul(rhs));
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::DIV_INT64 => {
            if is_const(0) && is_const(1) {
                let lhs = function.int64_op(read(0));
                let rhs = function.int64_op(read(1));
                if rhs != 0 && !(lhs == i64::MIN && rhs == -1) {
                    let c = build.const_int_64(lhs / rhs);
                    substitute(function, unsafe { &mut *inst_ptr }, c);
                }
            }
        }
        IrCmd::IDIV_INT64 => {
            if is_const(0) && is_const(1) {
                let lhs = function.int64_op(read(0));
                let rhs = function.int64_op(read(1));
                if rhs != 0 && !(lhs == i64::MIN && rhs == -1) {
                    let mut q = lhs / rhs;
                    // Floored division: adjust if signs differ and there's a remainder
                    if (lhs ^ rhs) < 0 && q.wrapping_mul(rhs) != lhs {
                        q -= 1;
                    }
                    let c = build.const_int_64(q);
                    substitute(function, unsafe { &mut *inst_ptr }, c);
                }
            }
        }
        IrCmd::UDIV_INT64 => {
            if is_const(0) && is_const(1) {
                let lhs = function.int64_op(read(0)) as u64;
                let rhs = function.int64_op(read(1)) as u64;
                if rhs != 0 {
                    let c = build.const_int_64((lhs / rhs) as i64);
                    substitute(function, unsafe { &mut *inst_ptr }, c);
                }
            }
        }
        IrCmd::REM_INT64 => {
            if is_const(0) && is_const(1) {
                let lhs = function.int64_op(read(0));
                let rhs = function.int64_op(read(1));
                if rhs != 0 && !(lhs == i64::MIN && rhs == -1) {
                    let c = build.const_int_64(lhs % rhs);
                    substitute(function, unsafe { &mut *inst_ptr }, c);
                }
            }
        }
        IrCmd::UREM_INT64 => {
            if is_const(0) && is_const(1) {
                let lhs = function.int64_op(read(0)) as u64;
                let rhs = function.int64_op(read(1)) as u64;
                if rhs != 0 {
                    let c = build.const_int_64((lhs % rhs) as i64);
                    substitute(function, unsafe { &mut *inst_ptr }, c);
                }
            }
        }
        IrCmd::MOD_INT64 => {
            if is_const(0) && is_const(1) {
                let lhs = function.int64_op(read(0));
                let rhs = function.int64_op(read(1));
                if rhs != 0 && !(lhs == i64::MIN && rhs == -1) {
                    let mut rem = lhs % rhs;
                    // Floored modulus: adjust if remainder != 0 and signs differ
                    if rem != 0 && (rem ^ rhs) < 0 {
                        rem += rhs;
                    }
                    let c = build.const_int_64(rem);
                    substitute(function, unsafe { &mut *inst_ptr }, c);
                }
            }
        }
        IrCmd::CHECK_DIV_INT64 => {
            if is_const(0) && is_const(1) {
                let lhs = function.int64_op(read(0));
                let rhs = function.int64_op(read(1));
                if rhs != 0 && !(lhs == i64::MIN && rhs == -1) {
                    kill_ir_function_ir_inst(function, unsafe { &mut *inst_ptr });
                // guard is satisfied, eliminate it
                } else {
                    let r = make_inst(IrCmd::JUMP, &[read(2)]);
                    replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
                }
            }
        }
        IrCmd::CHECK_CMP_INT64 => {
            if is_const(0) && is_const(1) {
                let lhs = function.int64_op(read(0));
                let rhs = function.int64_op(read(1));
                if compare_i64_i64_ir_condition(lhs, rhs, condition_op(read(2))) {
                    kill_ir_function_ir_inst(function, unsafe { &mut *inst_ptr });
                } else {
                    let r = make_inst(IrCmd::JUMP, &[read(3)]);
                    replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
                }
            }
        }
        IrCmd::BITAND_INT64 => {
            if is_const(0) && is_const(1) {
                let op1 = function.int64_op(read(0));
                let op2 = function.int64_op(read(1));
                let c = build.const_int_64(op1 & op2);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            } else if is_const(0) && function.int64_op(read(0)) == 0 {
                let c = build.const_int_64(0);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            } else if is_const(0) && function.int64_op(read(0)) == -1 {
                let repl = read(1);
                substitute(function, unsafe { &mut *inst_ptr }, repl);
            } else if is_const(1) && function.int64_op(read(1)) == 0 {
                let c = build.const_int_64(0);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            } else if is_const(1) && function.int64_op(read(1)) == -1 {
                let repl = read(0);
                substitute(function, unsafe { &mut *inst_ptr }, repl);
            }
        }
        IrCmd::BITXOR_INT64 => {
            if is_const(0) && is_const(1) {
                let op1 = function.int64_op(read(0));
                let op2 = function.int64_op(read(1));
                let c = build.const_int_64(op1 ^ op2);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            } else if is_const(0) && function.int64_op(read(0)) == 0 {
                let repl = read(1);
                substitute(function, unsafe { &mut *inst_ptr }, repl);
            } else if is_const(1) && function.int64_op(read(1)) == 0 {
                let repl = read(0);
                substitute(function, unsafe { &mut *inst_ptr }, repl);
            }
        }
        IrCmd::BITOR_INT64 => {
            if is_const(0) && is_const(1) {
                let op1 = function.int64_op(read(0));
                let op2 = function.int64_op(read(1));
                let c = build.const_int_64(op1 | op2);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            } else if is_const(0) && function.int64_op(read(0)) == 0 {
                let repl = read(1);
                substitute(function, unsafe { &mut *inst_ptr }, repl);
            } else if is_const(0) && function.int64_op(read(0)) == -1 {
                let c = build.const_int_64(-1);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            } else if is_const(1) && function.int64_op(read(1)) == 0 {
                let repl = read(0);
                substitute(function, unsafe { &mut *inst_ptr }, repl);
            } else if is_const(1) && function.int64_op(read(1)) == -1 {
                let c = build.const_int_64(-1);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::BITNOT_INT64 => {
            if is_const(0) {
                let op1 = function.int64_op(read(0));
                let c = build.const_int_64(!op1);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::BITLSHIFT_INT64 => {
            if is_const(0) && is_const(1) {
                let n = function.int64_op(read(0)) as u64;
                let i = function.int64_op(read(1));
                let result = if (-63..=63).contains(&i) {
                    (if i < 0 {
                        n >> ((-i) as u32)
                    } else {
                        n << (i as u32)
                    }) as i64
                } else {
                    0
                };
                let c = build.const_int_64(result);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::BITRSHIFT_INT64 => {
            if is_const(0) && is_const(1) {
                let n = function.int64_op(read(0)) as u64;
                let i = function.int64_op(read(1));
                let result = if (-63..=63).contains(&i) {
                    (if i < 0 {
                        n << ((-i) as u32)
                    } else {
                        n >> (i as u32)
                    }) as i64
                } else {
                    0
                };
                let c = build.const_int_64(result);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::BITARSHIFT_INT64 => {
            if is_const(0) && is_const(1) {
                let n = function.int64_op(read(0));
                let i = function.int64_op(read(1));
                let result = if (-63..=63).contains(&i) {
                    if i < 0 {
                        ((n as u64) << ((-i) as u32)) as i64
                    } else {
                        n >> (i as u32) // arithmetic shift for signed i64
                    }
                } else if i < -63 {
                    0
                } else if n < 0 {
                    -1
                } else {
                    0
                };
                let c = build.const_int_64(result);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::BITLROTATE_INT64 => {
            if is_const(0) && is_const(1) {
                let n = function.int64_op(read(0)) as u64;
                let s = ((function.int64_op(read(1)) as u64) % 64) as u32;
                let r = if s != 0 {
                    (n << s) | (n >> (64 - s))
                } else {
                    n
                };
                let c = build.const_int_64(r as i64);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::BITRROTATE_INT64 => {
            if is_const(0) && is_const(1) {
                let n = function.int64_op(read(0)) as u64;
                let s = ((function.int64_op(read(1)) as u64) % 64) as u32;
                let r = if s != 0 {
                    (n >> s) | (n << (64 - s))
                } else {
                    n
                };
                let c = build.const_int_64(r as i64);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::BITCOUNTLZ_INT64 => {
            if is_const(0) {
                let n = function.int64_op(read(0)) as u64;
                let c = build.const_int_64(countlz_u64(n) as i64);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::BITCOUNTRZ_INT64 => {
            if is_const(0) {
                let n = function.int64_op(read(0)) as u64;
                let c = build.const_int_64(countrz_u64(n) as i64);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::BYTESWAP_INT64 => {
            if is_const(0) {
                let a = function.int64_op(read(0)) as u64;
                let result = byteswap(a);
                let c = build.const_int_64(result as i64);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::BITAND_UINT => {
            if is_const(0) && is_const(1) {
                let op1 = function.int_op(read(0)) as u32;
                let op2 = function.int_op(read(1)) as u32;
                let c = build.const_int((op1 & op2) as i32);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            } else if is_const(0) && function.int_op(read(0)) == 0 {
                let c = build.const_int(0);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            } else if is_const(0) && function.int_op(read(0)) == -1 {
                let op = read(1);
                substitute_with_truncated_uint(function, block, unsafe { &mut *inst_ptr }, op);
            } else if is_const(1) && function.int_op(read(1)) == 0 {
                let c = build.const_int(0);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            } else if is_const(1) && function.int_op(read(1)) == -1 {
                let op = read(0);
                substitute_with_truncated_uint(function, block, unsafe { &mut *inst_ptr }, op);
            }
        }
        IrCmd::BITXOR_UINT => {
            if is_const(0) && is_const(1) {
                let op1 = function.int_op(read(0)) as u32;
                let op2 = function.int_op(read(1)) as u32;
                let c = build.const_int((op1 ^ op2) as i32);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            } else if is_const(0) && function.int_op(read(0)) == 0 {
                let op = read(1);
                substitute_with_truncated_uint(function, block, unsafe { &mut *inst_ptr }, op);
            } else if is_const(0) && function.int_op(read(0)) == -1 {
                let r = make_inst(IrCmd::BITNOT_UINT, &[read(1)]);
                replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
            } else if is_const(1) && function.int_op(read(1)) == 0 {
                let op = read(0);
                substitute_with_truncated_uint(function, block, unsafe { &mut *inst_ptr }, op);
            } else if is_const(1) && function.int_op(read(1)) == -1 {
                let r = make_inst(IrCmd::BITNOT_UINT, &[read(0)]);
                replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
            }
        }
        IrCmd::BITOR_UINT => {
            if is_const(0) && is_const(1) {
                let op1 = function.int_op(read(0)) as u32;
                let op2 = function.int_op(read(1)) as u32;
                let c = build.const_int((op1 | op2) as i32);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            } else if is_const(0) && function.int_op(read(0)) == 0 {
                let op = read(1);
                substitute_with_truncated_uint(function, block, unsafe { &mut *inst_ptr }, op);
            } else if is_const(0) && function.int_op(read(0)) == -1 {
                let c = build.const_int(-1);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            } else if is_const(1) && function.int_op(read(1)) == 0 {
                let op = read(0);
                substitute_with_truncated_uint(function, block, unsafe { &mut *inst_ptr }, op);
            } else if is_const(1) && function.int_op(read(1)) == -1 {
                let c = build.const_int(-1);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::BITNOT_UINT => {
            if is_const(0) {
                let c = build.const_int(!(function.int_op(read(0)) as u32) as i32);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::BITLSHIFT_UINT => {
            if is_const(0) && is_const(1) {
                let op1 = function.int_op(read(0)) as u32;
                let op2 = function.int_op(read(1));
                let c = build.const_int((op1 << ((op2 & 31) as u32)) as i32);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            } else if is_const(1) && function.int_op(read(1)) == 0 {
                let op = read(0);
                substitute_with_truncated_uint(function, block, unsafe { &mut *inst_ptr }, op);
            }
        }
        IrCmd::BITRSHIFT_UINT => {
            if is_const(0) && is_const(1) {
                let op1 = function.int_op(read(0)) as u32;
                let op2 = function.int_op(read(1));
                let c = build.const_int((op1 >> ((op2 & 31) as u32)) as i32);
                substitute(function, unsafe { &mut *inst_ptr }, c);
            } else if is_const(1) && function.int_op(read(1)) == 0 {
                let op = read(0);
                substitute_with_truncated_uint(function, block, unsafe { &mut *inst_ptr }, op);
            }
        }
        IrCmd::BITARSHIFT_UINT => {
            if is_const(0) && is_const(1) {
                let op1 = function.int_op(read(0));
                let op2 = function.int_op(read(1));
                // signed arithmetic right shift
                let c = build.const_int(op1 >> ((op2 & 31) as u32));
                substitute(function, unsafe { &mut *inst_ptr }, c);
            } else if is_const(1) && function.int_op(read(1)) == 0 {
                let op = read(0);
                substitute_with_truncated_uint(function, block, unsafe { &mut *inst_ptr }, op);
            }
        }
        IrCmd::BITLROTATE_UINT => {
            if is_const(0) && is_const(1) {
                let c = build.const_int(lrotate(
                    function.int_op(read(0)) as u32,
                    function.int_op(read(1)),
                ));
                substitute(function, unsafe { &mut *inst_ptr }, c);
            } else if is_const(1) && function.int_op(read(1)) == 0 {
                let op = read(0);
                substitute_with_truncated_uint(function, block, unsafe { &mut *inst_ptr }, op);
            }
        }
        IrCmd::BITRROTATE_UINT => {
            if is_const(0) && is_const(1) {
                let c = build.const_int(rrotate(
                    function.int_op(read(0)) as u32,
                    function.int_op(read(1)),
                ));
                substitute(function, unsafe { &mut *inst_ptr }, c);
            } else if is_const(1) && function.int_op(read(1)) == 0 {
                let op = read(0);
                substitute_with_truncated_uint(function, block, unsafe { &mut *inst_ptr }, op);
            }
        }
        IrCmd::BITCOUNTLZ_UINT => {
            if is_const(0) {
                let c = build.const_int(countlz_u32(function.int_op(read(0)) as u32));
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::BITCOUNTRZ_UINT => {
            if is_const(0) {
                let c = build.const_int(countrz_u32(function.int_op(read(0)) as u32));
                substitute(function, unsafe { &mut *inst_ptr }, c);
            }
        }
        IrCmd::CHECK_BUFFER_LEN => {
            if is_const(1) && is_const(4) {
                // If base offset and base offset source double value are both constants, we can get rid of that check or fallback
                if (function.int_op(read(1)) as f64) == function.double_op(read(4)) {
                    let u = build.undef();
                    // This disables equality check at runtime
                    replace_ir_function_ir_op_ir_op(
                        function,
                        get_op_mut(unsafe { &mut *inst_ptr }, 4),
                        u,
                    );
                } else {
                    let r = make_inst(IrCmd::JUMP, &[read(5)]); // Shows a conflict in assumptions on this path
                    replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
                }
            } else if read(1).kind() == IrOpKind::Inst && is_const(4) {
                // If only the base offset source double value is a constant, it means we couldn't constant-fold NUM_TO_INT
                let e_op = read(4);
                let inner = function.inst_op(read(1));
                let ok = inner.cmd == IrCmd::NUM_TO_INT
                    && inner.ops.as_slice().get(0).copied().unwrap_or_default() == e_op;
                CODEGEN_ASSERT!(ok);

                let r = make_inst(IrCmd::JUMP, &[read(5)]); // Shows a conflict in assumptions on this path
                replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
            }
        }
        _ => {}
    }
}
