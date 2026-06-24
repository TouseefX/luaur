#[cfg(test)]
#[test]
fn ir_builder_check_cmp_num_na_n() {
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

        let z1 = b.const_double(0.0);
        let z2 = b.const_double(0.0);
        let nan = b.inst_ir_cmd_ir_op_ir_op(IrCmd::DIV_NUM, z1, z2);
        let c1 = b.const_double(1.0);
        let cond = b.cond(IrCondition::Equal);
        b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::CHECK_CMP_NUM, nan, c1, cond, fallback);
        let cd = b.const_double(42.0);
        let r = b.vm_reg(0);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r, cd);
        let c0 = b.const_uint(0);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);

        b.begin_block(fallback);
        let c1u = b.const_uint(1);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c1u);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   JUMP bb_1\n\nbb_1:\n   RETURN 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
