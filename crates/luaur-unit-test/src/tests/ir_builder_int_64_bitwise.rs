#[cfg(test)]
#[test]
fn ir_builder_int_64_bitwise() {
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

        let r0 = b.vm_reg(0);
        let unk = b.inst_ir_cmd_ir_op(IrCmd::LOAD_INT64, r0);

        macro_rules! bin {
            ($reg:expr, $cmd:expr, ($($a:tt)+), ($($bb:tt)+)) => {{
                let ca = bin!(@op $($a)+);
                let cb = bin!(@op $($bb)+);
                let op = b.inst_ir_cmd_ir_op_ir_op($cmd, ca, cb);
                let r = b.vm_reg($reg);
                b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT64, r, op);
            }};
            (@op unk) => { unk };
            (@op $v:expr) => { b.const_int_64($v) };
        }

        bin!(0, IrCmd::BITAND_INT64, (0xFE), (0x0E));
        bin!(1, IrCmd::BITAND_INT64, (unk), (0));
        bin!(2, IrCmd::BITAND_INT64, (0), (unk));
        bin!(3, IrCmd::BITAND_INT64, (unk), (-1));
        bin!(4, IrCmd::BITAND_INT64, (-1), (unk));
        bin!(5, IrCmd::BITXOR_INT64, (0xFE), (0x0E));
        bin!(6, IrCmd::BITXOR_INT64, (unk), (0));
        bin!(7, IrCmd::BITXOR_INT64, (0), (unk));
        bin!(8, IrCmd::BITOR_INT64, (0xF0), (0x0E));
        bin!(9, IrCmd::BITOR_INT64, (unk), (0));
        bin!(10, IrCmd::BITOR_INT64, (0), (unk));
        bin!(11, IrCmd::BITOR_INT64, (unk), (-1));
        bin!(12, IrCmd::BITOR_INT64, (-1), (unk));

        let c = b.const_int_64(0x0E);
        let op = b.inst_ir_cmd_ir_op(IrCmd::BITNOT_INT64, c);
        let r = b.vm_reg(13);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT64, r, op);

        let c0 = b.const_uint(0);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_INT64 R0\n   STORE_INT64 R0, 14i\n   STORE_INT64 R1, 0i\n   STORE_INT64 R2, 0i\n   STORE_INT64 R3, %0\n   STORE_INT64 R4, %0\n   STORE_INT64 R5, 240i\n   STORE_INT64 R6, %0\n   STORE_INT64 R7, %0\n   STORE_INT64 R8, 254i\n   STORE_INT64 R9, %0\n   STORE_INT64 R10, %0\n   STORE_INT64 R11, -1i\n   STORE_INT64 R12, -1i\n   STORE_INT64 R13, -15i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
