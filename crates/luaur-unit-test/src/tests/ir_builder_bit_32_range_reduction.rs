#[cfg(test)]
#[test]
fn ir_builder_bit_32_range_reduction() {
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
            ($cmd:expr, $a:expr, $bb:expr) => {{
                let ca = b.const_int($a);
                let cb = b.const_int($bb);
                let op = b.inst_ir_cmd_ir_op_ir_op($cmd, ca, cb);
                let r = b.vm_reg(10);
                b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, r, op);
            }};
        }

        bin!(IrCmd::BITLSHIFT_UINT, 0xf, -10);
        bin!(IrCmd::BITLSHIFT_UINT, 0xf, 140);
        bin!(IrCmd::BITRSHIFT_UINT, 0xffffff, -10);
        bin!(IrCmd::BITRSHIFT_UINT, 0xffffff, 140);
        bin!(IrCmd::BITARSHIFT_UINT, 0xffffff, -10);
        bin!(IrCmd::BITARSHIFT_UINT, 0xffffff, 140);

        let c0 = b.const_uint(0);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT R10, 62914560i\n   STORE_INT R10, 61440i\n   STORE_INT R10, 3i\n   STORE_INT R10, 4095i\n   STORE_INT R10, 3i\n   STORE_INT R10, 4095i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
