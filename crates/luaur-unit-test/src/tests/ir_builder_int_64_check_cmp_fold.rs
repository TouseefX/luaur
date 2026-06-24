#[cfg(test)]
#[test]
fn ir_builder_int_64_check_cmp_fold() {
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

        macro_rules! chk {
            ($a:expr, $bb:expr, $cond:expr) => {{
                let ca = b.const_int_64($a);
                let cb = b.const_int_64($bb);
                let cc = b.cond($cond);
                b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::CHECK_CMP_INT64, ca, cb, cc, fallback);
            }};
        }

        chk!(0, 0, IrCondition::NotEqual);
        chk!(10, 20, IrCondition::Less);
        chk!(5, 0, IrCondition::NotEqual);

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
