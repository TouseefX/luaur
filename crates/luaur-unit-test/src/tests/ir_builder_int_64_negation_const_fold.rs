#[cfg(test)]
#[test]
fn ir_builder_int_64_negation_const_fold() {
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

        macro_rules! neg {
            ($reg:expr, $a:expr) => {{
                let ca = b.const_int_64(0);
                let cb = b.const_int_64($a);
                let op = b.inst_ir_cmd_ir_op_ir_op(IrCmd::SUB_INT64, ca, cb);
                let r = b.vm_reg($reg);
                b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT64, r, op);
            }};
        }

        neg!(0, 42);
        neg!(1, i64::MIN);
        neg!(2, 0);
        neg!(3, -1);

        let c0 = b.const_uint(0);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT64 R0, -42i\n   STORE_INT64 R1, -9223372036854775808i\n   STORE_INT64 R2, 0i\n   STORE_INT64 R3, 1i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
