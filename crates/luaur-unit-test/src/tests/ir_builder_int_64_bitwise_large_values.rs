#[cfg(test)]
#[test]
fn ir_builder_int_64_bitwise_large_values() {
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

        let hi_val = 0x8000000000000000u64 as i64; // INT64_MIN
        let hi_mask = 0xFFFFFFFF00000000u64 as i64;
        let lo_mask = 0x00000000FFFFFFFFu64 as i64;

        macro_rules! bin {
            ($reg:expr, $cmd:expr, $a:expr, $bb:expr) => {{
                let ca = b.const_int_64($a);
                let cb = b.const_int_64($bb);
                let op = b.inst_ir_cmd_ir_op_ir_op($cmd, ca, cb);
                let r = b.vm_reg($reg);
                b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT64, r, op);
            }};
        }
        macro_rules! un {
            ($reg:expr, $cmd:expr, $a:expr) => {{
                let ca = b.const_int_64($a);
                let op = b.inst_ir_cmd_ir_op($cmd, ca);
                let r = b.vm_reg($reg);
                b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT64, r, op);
            }};
        }

        bin!(0, IrCmd::BITAND_INT64, 0x123456789ABCDEF0i64, hi_mask);
        bin!(1, IrCmd::BITAND_INT64, 0x123456789ABCDEF0i64, lo_mask);
        bin!(2, IrCmd::BITXOR_INT64, hi_val, hi_val);
        bin!(
            3,
            IrCmd::BITOR_INT64,
            0xFF00000000000000u64 as i64,
            0x00000000000000FFi64
        );
        un!(4, IrCmd::BYTESWAP_INT64, 0x0123456789ABCDEFi64);
        bin!(5, IrCmd::BITLROTATE_INT64, 0x0000000100000002i64, 32);

        let c0 = b.const_uint(0);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT64 R0, 1311768464867721216i\n   STORE_INT64 R1, 2596069104i\n   STORE_INT64 R2, 0i\n   STORE_INT64 R3, -72057594037927681i\n   STORE_INT64 R4, -1167088121787636991i\n   STORE_INT64 R5, 8589934593i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
