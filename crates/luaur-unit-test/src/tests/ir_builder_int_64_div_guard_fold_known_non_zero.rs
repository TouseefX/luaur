#[cfg(test)]
#[test]
fn ir_builder_int_64_div_guard_fold_known_non_zero() {
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

        let r0 = b.vm_reg(0);
        let a = b.inst_ir_cmd_ir_op(IrCmd::LOAD_INT64, r0);

        let c7 = b.const_int_64(7);
        let c0c = b.const_int_64(0);
        let cc = b.cond(IrCondition::NotEqual);
        b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::CHECK_CMP_INT64, c7, c0c, cc, fallback);

        let c7b = b.const_int_64(7);
        let op = b.inst_ir_cmd_ir_op_ir_op(IrCmd::DIV_INT64, a, c7b);
        let r = b.vm_reg(1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT64, r, op);
        let c0 = b.const_uint(0);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);

        b.begin_block(fallback);
        let c1 = b.const_uint(1);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c1);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_INT64 R0\n   %2 = DIV_INT64 %0, 7i\n   STORE_INT64 R1, %2\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
