//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:3553:ir_builder_cmp_split_tag_value_simplification`
//! Source: `tests/IrBuilder.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/IrBuilder.test.cpp
//! - source_includes:
//!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
//!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
//!   - includes -> source_file CodeGen/include/Luau/IrDump.h
//!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
//!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
//!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
//!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/IrBuilder.test.cpp
//! - outgoing:
//!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
//!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
//!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
//!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
//!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
//!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
//!   - type_ref -> enum IrCondition (CodeGen/include/Luau/IrData.h)
//!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
//!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
//!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
//!   - translates_to -> rust_item ir_builder_cmp_split_tag_value_simplification

#[cfg(test)]
#[test]
fn ir_builder_cmp_split_tag_value_simplification() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use luaur_code_gen::enums::include_use_info::IncludeUseInfo;
    use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::enums::ir_condition::IrCondition;
    use luaur_code_gen::functions::const_prop_in_block_chains::const_prop_in_block_chains;
    use luaur_code_gen::functions::to_string_ir_dump_alt_g::to_string;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;

    const TNIL: u8 = 0;
    const TBOOLEAN: u8 = 1;
    const TNUMBER: u8 = 3;

    macro_rules! store_cmp_split {
        ($b:ident, $reg:expr, $tag_a:expr, $tag_b:expr, $value_a:expr, $value_b:expr, $cond:expr) => {{
            let cmp = $b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
                IrCmd::CMP_SPLIT_TVALUE,
                $tag_a,
                $tag_b,
                $value_a,
                $value_b,
                $cond,
            );
            let reg = $b.vm_reg($reg);
            $b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, reg, cmp);
        }};
    }

    let mut fix = IrBuilderFixture::new();
    {
        let b = &mut fix.build;
        let block = b.block(IrBlockKind::Internal);

        b.begin_block(block);
        let r1 = b.vm_reg(1);
        let value_boolean = b.inst_ir_cmd_ir_op(IrCmd::LOAD_INT, r1);
        let zero = b.const_double(0.0);
        let zero2 = b.const_double(0.0);
        let value_nan = b.inst_ir_cmd_ir_op_ir_op(IrCmd::DIV_NUM, zero, zero2);

        let r0 = b.vm_reg(0);
        let op_tag = b.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, r0);
        let op_nil = b.const_tag(TNIL);
        let op_bool = b.const_tag(TBOOLEAN);
        let op_num = b.const_tag(TNUMBER);

        let eq = b.cond(IrCondition::Equal);
        let neq = b.cond(IrCondition::NotEqual);

        let zero = b.const_int(0);
        store_cmp_split!(b, 2, op_nil, op_bool, zero, value_boolean, eq);
        let zero1 = b.const_int(0);
        let zero2 = b.const_int(0);
        store_cmp_split!(b, 3, op_bool, op_bool, zero1, zero2, eq);
        let zero = b.const_int(0);
        let one = b.const_int(1);
        store_cmp_split!(b, 4, op_bool, op_bool, zero, one, eq);
        let zero1 = b.const_double(0.0);
        let zero2 = b.const_double(0.0);
        store_cmp_split!(b, 5, op_num, op_num, zero1, zero2, eq);
        let zero = b.const_double(0.0);
        let one = b.const_double(1.0);
        store_cmp_split!(b, 6, op_num, op_num, zero, one, eq);

        let zero = b.const_int(0);
        store_cmp_split!(b, 7, op_nil, op_bool, zero, value_boolean, neq);
        let zero1 = b.const_int(0);
        let zero2 = b.const_int(0);
        store_cmp_split!(b, 8, op_bool, op_bool, zero1, zero2, neq);
        let zero = b.const_int(0);
        let one = b.const_int(1);
        store_cmp_split!(b, 9, op_bool, op_bool, zero, one, neq);
        let zero1 = b.const_double(0.0);
        let zero2 = b.const_double(0.0);
        store_cmp_split!(b, 10, op_num, op_num, zero1, zero2, neq);
        let zero = b.const_double(0.0);
        let one = b.const_double(1.0);
        store_cmp_split!(b, 11, op_num, op_num, zero, one, neq);

        let zero1 = b.const_int(0);
        let zero2 = b.const_int(0);
        store_cmp_split!(b, 12, op_tag, op_bool, zero1, zero2, eq);
        let zero = b.const_int(0);
        let one = b.const_int(1);
        store_cmp_split!(b, 13, op_tag, op_bool, zero, one, eq);
        let zero1 = b.const_double(0.0);
        let zero2 = b.const_double(0.0);
        store_cmp_split!(b, 14, op_tag, op_num, zero1, zero2, eq);
        let zero = b.const_double(0.0);
        let one = b.const_double(1.0);
        store_cmp_split!(b, 15, op_tag, op_num, zero, one, eq);

        let zero1 = b.const_int(0);
        let zero2 = b.const_int(0);
        store_cmp_split!(b, 16, op_tag, op_bool, zero1, zero2, neq);
        let zero = b.const_int(0);
        let one = b.const_int(1);
        store_cmp_split!(b, 17, op_tag, op_bool, zero, one, neq);
        let zero1 = b.const_double(0.0);
        let zero2 = b.const_double(0.0);
        store_cmp_split!(b, 18, op_tag, op_num, zero1, zero2, neq);
        let zero = b.const_double(0.0);
        let one = b.const_double(1.0);
        store_cmp_split!(b, 19, op_tag, op_num, zero, one, neq);

        store_cmp_split!(b, 20, op_num, op_num, value_nan, value_nan, eq);
        store_cmp_split!(b, 21, op_num, op_num, value_nan, value_nan, neq);
        store_cmp_split!(b, 22, op_tag, op_num, value_nan, value_nan, eq);
        store_cmp_split!(b, 23, op_tag, op_num, value_nan, value_nan, neq);

        let r2 = b.vm_reg(2);
        let twenty_one = b.const_int(21);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r2, twenty_one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %2 = LOAD_TAG R0\n   STORE_INT R2, 0i\n   STORE_INT R3, 1i\n   STORE_INT R4, 0i\n   STORE_INT R5, 1i\n   STORE_INT R6, 0i\n   STORE_INT R7, 1i\n   STORE_INT R8, 0i\n   STORE_INT R9, 1i\n   STORE_INT R10, 0i\n   STORE_INT R11, 1i\n   %23 = CMP_TAG %2, tboolean, eq\n   STORE_INT R12, %23\n   STORE_INT R13, 0i\n   %27 = CMP_TAG %2, tnumber, eq\n   STORE_INT R14, %27\n   STORE_INT R15, 0i\n   %31 = CMP_TAG %2, tboolean, not_eq\n   STORE_INT R16, %31\n   STORE_INT R17, 1i\n   %35 = CMP_TAG %2, tnumber, not_eq\n   STORE_INT R18, %35\n   STORE_INT R19, 1i\n   STORE_INT R20, 0i\n   STORE_INT R21, 1i\n   STORE_INT R22, 0i\n   STORE_INT R23, 1i\n   RETURN R2, 21i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
