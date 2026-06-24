#[cfg(test)]
#[test]
fn ir_builder_control_flow_cmp_int() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::enums::ir_condition::IrCondition;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;
    use luaur_code_gen::records::ir_inst::IrInst;
    use luaur_code_gen::records::ir_op::IrOp;

    // lhs/rhs are concrete const ops created against `fix.build` before the call.
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
            let c = b.cond(cond);
            inst_op = b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
                IrCmd::JUMP_CMP_INT,
                lhs,
                rhs,
                c,
                a,
                bb,
            );
            expected_target = if result { a } else { bb };
        });

        update_use_counts(&mut fix.build.function);
        fix.constant_fold();

        let expected = IrInst::ir_inst_new(IrCmd::JUMP, &[expected_target]);
        fix.check_eq(inst_op, &expected);
    }

    let mut fix = IrBuilderFixture::new();

    macro_rules! cf {
        ($a:expr, $bv:expr, $cond:expr, $res:expr) => {{
            let lhs = fix.build.const_int($a);
            let rhs = fix.build.const_int($bv);
            compare_fold(&mut fix, lhs, rhs, $cond, $res);
        }};
    }

    cf!(1, 1, IrCondition::Equal, true);
    cf!(1, 2, IrCondition::Equal, false);

    cf!(1, 1, IrCondition::NotEqual, false);
    cf!(1, 2, IrCondition::NotEqual, true);

    cf!(1, 1, IrCondition::Less, false);
    cf!(1, 2, IrCondition::Less, true);
    cf!(2, 1, IrCondition::Less, false);

    cf!(1, 1, IrCondition::NotLess, true);
    cf!(1, 2, IrCondition::NotLess, false);
    cf!(2, 1, IrCondition::NotLess, true);

    cf!(1, 1, IrCondition::LessEqual, true);
    cf!(1, 2, IrCondition::LessEqual, true);
    cf!(2, 1, IrCondition::LessEqual, false);

    cf!(1, 1, IrCondition::NotLessEqual, false);
    cf!(1, 2, IrCondition::NotLessEqual, false);
    cf!(2, 1, IrCondition::NotLessEqual, true);

    cf!(1, 1, IrCondition::Greater, false);
    cf!(1, 2, IrCondition::Greater, false);
    cf!(2, 1, IrCondition::Greater, true);

    cf!(1, 1, IrCondition::NotGreater, true);
    cf!(1, 2, IrCondition::NotGreater, true);
    cf!(2, 1, IrCondition::NotGreater, false);

    cf!(1, 1, IrCondition::GreaterEqual, true);
    cf!(1, 2, IrCondition::GreaterEqual, false);
    cf!(2, 1, IrCondition::GreaterEqual, true);

    cf!(1, 1, IrCondition::NotGreaterEqual, false);
    cf!(1, 2, IrCondition::NotGreaterEqual, true);
    cf!(2, 1, IrCondition::NotGreaterEqual, false);
}
