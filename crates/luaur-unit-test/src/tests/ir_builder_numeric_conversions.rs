#[cfg(test)]
#[test]
fn ir_builder_numeric_conversions() {
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

        let c8 = b.const_int(8);
        let op = b.inst_ir_cmd_ir_op(IrCmd::INT_TO_NUM, c8);
        let r = b.vm_reg(0);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r, op);

        let c = b.const_int(0xdeee0000u32 as i32);
        let op = b.inst_ir_cmd_ir_op(IrCmd::UINT_TO_NUM, c);
        let r = b.vm_reg(1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r, op);

        let c = b.const_double(200.0);
        let op = b.inst_ir_cmd_ir_op(IrCmd::NUM_TO_INT, c);
        let r = b.vm_reg(2);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, r, op);

        let c = b.const_double(3740139520.0);
        let op = b.inst_ir_cmd_ir_op(IrCmd::NUM_TO_UINT, c);
        let r = b.vm_reg(3);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, r, op);

        let c = b.const_double(-10.0);
        let op = b.inst_ir_cmd_ir_op(IrCmd::NUM_TO_UINT, c);
        let r = b.vm_reg(4);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, r, op);

        let c = b.const_double(-12345678901234.0);
        let op = b.inst_ir_cmd_ir_op(IrCmd::NUM_TO_UINT, c);
        let r = b.vm_reg(5);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, r, op);

        let c0 = b.const_uint(0);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_DOUBLE R0, 8\n   STORE_DOUBLE R1, 3740139520\n   STORE_INT R2, 200i\n   STORE_INT R3, -554827776i\n   STORE_INT R4, -10i\n   STORE_INT R5, -1942892530i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
