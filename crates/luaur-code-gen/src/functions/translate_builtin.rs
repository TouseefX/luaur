use crate::enums::builtin_impl_type::BuiltinImplType;
use crate::enums::int_64_binary::Int64Binary;
use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_condition::IrCondition;
use crate::enums::ir_const_kind::IrConstKind;
use crate::enums::ir_op_kind::IrOpKind;
use crate::functions::is_compatible_constant::is_compatible_constant;
use crate::functions::translate_builtin_2_number_to_number_libm::translate_builtin_2_number_to_number_libm;
use crate::functions::translate_builtin_assert::translate_builtin_assert;
use crate::functions::translate_builtin_bit_32_bnot::translate_builtin_bit_32_bnot;
use crate::functions::translate_builtin_bit_32_extract::translate_builtin_bit_32_extract;
use crate::functions::translate_builtin_bit_32_extract_k::translate_builtin_bit_32_extract_k;
use crate::functions::translate_builtin_bit_32_multiarg_op::translate_builtin_bit_32_multiarg_op;
use crate::functions::translate_builtin_bit_32_replace::translate_builtin_bit_32_replace;
use crate::functions::translate_builtin_bit_32_rotate::translate_builtin_bit_32_rotate;
use crate::functions::translate_builtin_bit_32_shift::translate_builtin_bit_32_shift;
use crate::functions::translate_builtin_bit_32_unary::translate_builtin_bit_32_unary;
use crate::functions::translate_builtin_buffer_read::translate_builtin_buffer_read;
use crate::functions::translate_builtin_buffer_write::translate_builtin_buffer_write;
use crate::functions::translate_builtin_int_64_binary::translate_builtin_int_64_binary;
use crate::functions::translate_builtin_int_64_bnot::translate_builtin_int_64_bnot;
use crate::functions::translate_builtin_int_64_clamp::translate_builtin_int_64_clamp;
use crate::functions::translate_builtin_int_64_compare::translate_builtin_int_64_compare;
use crate::functions::translate_builtin_int_64_create::translate_builtin_int_64_create;
use crate::functions::translate_builtin_int_64_extract::translate_builtin_int_64_extract;
use crate::functions::translate_builtin_int_64_min_max::translate_builtin_int_64_min_max;
use crate::functions::translate_builtin_int_64_multiarg_op::translate_builtin_int_64_multiarg_op;
use crate::functions::translate_builtin_int_64_neg::translate_builtin_int_64_neg;
use crate::functions::translate_builtin_int_64_rotate::translate_builtin_int_64_rotate;
use crate::functions::translate_builtin_int_64_shift::translate_builtin_int_64_shift;
use crate::functions::translate_builtin_int_64_to_number::translate_builtin_int_64_to_number;
use crate::functions::translate_builtin_int_64_unary::translate_builtin_int_64_unary;
use crate::functions::translate_builtin_math_clamp::translate_builtin_math_clamp;
use crate::functions::translate_builtin_math_deg_rad::translate_builtin_math_deg_rad;
use crate::functions::translate_builtin_math_is_nan::translate_builtin_math_is_nan;
use crate::functions::translate_builtin_math_lerp::translate_builtin_math_lerp;
use crate::functions::translate_builtin_math_log::translate_builtin_math_log;
use crate::functions::translate_builtin_math_min_max::translate_builtin_math_min_max;
use crate::functions::translate_builtin_math_unary::translate_builtin_math_unary;
use crate::functions::translate_builtin_number_to_2_number::translate_builtin_number_to_2_number;
use crate::functions::translate_builtin_number_to_number_libm::translate_builtin_number_to_number_libm;
use crate::functions::translate_builtin_string_len::translate_builtin_string_len;
use crate::functions::translate_builtin_table_insert::translate_builtin_table_insert;
use crate::functions::translate_builtin_type::translate_builtin_type;
use crate::functions::translate_builtin_typeof::translate_builtin_typeof;
use crate::functions::translate_builtin_vector::translate_builtin_vector;
use crate::functions::translate_builtin_vector_clamp::translate_builtin_vector_clamp;
use crate::functions::translate_builtin_vector_cross::translate_builtin_vector_cross;
use crate::functions::translate_builtin_vector_dot::translate_builtin_vector_dot;
use crate::functions::translate_builtin_vector_lerp::translate_builtin_vector_lerp;
use crate::functions::translate_builtin_vector_magnitude::translate_builtin_vector_magnitude;
use crate::functions::translate_builtin_vector_map_1::translate_builtin_vector_map_1;
use crate::functions::translate_builtin_vector_map_1_x_4::translate_builtin_vector_map_1_x_4;
use crate::functions::translate_builtin_vector_min_max::translate_builtin_vector_min_max;
use crate::functions::translate_builtin_vector_normalize::translate_builtin_vector_normalize;
use crate::records::builtin_impl_result::BuiltinImplResult;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;
use luaur_common::enums::luau_builtin_function::LuauBuiltinFunction as LBF;
use luaur_common::FFlag;
use luaur_vm::enums::lua_type::lua_Type;
use luaur_vm::macros::lua_multret::LUA_MULTRET;

fn no_builtin() -> BuiltinImplResult {
    BuiltinImplResult {
        r#type: BuiltinImplType::None,
        actual_result_count: -1,
    }
}

pub fn translate_builtin(
    build: &mut IrBuilder,
    bfid: i32,
    ra: i32,
    arg: i32,
    args: IrOp,
    arg3: IrOp,
    nparams: i32,
    nresults: i32,
    fallback: IrOp,
    pcpos: i32,
) -> BuiltinImplResult {
    if nparams == LUA_MULTRET {
        return no_builtin();
    }

    if FFlag::LuauCodegenInteger2.get()
        && (args.kind() == IrOpKind::Constant || arg3.kind() == IrOpKind::Constant)
    {
        match bfid {
            x if x == LBF::LBF_MATH_MIN as i32
                || x == LBF::LBF_MATH_MAX as i32
                || x == LBF::LBF_MATH_POW as i32
                || x == LBF::LBF_MATH_FMOD as i32
                || x == LBF::LBF_MATH_ATAN2 as i32
                || x == LBF::LBF_MATH_LDEXP as i32
                || x == LBF::LBF_MATH_LERP as i32
                || x == LBF::LBF_MATH_CLAMP as i32
                || x == LBF::LBF_BIT32_BAND as i32
                || x == LBF::LBF_BIT32_BOR as i32
                || x == LBF::LBF_BIT32_BXOR as i32
                || x == LBF::LBF_BIT32_BTEST as i32
                || x == LBF::LBF_BIT32_LSHIFT as i32
                || x == LBF::LBF_BIT32_RSHIFT as i32
                || x == LBF::LBF_BIT32_ARSHIFT as i32
                || x == LBF::LBF_BIT32_LROTATE as i32
                || x == LBF::LBF_BIT32_RROTATE as i32
                || x == LBF::LBF_BIT32_EXTRACT as i32
                || x == LBF::LBF_BIT32_EXTRACTK as i32
                || x == LBF::LBF_BIT32_REPLACE as i32
                || x == LBF::LBF_VECTOR as i32
                || x == LBF::LBF_TABLE_INSERT as i32
                || x == LBF::LBF_BUFFER_READI8 as i32
                || x == LBF::LBF_BUFFER_READU8 as i32
                || x == LBF::LBF_BUFFER_WRITEU8 as i32
                || x == LBF::LBF_BUFFER_READI16 as i32
                || x == LBF::LBF_BUFFER_READU16 as i32
                || x == LBF::LBF_BUFFER_WRITEU16 as i32
                || x == LBF::LBF_BUFFER_READI32 as i32
                || x == LBF::LBF_BUFFER_READU32 as i32
                || x == LBF::LBF_BUFFER_WRITEU32 as i32
                || x == LBF::LBF_BUFFER_READF32 as i32
                || x == LBF::LBF_BUFFER_WRITEF32 as i32
                || x == LBF::LBF_BUFFER_READF64 as i32
                || x == LBF::LBF_BUFFER_WRITEF64 as i32
                || x == LBF::LBF_BUFFER_READINTEGER as i32 =>
            {
                if !is_compatible_constant(build, args, IrConstKind::Double) {
                    return no_builtin();
                }
                if !is_compatible_constant(build, arg3, IrConstKind::Double) {
                    return no_builtin();
                }
            }
            x if x == LBF::LBF_BUFFER_WRITEINTEGER as i32 => {
                if !is_compatible_constant(build, args, IrConstKind::Double) {
                    return no_builtin();
                }
                if !is_compatible_constant(build, arg3, IrConstKind::Int64) {
                    return no_builtin();
                }
            }
            x if x == LBF::LBF_INTEGER_ADD as i32
                || x == LBF::LBF_INTEGER_SUB as i32
                || x == LBF::LBF_INTEGER_MUL as i32
                || x == LBF::LBF_INTEGER_DIV as i32
                || x == LBF::LBF_INTEGER_IDIV as i32
                || x == LBF::LBF_INTEGER_UDIV as i32
                || x == LBF::LBF_INTEGER_REM as i32
                || x == LBF::LBF_INTEGER_UREM as i32
                || x == LBF::LBF_INTEGER_MOD as i32
                || x == LBF::LBF_INTEGER_MIN as i32
                || x == LBF::LBF_INTEGER_MAX as i32
                || x == LBF::LBF_INTEGER_CLAMP as i32
                || x == LBF::LBF_INTEGER_LT as i32
                || x == LBF::LBF_INTEGER_LE as i32
                || x == LBF::LBF_INTEGER_GT as i32
                || x == LBF::LBF_INTEGER_GE as i32
                || x == LBF::LBF_INTEGER_ULT as i32
                || x == LBF::LBF_INTEGER_ULE as i32
                || x == LBF::LBF_INTEGER_UGT as i32
                || x == LBF::LBF_INTEGER_UGE as i32
                || x == LBF::LBF_INTEGER_BAND as i32
                || x == LBF::LBF_INTEGER_BOR as i32
                || x == LBF::LBF_INTEGER_BXOR as i32
                || x == LBF::LBF_INTEGER_BNOT as i32
                || x == LBF::LBF_INTEGER_BTEST as i32
                || x == LBF::LBF_INTEGER_LSHIFT as i32
                || x == LBF::LBF_INTEGER_RSHIFT as i32
                || x == LBF::LBF_INTEGER_ARSHIFT as i32
                || x == LBF::LBF_INTEGER_LROTATE as i32
                || x == LBF::LBF_INTEGER_RROTATE as i32
                || x == LBF::LBF_INTEGER_EXTRACT as i32 =>
            {
                if !is_compatible_constant(build, args, IrConstKind::Int64) {
                    return no_builtin();
                }
                if !is_compatible_constant(build, arg3, IrConstKind::Int64) {
                    return no_builtin();
                }
            }
            _ => {}
        }
    }

    match bfid {
        x if x == LBF::LBF_ASSERT as i32 => {
            translate_builtin_assert(build, nparams, ra, arg, args, nresults, pcpos)
        }
        x if x == LBF::LBF_MATH_DEG as i32 => translate_builtin_math_deg_rad(
            build,
            IrCmd::DIV_NUM,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_MATH_RAD as i32 => translate_builtin_math_deg_rad(
            build,
            IrCmd::MUL_NUM,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_MATH_LOG as i32 => {
            translate_builtin_math_log(build, nparams, ra, arg, args, nresults, pcpos)
        }
        x if x == LBF::LBF_MATH_MIN as i32 => translate_builtin_math_min_max(
            build,
            IrCmd::MIN_NUM,
            nparams,
            ra,
            arg,
            args,
            arg3,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_MATH_MAX as i32 => translate_builtin_math_min_max(
            build,
            IrCmd::MAX_NUM,
            nparams,
            ra,
            arg,
            args,
            arg3,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_MATH_CLAMP as i32 => translate_builtin_math_clamp(
            build, nparams, ra, arg, args, arg3, nresults, fallback, pcpos,
        ),
        x if x == LBF::LBF_MATH_FLOOR as i32 => {
            translate_builtin_math_unary(build, IrCmd::FLOOR_NUM, nparams, ra, arg, nresults, pcpos)
        }
        x if x == LBF::LBF_MATH_CEIL as i32 => {
            translate_builtin_math_unary(build, IrCmd::CEIL_NUM, nparams, ra, arg, nresults, pcpos)
        }
        x if x == LBF::LBF_MATH_SQRT as i32 => {
            translate_builtin_math_unary(build, IrCmd::SQRT_NUM, nparams, ra, arg, nresults, pcpos)
        }
        x if x == LBF::LBF_MATH_ABS as i32 => {
            translate_builtin_math_unary(build, IrCmd::ABS_NUM, nparams, ra, arg, nresults, pcpos)
        }
        x if x == LBF::LBF_MATH_ROUND as i32 => {
            translate_builtin_math_unary(build, IrCmd::ROUND_NUM, nparams, ra, arg, nresults, pcpos)
        }
        x if x == LBF::LBF_MATH_EXP as i32
            || x == LBF::LBF_MATH_ASIN as i32
            || x == LBF::LBF_MATH_SIN as i32
            || x == LBF::LBF_MATH_SINH as i32
            || x == LBF::LBF_MATH_ACOS as i32
            || x == LBF::LBF_MATH_COS as i32
            || x == LBF::LBF_MATH_COSH as i32
            || x == LBF::LBF_MATH_ATAN as i32
            || x == LBF::LBF_MATH_TAN as i32
            || x == LBF::LBF_MATH_TANH as i32
            || x == LBF::LBF_MATH_LOG10 as i32 =>
        {
            translate_builtin_number_to_number_libm(
                build,
                unsafe { core::mem::transmute(bfid as u8) },
                nparams,
                ra,
                arg,
                nresults,
                pcpos,
            )
        }
        x if x == LBF::LBF_MATH_SIGN as i32 => {
            translate_builtin_math_unary(build, IrCmd::SIGN_NUM, nparams, ra, arg, nresults, pcpos)
        }
        x if x == LBF::LBF_MATH_POW as i32
            || x == LBF::LBF_MATH_FMOD as i32
            || x == LBF::LBF_MATH_ATAN2 as i32
            || x == LBF::LBF_MATH_LDEXP as i32 =>
        {
            translate_builtin_2_number_to_number_libm(
                build,
                unsafe { core::mem::transmute(bfid as u8) },
                nparams,
                ra,
                arg,
                args,
                nresults,
                pcpos,
            )
        }
        x if x == LBF::LBF_MATH_FREXP as i32 || x == LBF::LBF_MATH_MODF as i32 => {
            translate_builtin_number_to_2_number(
                build,
                unsafe { core::mem::transmute(bfid as u8) },
                nparams,
                ra,
                arg,
                args,
                nresults,
                pcpos,
            )
        }
        x if x == LBF::LBF_BIT32_BAND as i32 => translate_builtin_bit_32_multiarg_op(
            build,
            IrCmd::BITAND_UINT,
            false,
            nparams,
            ra,
            arg,
            args,
            arg3,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_BIT32_BOR as i32 => translate_builtin_bit_32_multiarg_op(
            build,
            IrCmd::BITOR_UINT,
            false,
            nparams,
            ra,
            arg,
            args,
            arg3,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_BIT32_BXOR as i32 => translate_builtin_bit_32_multiarg_op(
            build,
            IrCmd::BITXOR_UINT,
            false,
            nparams,
            ra,
            arg,
            args,
            arg3,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_BIT32_BTEST as i32 => translate_builtin_bit_32_multiarg_op(
            build,
            IrCmd::BITAND_UINT,
            true,
            nparams,
            ra,
            arg,
            args,
            arg3,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_BIT32_BNOT as i32 => {
            translate_builtin_bit_32_bnot(build, nparams, ra, arg, args, nresults, pcpos)
        }
        x if x == LBF::LBF_BIT32_LSHIFT as i32 => translate_builtin_bit_32_shift(
            build,
            IrCmd::BITLSHIFT_UINT,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_BIT32_RSHIFT as i32 => translate_builtin_bit_32_shift(
            build,
            IrCmd::BITRSHIFT_UINT,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_BIT32_ARSHIFT as i32 => translate_builtin_bit_32_shift(
            build,
            IrCmd::BITARSHIFT_UINT,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_BIT32_LROTATE as i32 => translate_builtin_bit_32_rotate(
            build,
            IrCmd::BITLROTATE_UINT,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_BIT32_RROTATE as i32 => translate_builtin_bit_32_rotate(
            build,
            IrCmd::BITRROTATE_UINT,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_BIT32_EXTRACT as i32 => {
            translate_builtin_bit_32_extract(build, nparams, ra, arg, args, arg3, nresults, pcpos)
        }
        x if x == LBF::LBF_BIT32_EXTRACTK as i32 => {
            translate_builtin_bit_32_extract_k(build, nparams, ra, arg, args, nresults, pcpos)
        }
        x if x == LBF::LBF_BIT32_COUNTLZ as i32 => translate_builtin_bit_32_unary(
            build,
            IrCmd::BITCOUNTLZ_UINT,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_BIT32_COUNTRZ as i32 => translate_builtin_bit_32_unary(
            build,
            IrCmd::BITCOUNTRZ_UINT,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_BIT32_REPLACE as i32 => {
            translate_builtin_bit_32_replace(build, nparams, ra, arg, args, arg3, nresults, pcpos)
        }
        x if x == LBF::LBF_TYPE as i32 => {
            translate_builtin_type(build, nparams, ra, arg, args, nresults)
        }
        x if x == LBF::LBF_TYPEOF as i32 => {
            translate_builtin_typeof(build, nparams, ra, arg, args, nresults)
        }
        x if x == LBF::LBF_VECTOR as i32 => {
            translate_builtin_vector(build, nparams, ra, arg, args, arg3, nresults, pcpos)
        }
        x if x == LBF::LBF_TABLE_INSERT as i32 => {
            translate_builtin_table_insert(build, nparams, ra, arg, args, nresults, pcpos)
        }
        x if x == LBF::LBF_STRING_LEN as i32 => {
            translate_builtin_string_len(build, nparams, ra, arg, args, nresults, pcpos)
        }
        x if x == LBF::LBF_BIT32_BYTESWAP as i32 => translate_builtin_bit_32_unary(
            build,
            IrCmd::BYTESWAP_UINT,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_BUFFER_READI8 as i32 => translate_builtin_buffer_read(
            build,
            nparams,
            ra,
            arg,
            args,
            arg3,
            nresults,
            pcpos,
            IrCmd::BUFFER_READI8,
            1,
            IrCmd::INT_TO_NUM,
            IrCmd::STORE_DOUBLE,
            lua_Type::LUA_TNUMBER as u8,
        ),
        x if x == LBF::LBF_BUFFER_READU8 as i32 => translate_builtin_buffer_read(
            build,
            nparams,
            ra,
            arg,
            args,
            arg3,
            nresults,
            pcpos,
            IrCmd::BUFFER_READU8,
            1,
            IrCmd::INT_TO_NUM,
            IrCmd::STORE_DOUBLE,
            lua_Type::LUA_TNUMBER as u8,
        ),
        x if x == LBF::LBF_BUFFER_WRITEU8 as i32 => translate_builtin_buffer_write(
            build,
            nparams,
            ra,
            arg,
            args,
            arg3,
            nresults,
            pcpos,
            IrCmd::BUFFER_WRITEI8,
            1,
            IrCmd::NUM_TO_UINT,
            false,
        ),
        x if x == LBF::LBF_BUFFER_READI16 as i32 => translate_builtin_buffer_read(
            build,
            nparams,
            ra,
            arg,
            args,
            arg3,
            nresults,
            pcpos,
            IrCmd::BUFFER_READI16,
            2,
            IrCmd::INT_TO_NUM,
            IrCmd::STORE_DOUBLE,
            lua_Type::LUA_TNUMBER as u8,
        ),
        x if x == LBF::LBF_BUFFER_READU16 as i32 => translate_builtin_buffer_read(
            build,
            nparams,
            ra,
            arg,
            args,
            arg3,
            nresults,
            pcpos,
            IrCmd::BUFFER_READU16,
            2,
            IrCmd::INT_TO_NUM,
            IrCmd::STORE_DOUBLE,
            lua_Type::LUA_TNUMBER as u8,
        ),
        x if x == LBF::LBF_BUFFER_WRITEU16 as i32 => translate_builtin_buffer_write(
            build,
            nparams,
            ra,
            arg,
            args,
            arg3,
            nresults,
            pcpos,
            IrCmd::BUFFER_WRITEI16,
            2,
            IrCmd::NUM_TO_UINT,
            false,
        ),
        x if x == LBF::LBF_BUFFER_READI32 as i32 => translate_builtin_buffer_read(
            build,
            nparams,
            ra,
            arg,
            args,
            arg3,
            nresults,
            pcpos,
            IrCmd::BUFFER_READI32,
            4,
            IrCmd::INT_TO_NUM,
            IrCmd::STORE_DOUBLE,
            lua_Type::LUA_TNUMBER as u8,
        ),
        x if x == LBF::LBF_BUFFER_READU32 as i32 => translate_builtin_buffer_read(
            build,
            nparams,
            ra,
            arg,
            args,
            arg3,
            nresults,
            pcpos,
            IrCmd::BUFFER_READI32,
            4,
            IrCmd::UINT_TO_NUM,
            IrCmd::STORE_DOUBLE,
            lua_Type::LUA_TNUMBER as u8,
        ),
        x if x == LBF::LBF_BUFFER_WRITEU32 as i32 => translate_builtin_buffer_write(
            build,
            nparams,
            ra,
            arg,
            args,
            arg3,
            nresults,
            pcpos,
            IrCmd::BUFFER_WRITEI32,
            4,
            IrCmd::NUM_TO_UINT,
            false,
        ),
        x if x == LBF::LBF_BUFFER_READF32 as i32 => translate_builtin_buffer_read(
            build,
            nparams,
            ra,
            arg,
            args,
            arg3,
            nresults,
            pcpos,
            IrCmd::BUFFER_READF32,
            4,
            IrCmd::FLOAT_TO_NUM,
            IrCmd::STORE_DOUBLE,
            lua_Type::LUA_TNUMBER as u8,
        ),
        x if x == LBF::LBF_BUFFER_WRITEF32 as i32 => translate_builtin_buffer_write(
            build,
            nparams,
            ra,
            arg,
            args,
            arg3,
            nresults,
            pcpos,
            IrCmd::BUFFER_WRITEF32,
            4,
            IrCmd::NUM_TO_FLOAT,
            false,
        ),
        x if x == LBF::LBF_BUFFER_READF64 as i32 => translate_builtin_buffer_read(
            build,
            nparams,
            ra,
            arg,
            args,
            arg3,
            nresults,
            pcpos,
            IrCmd::BUFFER_READF64,
            8,
            IrCmd::NOP,
            IrCmd::STORE_DOUBLE,
            lua_Type::LUA_TNUMBER as u8,
        ),
        x if x == LBF::LBF_BUFFER_WRITEF64 as i32 => translate_builtin_buffer_write(
            build,
            nparams,
            ra,
            arg,
            args,
            arg3,
            nresults,
            pcpos,
            IrCmd::BUFFER_WRITEF64,
            8,
            IrCmd::NOP,
            false,
        ),
        x if x == LBF::LBF_BUFFER_READINTEGER as i32 => {
            if FFlag::LuauCodegenBufferInteger.get() {
                translate_builtin_buffer_read(
                    build,
                    nparams,
                    ra,
                    arg,
                    args,
                    arg3,
                    nresults,
                    pcpos,
                    IrCmd::BUFFER_READI64,
                    8,
                    IrCmd::NOP,
                    IrCmd::STORE_INT64,
                    lua_Type::LUA_TINTEGER as u8,
                )
            } else {
                no_builtin()
            }
        }
        x if x == LBF::LBF_BUFFER_WRITEINTEGER as i32 => {
            if FFlag::LuauCodegenBufferInteger.get() {
                translate_builtin_buffer_write(
                    build,
                    nparams,
                    ra,
                    arg,
                    args,
                    arg3,
                    nresults,
                    pcpos,
                    IrCmd::BUFFER_WRITEI64,
                    8,
                    IrCmd::NOP,
                    true,
                )
            } else {
                no_builtin()
            }
        }
        x if x == LBF::LBF_VECTOR_MAGNITUDE as i32 => {
            translate_builtin_vector_magnitude(build, nparams, ra, arg, args, arg3, nresults, pcpos)
        }
        x if x == LBF::LBF_VECTOR_NORMALIZE as i32 => {
            translate_builtin_vector_normalize(build, nparams, ra, arg, args, arg3, nresults, pcpos)
        }
        x if x == LBF::LBF_VECTOR_CROSS as i32 => {
            translate_builtin_vector_cross(build, nparams, ra, arg, args, arg3, nresults, pcpos)
        }
        x if x == LBF::LBF_VECTOR_DOT as i32 => {
            translate_builtin_vector_dot(build, nparams, ra, arg, args, arg3, nresults, pcpos)
        }
        x if x == LBF::LBF_VECTOR_FLOOR as i32 => translate_builtin_vector_map_1_x_4(
            build,
            IrCmd::FLOOR_VEC,
            nparams,
            ra,
            arg,
            args,
            arg3,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_VECTOR_CEIL as i32 => translate_builtin_vector_map_1_x_4(
            build,
            IrCmd::CEIL_VEC,
            nparams,
            ra,
            arg,
            args,
            arg3,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_VECTOR_ABS as i32 => translate_builtin_vector_map_1_x_4(
            build,
            IrCmd::ABS_VEC,
            nparams,
            ra,
            arg,
            args,
            arg3,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_VECTOR_SIGN as i32 => translate_builtin_vector_map_1(
            build,
            IrCmd::SIGN_FLOAT,
            nparams,
            ra,
            arg,
            args,
            arg3,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_VECTOR_CLAMP as i32 => translate_builtin_vector_clamp(
            build, nparams, ra, arg, args, arg3, nresults, fallback, pcpos,
        ),
        x if x == LBF::LBF_VECTOR_MIN as i32 => translate_builtin_vector_min_max(
            build,
            IrCmd::MIN_VEC,
            nparams,
            ra,
            arg,
            args,
            arg3,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_VECTOR_MAX as i32 => translate_builtin_vector_min_max(
            build,
            IrCmd::MAX_VEC,
            nparams,
            ra,
            arg,
            args,
            arg3,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_VECTOR_LERP as i32 => {
            translate_builtin_vector_lerp(build, nparams, ra, arg, args, arg3, nresults, pcpos)
        }
        x if x == LBF::LBF_MATH_LERP as i32 => translate_builtin_math_lerp(
            build, nparams, ra, arg, args, arg3, nresults, fallback, pcpos,
        ),
        x if x == LBF::LBF_MATH_ISNAN as i32 => {
            translate_builtin_math_is_nan(build, nparams, ra, arg, args, nresults, pcpos)
        }
        x if x == LBF::LBF_INTEGER_CREATE as i32 => {
            if FFlag::LuauCodegenInteger2.get() {
                translate_builtin_int_64_create(build, nparams, ra, arg, nresults, pcpos)
            } else {
                no_builtin()
            }
        }
        x if x == LBF::LBF_INTEGER_TONUMBER as i32 => {
            if FFlag::LuauCodegenInteger2.get() {
                translate_builtin_int_64_to_number(build, nparams, ra, arg, nresults, pcpos)
            } else {
                no_builtin()
            }
        }
        x if x == LBF::LBF_INTEGER_ADD as i32 => int2_binary(
            build,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
            Int64Binary::Add,
        ),
        x if x == LBF::LBF_INTEGER_SUB as i32 => int2_binary(
            build,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
            Int64Binary::Sub,
        ),
        x if x == LBF::LBF_INTEGER_MUL as i32 => int2_binary(
            build,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
            Int64Binary::Mul,
        ),
        x if x == LBF::LBF_INTEGER_DIV as i32 => int2_binary(
            build,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
            Int64Binary::Div,
        ),
        x if x == LBF::LBF_INTEGER_IDIV as i32 => int2_binary(
            build,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
            Int64Binary::Idiv,
        ),
        x if x == LBF::LBF_INTEGER_UDIV as i32 => int2_binary(
            build,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
            Int64Binary::Udiv,
        ),
        x if x == LBF::LBF_INTEGER_REM as i32 => int2_binary(
            build,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
            Int64Binary::Rem,
        ),
        x if x == LBF::LBF_INTEGER_UREM as i32 => int2_binary(
            build,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
            Int64Binary::Urem,
        ),
        x if x == LBF::LBF_INTEGER_MOD as i32 => int2_binary(
            build,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
            Int64Binary::Mod,
        ),
        x if x == LBF::LBF_INTEGER_MIN as i32 => {
            if FFlag::LuauCodegenInteger2.get() {
                translate_builtin_int_64_min_max(
                    build, nparams, ra, arg, args, arg3, nresults, pcpos, true,
                )
            } else {
                no_builtin()
            }
        }
        x if x == LBF::LBF_INTEGER_MAX as i32 => {
            if FFlag::LuauCodegenInteger2.get() {
                translate_builtin_int_64_min_max(
                    build, nparams, ra, arg, args, arg3, nresults, pcpos, false,
                )
            } else {
                no_builtin()
            }
        }
        x if x == LBF::LBF_INTEGER_NEG as i32 => {
            if FFlag::LuauCodegenInteger2.get() {
                translate_builtin_int_64_neg(build, nparams, ra, arg, nresults, pcpos)
            } else {
                no_builtin()
            }
        }
        x if x == LBF::LBF_INTEGER_CLAMP as i32 => {
            if FFlag::LuauCodegenInteger2.get() {
                translate_builtin_int_64_clamp(build, nparams, ra, arg, args, arg3, nresults, pcpos)
            } else {
                no_builtin()
            }
        }
        x if x == LBF::LBF_INTEGER_LT as i32 => int2_compare(
            build,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
            IrCondition::Less,
        ),
        x if x == LBF::LBF_INTEGER_LE as i32 => int2_compare(
            build,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
            IrCondition::LessEqual,
        ),
        x if x == LBF::LBF_INTEGER_GT as i32 => int2_compare(
            build,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
            IrCondition::Greater,
        ),
        x if x == LBF::LBF_INTEGER_GE as i32 => int2_compare(
            build,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
            IrCondition::GreaterEqual,
        ),
        x if x == LBF::LBF_INTEGER_ULT as i32 => int2_compare(
            build,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
            IrCondition::UnsignedLess,
        ),
        x if x == LBF::LBF_INTEGER_ULE as i32 => int2_compare(
            build,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
            IrCondition::UnsignedLessEqual,
        ),
        x if x == LBF::LBF_INTEGER_UGT as i32 => int2_compare(
            build,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
            IrCondition::UnsignedGreater,
        ),
        x if x == LBF::LBF_INTEGER_UGE as i32 => int2_compare(
            build,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
            IrCondition::UnsignedGreaterEqual,
        ),
        x if x == LBF::LBF_INTEGER_BAND as i32 => int2_multiarg(
            build,
            IrCmd::BITAND_INT64,
            false,
            -1,
            nparams,
            ra,
            arg,
            args,
            arg3,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_INTEGER_BOR as i32 => int2_multiarg(
            build,
            IrCmd::BITOR_INT64,
            false,
            0,
            nparams,
            ra,
            arg,
            args,
            arg3,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_INTEGER_BXOR as i32 => int2_multiarg(
            build,
            IrCmd::BITXOR_INT64,
            false,
            0,
            nparams,
            ra,
            arg,
            args,
            arg3,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_INTEGER_BNOT as i32 => {
            if FFlag::LuauCodegenInteger2.get() {
                translate_builtin_int_64_bnot(build, nparams, ra, arg, nresults, pcpos)
            } else {
                no_builtin()
            }
        }
        x if x == LBF::LBF_INTEGER_BTEST as i32 => int2_multiarg(
            build,
            IrCmd::BITAND_INT64,
            true,
            -1,
            nparams,
            ra,
            arg,
            args,
            arg3,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_INTEGER_LSHIFT as i32 => int2_shift(
            build,
            IrCmd::BITLSHIFT_INT64,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_INTEGER_RSHIFT as i32 => int2_shift(
            build,
            IrCmd::BITRSHIFT_INT64,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_INTEGER_ARSHIFT as i32 => int2_shift(
            build,
            IrCmd::BITARSHIFT_INT64,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_INTEGER_LROTATE as i32 => int2_rotate(
            build,
            IrCmd::BITLROTATE_INT64,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_INTEGER_RROTATE as i32 => int2_rotate(
            build,
            IrCmd::BITRROTATE_INT64,
            nparams,
            ra,
            arg,
            args,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_INTEGER_COUNTLZ as i32 => int2_unary(
            build,
            IrCmd::BITCOUNTLZ_INT64,
            nparams,
            ra,
            arg,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_INTEGER_COUNTRZ as i32 => int2_unary(
            build,
            IrCmd::BITCOUNTRZ_INT64,
            nparams,
            ra,
            arg,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_INTEGER_BSWAP as i32 => int2_unary(
            build,
            IrCmd::BYTESWAP_INT64,
            nparams,
            ra,
            arg,
            nresults,
            pcpos,
        ),
        x if x == LBF::LBF_INTEGER_EXTRACT as i32 => {
            if FFlag::LuauCodegenInteger2.get() {
                translate_builtin_int_64_extract(
                    build, nparams, ra, arg, args, arg3, nresults, pcpos,
                )
            } else {
                no_builtin()
            }
        }
        _ => no_builtin(),
    }
}

fn int2_binary(
    build: &mut IrBuilder,
    nparams: i32,
    ra: i32,
    arg: i32,
    args: IrOp,
    nresults: i32,
    pcpos: i32,
    op: Int64Binary,
) -> BuiltinImplResult {
    if FFlag::LuauCodegenInteger2.get() {
        translate_builtin_int_64_binary(build, nparams, ra, arg, args, nresults, pcpos, op)
    } else {
        no_builtin()
    }
}

fn int2_compare(
    build: &mut IrBuilder,
    nparams: i32,
    ra: i32,
    arg: i32,
    args: IrOp,
    nresults: i32,
    pcpos: i32,
    cond: IrCondition,
) -> BuiltinImplResult {
    if FFlag::LuauCodegenInteger2.get() {
        translate_builtin_int_64_compare(build, nparams, ra, arg, args, nresults, pcpos, cond)
    } else {
        no_builtin()
    }
}

fn int2_multiarg(
    build: &mut IrBuilder,
    cmd: IrCmd,
    btest: bool,
    identity: i64,
    nparams: i32,
    ra: i32,
    arg: i32,
    args: IrOp,
    arg3: IrOp,
    nresults: i32,
    pcpos: i32,
) -> BuiltinImplResult {
    if FFlag::LuauCodegenInteger2.get() {
        translate_builtin_int_64_multiarg_op(
            build, cmd, btest, identity, nparams, ra, arg, args, arg3, nresults, pcpos,
        )
    } else {
        no_builtin()
    }
}

fn int2_shift(
    build: &mut IrBuilder,
    cmd: IrCmd,
    nparams: i32,
    ra: i32,
    arg: i32,
    args: IrOp,
    nresults: i32,
    pcpos: i32,
) -> BuiltinImplResult {
    if FFlag::LuauCodegenInteger2.get() {
        translate_builtin_int_64_shift(build, cmd, nparams, ra, arg, args, nresults, pcpos)
    } else {
        no_builtin()
    }
}

fn int2_rotate(
    build: &mut IrBuilder,
    cmd: IrCmd,
    nparams: i32,
    ra: i32,
    arg: i32,
    args: IrOp,
    nresults: i32,
    pcpos: i32,
) -> BuiltinImplResult {
    if FFlag::LuauCodegenInteger2.get() {
        translate_builtin_int_64_rotate(build, cmd, nparams, ra, arg, args, nresults, pcpos)
    } else {
        no_builtin()
    }
}

fn int2_unary(
    build: &mut IrBuilder,
    cmd: IrCmd,
    nparams: i32,
    ra: i32,
    arg: i32,
    nresults: i32,
    pcpos: i32,
) -> BuiltinImplResult {
    if FFlag::LuauCodegenInteger2.get() {
        translate_builtin_int_64_unary(build, cmd, nparams, ra, arg, nresults, pcpos)
    } else {
        no_builtin()
    }
}
