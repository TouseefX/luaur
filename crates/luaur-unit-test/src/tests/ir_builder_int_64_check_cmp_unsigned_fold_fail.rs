#[cfg(test)]
#[test]
fn ir_builder_int_64_check_cmp_unsigned_fold_fail() {
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
        let fallback = b.block(IrBlockKind::Internal);

        b.begin_block(block);

        let c0c = b.const_int_64(0);
        let cm1 = b.const_int_64(-1);
        let cc = b.cond(IrCondition::UnsignedGreater);
        b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::CHECK_CMP_INT64, c0c, cm1, cc, fallback);
        let c1c = b.const_int_64(1);
        let r = b.vm_reg(0);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT64, r, c1c);
        let c0 = b.const_uint(0);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);

        b.begin_block(fallback);
        let c1 = b.const_uint(1);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c1);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   JUMP bb_1\n\nbb_1:\n   RETURN 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
