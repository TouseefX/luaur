#[cfg(test)]
#[test]
fn ir_builder_select_number() {
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

        let zero_num = b.const_double(0.0);
        let one_num = b.const_double(1.0);
        let r0 = b.vm_reg(0);
        let unknown_num = b.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, r0);

        let c4 = b.const_double(4.0);
        let c8 = b.const_double(8.0);
        let op =
            b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::SELECT_NUM, c4, c8, zero_num, zero_num);
        let r = b.vm_reg(0);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r, op);

        let c4 = b.const_double(4.0);
        let c8 = b.const_double(8.0);
        let op =
            b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::SELECT_NUM, c4, c8, zero_num, one_num);
        let r = b.vm_reg(0);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r, op);

        let c4a = b.const_double(4.0);
        let c4b = b.const_double(4.0);
        let op = b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
            IrCmd::SELECT_NUM,
            c4a,
            c4b,
            zero_num,
            unknown_num,
        );
        let r = b.vm_reg(0);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r, op);

        let c0 = b.const_uint(0);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_DOUBLE R0, 8\n   STORE_DOUBLE R0, 4\n   STORE_DOUBLE R0, 4\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
