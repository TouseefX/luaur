#[cfg(test)]
#[test]
fn ir_builder_dse_int_64_overwrite() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use luaur_code_gen::enums::include_use_info::IncludeUseInfo;
    use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::functions::compute_cfg_info::compute_cfg_info;
    use luaur_code_gen::functions::const_prop_in_block_chains::const_prop_in_block_chains;
    use luaur_code_gen::functions::mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains;
    use luaur_code_gen::functions::to_string_ir_dump_alt_g::to_string;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;

    const TNUMBER: u8 = 3;
    const TINTEGER: u8 = 4;

    let mut fix = IrBuilderFixture::new();
    {
        let b = &mut fix.build;
        let entry = b.block(IrBlockKind::Internal);
        b.begin_block(entry);

        let r0 = b.vm_reg(0);
        let val = b.inst_ir_cmd_ir_op(IrCmd::LOAD_INT64, r0);

        let r1 = b.vm_reg(1);
        let tnum = b.const_tag(TNUMBER);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r1, tnum);
        let r1b = b.vm_reg(1);
        let d1 = b.const_double(1.0);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r1b, d1);

        let r1c = b.vm_reg(1);
        let tint = b.const_tag(TINTEGER);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r1c, tint);
        let r1d = b.vm_reg(1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT64, r1d, val);

        let r1e = b.vm_reg(1);
        let c1 = b.const_int(1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1e, c1);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R0\n   %0 = LOAD_INT64 R0\n   STORE_SPLIT_TVALUE R1, tinteger, %0\n   RETURN R1, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
