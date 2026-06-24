#[cfg(test)]
#[test]
fn ir_builder_int_64_comparisons() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use luaur_code_gen::enums::include_use_info::IncludeUseInfo;
    use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::enums::ir_condition::IrCondition;
    use luaur_code_gen::functions::to_string_ir_dump_alt_g::to_string;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;

    let mut fix = IrBuilderFixture::new();
    {
        let b = &mut fix.build;
        let block = b.block(IrBlockKind::Internal);
        b.begin_block(block);

        macro_rules! cmp {
            ($reg:expr, $a:expr, $bb:expr, $cond:expr) => {{
                let ca = b.const_int_64($a);
                let cb = b.const_int_64($bb);
                let cc = b.cond($cond);
                let op = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CMP_INT64, ca, cb, cc);
                let r = b.vm_reg($reg);
                b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, r, op);
            }};
        }

        cmp!(0, 10, 20, IrCondition::Less);
        cmp!(1, 20, 10, IrCondition::Less);
        cmp!(2, 10, 10, IrCondition::Equal);
        cmp!(3, 10, 20, IrCondition::NotEqual);
        cmp!(4, -1, 0, IrCondition::Less);
        cmp!(5, i64::MIN, i64::MAX, IrCondition::Less);
        cmp!(6, -1, 0, IrCondition::UnsignedGreater);
        cmp!(7, -1, 0, IrCondition::UnsignedLess);
        cmp!(8, 10, 10, IrCondition::GreaterEqual);
        cmp!(9, 10, 10, IrCondition::LessEqual);

        let c0 = b.const_uint(0);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT R0, 1i\n   STORE_INT R1, 0i\n   STORE_INT R2, 1i\n   STORE_INT R3, 1i\n   STORE_INT R4, 1i\n   STORE_INT R5, 1i\n   STORE_INT R6, 1i\n   STORE_INT R7, 0i\n   STORE_INT R8, 1i\n   STORE_INT R9, 1i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
