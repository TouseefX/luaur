#[cfg(test)]
#[test]
fn ir_builder_int_64_select_preserved_with_different_branches() {
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

        let r0 = b.vm_reg(0);
        let a = b.inst_ir_cmd_ir_op(IrCmd::LOAD_INT64, r0);
        let r1 = b.vm_reg(1);
        let bb = b.inst_ir_cmd_ir_op(IrCmd::LOAD_INT64, r1);

        let cond1 = b.cond(IrCondition::Less);
        let op =
            b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(IrCmd::SELECT_INT64, a, bb, a, bb, cond1);
        let r = b.vm_reg(2);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT64, r, op);

        let cond2 = b.cond(IrCondition::LessEqual);
        let op =
            b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(IrCmd::SELECT_INT64, a, a, a, bb, cond2);
        let r = b.vm_reg(3);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT64, r, op);

        let c0 = b.const_uint(0);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_INT64 R0\n   %1 = LOAD_INT64 R1\n   %2 = SELECT_INT64 %0, %1, %0, %1, lt\n   STORE_INT64 R2, %2\n   %4 = SELECT_INT64 %0, %0, %0, %1, le\n   STORE_INT64 R3, %4\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
