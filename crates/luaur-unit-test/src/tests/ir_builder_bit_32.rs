#[cfg(test)]
#[test]
fn ir_builder_bit_32() {
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
        let unk = b.inst_ir_cmd_ir_op(IrCmd::LOAD_INT, r0);

        // Binary const-const / identity folds: STORE_INT R{reg}, CMD(a, b)
        macro_rules! bin {
            ($reg:expr, $cmd:expr, ($($a:tt)+), ($($bb:tt)+)) => {{
                let ca = bin!(@operand $($a)+);
                let cb = bin!(@operand $($bb)+);
                let op = b.inst_ir_cmd_ir_op_ir_op($cmd, ca, cb);
                let r = b.vm_reg($reg);
                b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, r, op);
            }};
            (@operand unk) => { unk };
            (@operand $v:expr) => { b.const_int($v) };
        }
        macro_rules! un {
            ($reg:expr, $cmd:expr, $a:expr) => {{
                let ca = b.const_int($a);
                let op = b.inst_ir_cmd_ir_op($cmd, ca);
                let r = b.vm_reg($reg);
                b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, r, op);
            }};
        }

        let all_ones = !0u32 as i32;

        bin!(0, IrCmd::BITAND_UINT, (0xfe), (0xe));
        bin!(1, IrCmd::BITAND_UINT, (unk), (0));
        bin!(2, IrCmd::BITAND_UINT, (0), (unk));
        bin!(3, IrCmd::BITAND_UINT, (unk), (all_ones));
        bin!(4, IrCmd::BITAND_UINT, (all_ones), (unk));
        bin!(5, IrCmd::BITXOR_UINT, (0xfe), (0xe));
        bin!(6, IrCmd::BITXOR_UINT, (unk), (0));
        bin!(7, IrCmd::BITXOR_UINT, (0), (unk));
        bin!(8, IrCmd::BITXOR_UINT, (unk), (all_ones));
        bin!(9, IrCmd::BITXOR_UINT, (all_ones), (unk));
        bin!(10, IrCmd::BITOR_UINT, (0xf0), (0xe));
        bin!(11, IrCmd::BITOR_UINT, (unk), (0));
        bin!(12, IrCmd::BITOR_UINT, (0), (unk));
        bin!(13, IrCmd::BITOR_UINT, (unk), (all_ones));
        bin!(14, IrCmd::BITOR_UINT, (all_ones), (unk));
        un!(15, IrCmd::BITNOT_UINT, 0xe);
        bin!(16, IrCmd::BITLSHIFT_UINT, (0xf0), (4));
        bin!(17, IrCmd::BITLSHIFT_UINT, (unk), (0));
        bin!(18, IrCmd::BITRSHIFT_UINT, (0xdeee0000u32 as i32), (8));
        bin!(19, IrCmd::BITRSHIFT_UINT, (unk), (0));
        bin!(20, IrCmd::BITARSHIFT_UINT, (0xdeee0000u32 as i32), (8));
        bin!(21, IrCmd::BITARSHIFT_UINT, (unk), (0));
        bin!(22, IrCmd::BITLROTATE_UINT, (0xdeee0000u32 as i32), (8));
        bin!(23, IrCmd::BITLROTATE_UINT, (unk), (0));
        bin!(24, IrCmd::BITRROTATE_UINT, (0xdeee0000u32 as i32), (8));
        bin!(25, IrCmd::BITRROTATE_UINT, (unk), (0));
        un!(26, IrCmd::BITCOUNTLZ_UINT, 0xff00);
        un!(27, IrCmd::BITCOUNTLZ_UINT, 0);
        un!(28, IrCmd::BITCOUNTRZ_UINT, 0xff00);
        un!(29, IrCmd::BITCOUNTRZ_UINT, 0);

        let c0 = b.const_uint(0);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_INT R0\n   STORE_INT R0, 14i\n   STORE_INT R1, 0i\n   STORE_INT R2, 0i\n   STORE_INT R3, %0\n   STORE_INT R4, %0\n   STORE_INT R5, 240i\n   STORE_INT R6, %0\n   STORE_INT R7, %0\n   %17 = BITNOT_UINT %0\n   STORE_INT R8, %17\n   %19 = BITNOT_UINT %0\n   STORE_INT R9, %19\n   STORE_INT R10, 254i\n   STORE_INT R11, %0\n   STORE_INT R12, %0\n   STORE_INT R13, -1i\n   STORE_INT R14, -1i\n   STORE_INT R15, -15i\n   STORE_INT R16, 3840i\n   STORE_INT R17, %0\n   STORE_INT R18, 14609920i\n   STORE_INT R19, %0\n   STORE_INT R20, -2167296i\n   STORE_INT R21, %0\n   STORE_INT R22, -301989666i\n   STORE_INT R23, %0\n   STORE_INT R24, 14609920i\n   STORE_INT R25, %0\n   STORE_INT R26, 16i\n   STORE_INT R27, 32i\n   STORE_INT R28, 8i\n   STORE_INT R29, 32i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
