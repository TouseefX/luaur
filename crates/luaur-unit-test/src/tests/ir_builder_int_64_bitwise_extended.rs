#[cfg(test)]
#[test]
fn ir_builder_int_64_bitwise_extended() {
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
        macro_rules! un {
            ($reg:expr, $cmd:expr, $a:expr) => {{
                let ca = b.const_int_64($a);
                let op = b.inst_ir_cmd_ir_op($cmd, ca);
                let r = b.vm_reg($reg);
                b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT64, r, op);
            }};
        }

        un!(0, IrCmd::BITNOT_INT64, 0);
        un!(1, IrCmd::BITNOT_INT64, -1);
        bin!(2, IrCmd::BITAND_INT64, 0xABCD, 0xABCD);
        bin!(3, IrCmd::BITXOR_INT64, 0xABCD, 0xABCD);
        bin!(4, IrCmd::BITOR_INT64, 0xABCD, 0xABCD);
        un!(5, IrCmd::BITCOUNTLZ_INT64, 1);
        un!(6, IrCmd::BITCOUNTLZ_INT64, -1);
        un!(7, IrCmd::BITCOUNTRZ_INT64, 1);
        un!(8, IrCmd::BITCOUNTRZ_INT64, -1);
        un!(9, IrCmd::BITCOUNTRZ_INT64, 1i64 << 32);
        un!(10, IrCmd::BYTESWAP_INT64, 0);
        un!(11, IrCmd::BYTESWAP_INT64, -1);

        let c0 = b.const_uint(0);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT64 R0, -1i\n   STORE_INT64 R1, 0i\n   STORE_INT64 R2, 43981i\n   STORE_INT64 R3, 0i\n   STORE_INT64 R4, 43981i\n   STORE_INT64 R5, 63i\n   STORE_INT64 R6, 0i\n   STORE_INT64 R7, 0i\n   STORE_INT64 R8, 0i\n   STORE_INT64 R9, 32i\n   STORE_INT64 R10, 0i\n   STORE_INT64 R11, -1i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
