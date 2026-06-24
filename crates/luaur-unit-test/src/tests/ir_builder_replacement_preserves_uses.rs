#[cfg(test)]
#[test]
fn ir_builder_replacement_preserves_uses() {
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
        let mask = b.const_int(!0u32 as i32);
        let op = b.inst_ir_cmd_ir_op_ir_op(IrCmd::BITXOR_UINT, unk, mask);
        let r = b.vm_reg(8);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, r, op);

        let c0 = b.const_uint(0);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::Yes);
    let expected = "\nbb_0:                                                       ; useCount: 0\n   %0 = LOAD_INT R0                                          ; useCount: 1, lastUse: %0\n   %1 = BITNOT_UINT %0                                       ; useCount: 1, lastUse: %0\n   STORE_INT R8, %1                                          ; %2\n   RETURN 0u                                                 ; %3\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
