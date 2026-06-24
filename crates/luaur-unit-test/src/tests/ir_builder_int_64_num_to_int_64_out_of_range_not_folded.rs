#[cfg(test)]
#[test]
fn ir_builder_int_64_num_to_int_64_out_of_range_not_folded() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use luaur_code_gen::enums::include_use_info::IncludeUseInfo;
    use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::functions::to_string_ir_dump_alt_g::to_string;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;

    let mut fix = IrBuilderFixture::new();
    {
        let b = &mut fix.build;
        let block = b.block(IrBlockKind::Internal);
        b.begin_block(block);

        let c = b.const_double(1e19);
        let op = b.inst_ir_cmd_ir_op(IrCmd::NUM_TO_INT64, c);
        let r = b.vm_reg(0);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT64, r, op);

        let z1 = b.const_double(0.0);
        let z2 = b.const_double(0.0);
        let nan = b.inst_ir_cmd_ir_op_ir_op(IrCmd::DIV_NUM, z1, z2);
        let op = b.inst_ir_cmd_ir_op(IrCmd::NUM_TO_INT64, nan);
        let r = b.vm_reg(1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT64, r, op);

        let o1 = b.const_double(1.0);
        let o2 = b.const_double(0.0);
        let inf = b.inst_ir_cmd_ir_op_ir_op(IrCmd::DIV_NUM, o1, o2);
        let op = b.inst_ir_cmd_ir_op(IrCmd::NUM_TO_INT64, inf);
        let r = b.vm_reg(2);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT64, r, op);

        let c0 = b.const_uint(0);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = NUM_TO_INT64 1e+19\n   STORE_INT64 R0, %0\n   %3 = NUM_TO_INT64 nan\n   STORE_INT64 R1, %3\n   %6 = NUM_TO_INT64 inf\n   STORE_INT64 R2, %6\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
