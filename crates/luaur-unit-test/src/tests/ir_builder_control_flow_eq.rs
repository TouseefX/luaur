#[cfg(test)]
#[test]
fn ir_builder_control_flow_eq() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use luaur_code_gen::enums::include_use_info::IncludeUseInfo;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::enums::ir_condition::IrCondition;
    use luaur_code_gen::functions::to_string_ir_dump_alt_g::to_string;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;

    const TNIL: u8 = 0;
    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();

    fix.with_two_blocks(|b, a, bb| {
        let t1 = b.const_tag(TNIL);
        let t2 = b.const_tag(TNIL);
        b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::JUMP_EQ_TAG, t1, t2, a, bb);
    });

    fix.with_two_blocks(|b, a, bb| {
        let t1 = b.const_tag(TNIL);
        let t2 = b.const_tag(TNUMBER);
        b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::JUMP_EQ_TAG, t1, t2, a, bb);
    });

    fix.with_two_blocks(|b, a, bb| {
        let c1 = b.const_int(0);
        let c2 = b.const_int(0);
        let cond = b.cond(IrCondition::Equal);
        b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(IrCmd::JUMP_CMP_INT, c1, c2, cond, a, bb);
    });

    fix.with_two_blocks(|b, a, bb| {
        let c1 = b.const_int(0);
        let c2 = b.const_int(1);
        let cond = b.cond(IrCondition::Equal);
        b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(IrCmd::JUMP_CMP_INT, c1, c2, cond, a, bb);
    });

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   JUMP bb_1\n\nbb_1:\n   RETURN 1u\n\nbb_3:\n   JUMP bb_5\n\nbb_5:\n   RETURN 2u\n\nbb_6:\n   JUMP bb_7\n\nbb_7:\n   RETURN 1u\n\nbb_9:\n   JUMP bb_11\n\nbb_11:\n   RETURN 2u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
