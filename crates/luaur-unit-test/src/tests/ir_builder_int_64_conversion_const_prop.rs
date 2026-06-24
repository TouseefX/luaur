#[cfg(test)]
#[test]
fn ir_builder_int_64_conversion_const_prop() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use luaur_code_gen::enums::include_use_info::IncludeUseInfo;
    use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::functions::const_prop_in_block_chains::const_prop_in_block_chains;
    use luaur_code_gen::functions::to_string_ir_dump_alt_g::to_string;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;

    let mut fix = IrBuilderFixture::new();
    {
        let b = &mut fix.build;
        let block = b.block(IrBlockKind::Internal);
        b.begin_block(block);

        let r0 = b.vm_reg(0);
        let val = b.inst_ir_cmd_ir_op(IrCmd::LOAD_INT64, r0);
        let as_num1 = b.inst_ir_cmd_ir_op(IrCmd::INT64_TO_NUM, val);
        let as_num2 = b.inst_ir_cmd_ir_op(IrCmd::INT64_TO_NUM, val);
        let r1 = b.vm_reg(1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r1, as_num1);
        let r2 = b.vm_reg(2);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r2, as_num2);

        let c0 = b.const_uint(0);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_INT64 R0\n   %1 = INT64_TO_NUM %0\n   STORE_DOUBLE R1, %1\n   STORE_DOUBLE R2, %1\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
