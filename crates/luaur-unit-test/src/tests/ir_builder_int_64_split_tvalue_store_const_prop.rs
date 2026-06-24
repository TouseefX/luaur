#[cfg(test)]
#[test]
fn ir_builder_int_64_split_tvalue_store_const_prop() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use luaur_code_gen::enums::include_use_info::IncludeUseInfo;
    use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::functions::compute_cfg_info::compute_cfg_info;
    use luaur_code_gen::functions::const_prop_in_block_chains::const_prop_in_block_chains;
    use luaur_code_gen::functions::mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains;
    use luaur_code_gen::functions::to_string_ir_dump_alt_g::to_string;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;

    const TINTEGER: u8 = 4;

    let mut fix = IrBuilderFixture::new();
    {
        let b = &mut fix.build;
        let entry = b.block(IrBlockKind::Internal);
        b.begin_block(entry);

        let r0 = b.vm_reg(0);
        let tag = b.const_tag(TINTEGER);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r0, tag);
        let r0b = b.vm_reg(0);
        let c42 = b.const_int_64(42);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT64, r0b, c42);

        let r0c = b.vm_reg(0);
        let c1 = b.const_int(1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0c, c1);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected =
        "\nbb_0:\n   STORE_TAG R0, tinteger\n   STORE_INT64 R0, 42i\n   RETURN R0, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
