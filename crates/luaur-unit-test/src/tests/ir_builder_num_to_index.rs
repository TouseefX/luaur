#[cfg(test)]
#[test]
fn ir_builder_num_to_index() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use luaur_code_gen::enums::include_use_info::IncludeUseInfo;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::functions::to_string_ir_dump_alt_g::to_string;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;

    let mut fix = IrBuilderFixture::new();

    fix.with_one_block(|b, a| {
        let c = b.const_double(4.0);
        let op = b.inst_ir_cmd_ir_op_ir_op(IrCmd::TRY_NUM_TO_INDEX, c, a);
        let r = b.vm_reg(0);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, r, op);
        let c0 = b.const_uint(0);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    });

    fix.with_one_block(|b, a| {
        let c = b.const_double(1.2);
        let op = b.inst_ir_cmd_ir_op_ir_op(IrCmd::TRY_NUM_TO_INDEX, c, a);
        let r = b.vm_reg(0);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, r, op);
        let c0 = b.const_uint(0);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    });

    fix.with_one_block(|b, a| {
        let z1 = b.const_double(0.0);
        let z2 = b.const_double(0.0);
        let nan = b.inst_ir_cmd_ir_op_ir_op(IrCmd::DIV_NUM, z1, z2);
        let op = b.inst_ir_cmd_ir_op_ir_op(IrCmd::TRY_NUM_TO_INDEX, nan, a);
        let r = b.vm_reg(0);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, r, op);
        let c0 = b.const_uint(0);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    });

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT R0, 4i\n   RETURN 0u\n\nbb_2:\n   JUMP bb_3\n\nbb_3:\n   RETURN 1u\n\nbb_4:\n   JUMP bb_5\n\nbb_5:\n   RETURN 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
