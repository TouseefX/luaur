#[cfg(test)]
#[test]
fn ir_builder_select_if_truthy() {
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

        let r1 = b.vm_reg(1);
        let unknown_tv1 = b.inst_ir_cmd_ir_op(IrCmd::LOAD_TVALUE, r1);
        let r2 = b.vm_reg(2);
        let unknown_tv2 = b.inst_ir_cmd_ir_op(IrCmd::LOAD_TVALUE, r2);

        let op = b.inst_ir_cmd_ir_op_ir_op_ir_op(
            IrCmd::SELECT_IF_TRUTHY,
            unknown_tv1,
            unknown_tv2,
            unknown_tv2,
        );
        let r = b.vm_reg(0);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, r, op);

        let c0 = b.const_uint(0);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %1 = LOAD_TVALUE R2\n   STORE_TVALUE R0, %1\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
