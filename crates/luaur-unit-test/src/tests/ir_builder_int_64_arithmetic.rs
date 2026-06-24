#[cfg(test)]
#[test]
fn ir_builder_int_64_arithmetic() {
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

        macro_rules! bin {
            ($reg:expr, $cmd:expr, $a:expr, $bb:expr) => {{
                let ca = b.const_int_64($a);
                let cb = b.const_int_64($bb);
                let op = b.inst_ir_cmd_ir_op_ir_op($cmd, ca, cb);
                let r = b.vm_reg($reg);
                b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT64, r, op);
            }};
        }

        bin!(0, IrCmd::ADD_INT64, 10, 20);
        bin!(1, IrCmd::ADD_INT64, i64::MAX, 1);
        bin!(2, IrCmd::SUB_INT64, 10, 20);
        bin!(3, IrCmd::SUB_INT64, i64::MIN, 1);
        bin!(4, IrCmd::MUL_INT64, 6, 7);

        let c0 = b.const_uint(0);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT64 R0, 30i\n   STORE_INT64 R1, -9223372036854775808i\n   STORE_INT64 R2, -10i\n   STORE_INT64 R3, 9223372036854775807i\n   STORE_INT64 R4, 42i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
