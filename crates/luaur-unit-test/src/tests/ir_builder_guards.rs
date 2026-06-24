#[cfg(test)]
#[test]
fn ir_builder_guards() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use luaur_code_gen::enums::include_use_info::IncludeUseInfo;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::functions::to_string_ir_dump_alt_g::to_string;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;

    const TNIL: u8 = 0;
    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();

    fix.with_one_block(|b, a| {
        let t1 = b.const_tag(TNUMBER);
        let t2 = b.const_tag(TNUMBER);
        b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TAG, t1, t2, a);
        let c0 = b.const_uint(0);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    });

    fix.with_one_block(|b, a| {
        let t1 = b.const_tag(TNIL);
        let t2 = b.const_tag(TNUMBER);
        b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TAG, t1, t2, a);
        let c0 = b.const_uint(0);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    });

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   RETURN 0u\n\nbb_2:\n   JUMP bb_3\n\nbb_3:\n   RETURN 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
