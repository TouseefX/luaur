#[cfg(test)]
#[test]
fn ir_builder_int_64_conversion_const_fold() {
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

        macro_rules! i2n {
            ($reg:expr, $a:expr) => {{
                let c = b.const_int_64($a);
                let op = b.inst_ir_cmd_ir_op(IrCmd::INT64_TO_NUM, c);
                let r = b.vm_reg($reg);
                b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r, op);
            }};
        }
        macro_rules! n2i {
            ($reg:expr, $a:expr) => {{
                let c = b.const_double($a);
                let op = b.inst_ir_cmd_ir_op(IrCmd::NUM_TO_INT64, c);
                let r = b.vm_reg($reg);
                b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT64, r, op);
            }};
        }

        i2n!(0, 42);
        i2n!(1, 0);
        i2n!(2, -100);
        n2i!(3, 42.0);
        n2i!(4, 0.0);
        n2i!(5, -100.0);
        n2i!(6, 3.7);

        let c0 = b.const_uint(0);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_DOUBLE R0, 42\n   STORE_DOUBLE R1, 0\n   STORE_DOUBLE R2, -100\n   STORE_INT64 R3, 42i\n   STORE_INT64 R4, 0i\n   STORE_INT64 R5, -100i\n   STORE_INT64 R6, 3i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
