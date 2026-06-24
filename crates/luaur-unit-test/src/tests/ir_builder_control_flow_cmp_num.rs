#[cfg(test)]
#[test]
fn ir_builder_control_flow_cmp_num() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::enums::ir_condition::IrCondition;
    use luaur_code_gen::enums::ir_op_kind::IrOpKind;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;
    use luaur_code_gen::records::ir_inst::IrInst;
    use luaur_code_gen::records::ir_op::IrOp;

    // A `None`-kind op signals a placement of a freshly-built `nan` inside the
    // block (matching the C++ `lhs.kind == IrOpKind::None ? nan : lhs`).
    fn compare_fold(
        fix: &mut IrBuilderFixture,
        lhs: IrOp,
        rhs: IrOp,
        cond: IrCondition,
        result: bool,
    ) {
        let mut inst_op = IrOp::default();
        let mut expected_target = IrOp::default();

        fix.with_two_blocks(|b, a, bb| {
            let z1 = b.const_double(0.0);
            let z2 = b.const_double(0.0);
            let nan = b.inst_ir_cmd_ir_op_ir_op(IrCmd::DIV_NUM, z1, z2);
            let l = if lhs.kind() == IrOpKind::None {
                nan
            } else {
                lhs
            };
            let r = if rhs.kind() == IrOpKind::None {
                nan
            } else {
                rhs
            };
            let c = b.cond(cond);
            inst_op =
                b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(IrCmd::JUMP_CMP_NUM, l, r, c, a, bb);
            expected_target = if result { a } else { bb };
        });

        update_use_counts(&mut fix.build.function);
        fix.constant_fold();

        let expected = IrInst::ir_inst_new(IrCmd::JUMP, &[expected_target]);
        fix.check_eq(inst_op, &expected);
    }

    let mut fix = IrBuilderFixture::new();
    let nan = IrOp::default();

    macro_rules! cfd {
        ($a:expr, $bv:expr, $cond:expr, $res:expr) => {{
            let lhs = fix.build.const_double($a);
            let rhs = fix.build.const_double($bv);
            compare_fold(&mut fix, lhs, rhs, $cond, $res);
        }};
    }

    cfd!(1.0, 1.0, IrCondition::Equal, true);
    cfd!(1.0, 2.0, IrCondition::Equal, false);
    compare_fold(&mut fix, nan, nan, IrCondition::Equal, false);

    cfd!(1.0, 1.0, IrCondition::NotEqual, false);
    cfd!(1.0, 2.0, IrCondition::NotEqual, true);
    compare_fold(&mut fix, nan, nan, IrCondition::NotEqual, true);

    cfd!(1.0, 1.0, IrCondition::Less, false);
    cfd!(1.0, 2.0, IrCondition::Less, true);
    cfd!(2.0, 1.0, IrCondition::Less, false);
    {
        let lhs = fix.build.const_double(1.0);
        compare_fold(&mut fix, lhs, nan, IrCondition::Less, false);
    }

    cfd!(1.0, 1.0, IrCondition::NotLess, true);
    cfd!(1.0, 2.0, IrCondition::NotLess, false);
    cfd!(2.0, 1.0, IrCondition::NotLess, true);
    {
        let lhs = fix.build.const_double(1.0);
        compare_fold(&mut fix, lhs, nan, IrCondition::NotLess, true);
    }

    cfd!(1.0, 1.0, IrCondition::LessEqual, true);
    cfd!(1.0, 2.0, IrCondition::LessEqual, true);
    cfd!(2.0, 1.0, IrCondition::LessEqual, false);
    {
        let lhs = fix.build.const_double(1.0);
        compare_fold(&mut fix, lhs, nan, IrCondition::LessEqual, false);
    }

    cfd!(1.0, 1.0, IrCondition::NotLessEqual, false);
    cfd!(1.0, 2.0, IrCondition::NotLessEqual, false);
    cfd!(2.0, 1.0, IrCondition::NotLessEqual, true);
    {
        let lhs = fix.build.const_double(1.0);
        compare_fold(&mut fix, lhs, nan, IrCondition::NotLessEqual, true);
    }

    cfd!(1.0, 1.0, IrCondition::Greater, false);
    cfd!(1.0, 2.0, IrCondition::Greater, false);
    cfd!(2.0, 1.0, IrCondition::Greater, true);
    {
        let lhs = fix.build.const_double(1.0);
        compare_fold(&mut fix, lhs, nan, IrCondition::Greater, false);
    }

    cfd!(1.0, 1.0, IrCondition::NotGreater, true);
    cfd!(1.0, 2.0, IrCondition::NotGreater, true);
    cfd!(2.0, 1.0, IrCondition::NotGreater, false);
    {
        let lhs = fix.build.const_double(1.0);
        compare_fold(&mut fix, lhs, nan, IrCondition::NotGreater, true);
    }

    cfd!(1.0, 1.0, IrCondition::GreaterEqual, true);
    cfd!(1.0, 2.0, IrCondition::GreaterEqual, false);
    cfd!(2.0, 1.0, IrCondition::GreaterEqual, true);
    {
        let lhs = fix.build.const_double(1.0);
        compare_fold(&mut fix, lhs, nan, IrCondition::GreaterEqual, false);
    }

    cfd!(1.0, 1.0, IrCondition::NotGreaterEqual, false);
    cfd!(1.0, 2.0, IrCondition::NotGreaterEqual, true);
    cfd!(2.0, 1.0, IrCondition::NotGreaterEqual, false);
    {
        let lhs = fix.build.const_double(1.0);
        compare_fold(&mut fix, lhs, nan, IrCondition::NotGreaterEqual, true);
    }
}
