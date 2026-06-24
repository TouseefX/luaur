#[cfg(test)]
#[test]
fn ir_builder_int_64_shift_boundary_63() {
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

        bin!(0, IrCmd::BITLSHIFT_INT64, 1, 63);
        bin!(1, IrCmd::BITRSHIFT_INT64, i64::MIN, 63);
        bin!(2, IrCmd::BITARSHIFT_INT64, -1, 63);
        bin!(3, IrCmd::BITARSHIFT_INT64, i64::MAX, 63);
        bin!(4, IrCmd::BITLSHIFT_INT64, i64::MIN, -63);
        bin!(5, IrCmd::BITRSHIFT_INT64, 1, -63);

        let c0 = b.const_uint(0);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT64 R0, -9223372036854775808i\n   STORE_INT64 R1, 1i\n   STORE_INT64 R2, -1i\n   STORE_INT64 R3, 0i\n   STORE_INT64 R4, 1i\n   STORE_INT64 R5, -9223372036854775808i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
