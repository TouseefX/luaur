#[cfg(test)]
#[test]
fn ir_builder_int_64_division_unsafe_cases_not_folded() {
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

        bin!(0, IrCmd::DIV_INT64, 42, 0);
        bin!(1, IrCmd::DIV_INT64, i64::MIN, -1);
        bin!(2, IrCmd::IDIV_INT64, 42, 0);
        bin!(3, IrCmd::REM_INT64, 42, 0);
        bin!(4, IrCmd::REM_INT64, i64::MIN, -1);
        bin!(5, IrCmd::UDIV_INT64, 42, 0);
        bin!(6, IrCmd::MOD_INT64, i64::MIN, -1);

        let c0 = b.const_uint(0);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = DIV_INT64 42i, 0i\n   STORE_INT64 R0, %0\n   %2 = DIV_INT64 -9223372036854775808i, -1i\n   STORE_INT64 R1, %2\n   %4 = IDIV_INT64 42i, 0i\n   STORE_INT64 R2, %4\n   %6 = REM_INT64 42i, 0i\n   STORE_INT64 R3, %6\n   %8 = REM_INT64 -9223372036854775808i, -1i\n   STORE_INT64 R4, %8\n   %10 = UDIV_INT64 42i, 0i\n   STORE_INT64 R5, %10\n   %12 = MOD_INT64 -9223372036854775808i, -1i\n   STORE_INT64 R6, %12\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
