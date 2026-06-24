#[cfg(test)]
#[test]
fn ir_builder_int_64_arith_chain_const_fold() {
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

        // ((10 + 20) * 3) - 5
        let c10 = b.const_int_64(10);
        let c20 = b.const_int_64(20);
        let add = b.inst_ir_cmd_ir_op_ir_op(IrCmd::ADD_INT64, c10, c20);
        let c3 = b.const_int_64(3);
        let mul = b.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_INT64, add, c3);
        let c5 = b.const_int_64(5);
        let sub = b.inst_ir_cmd_ir_op_ir_op(IrCmd::SUB_INT64, mul, c5);
        let r0 = b.vm_reg(0);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT64, r0, sub);

        // (0xFF & 0x0F) | 0xF0
        let cff = b.const_int_64(0xFF);
        let c0f = b.const_int_64(0x0F);
        let band = b.inst_ir_cmd_ir_op_ir_op(IrCmd::BITAND_INT64, cff, c0f);
        let cf0 = b.const_int_64(0xF0);
        let bor = b.inst_ir_cmd_ir_op_ir_op(IrCmd::BITOR_INT64, band, cf0);
        let r1 = b.vm_reg(1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT64, r1, bor);

        // (1 << 10) >> 5
        let c1 = b.const_int_64(1);
        let c10b = b.const_int_64(10);
        let lshift = b.inst_ir_cmd_ir_op_ir_op(IrCmd::BITLSHIFT_INT64, c1, c10b);
        let c5b = b.const_int_64(5);
        let rshift = b.inst_ir_cmd_ir_op_ir_op(IrCmd::BITRSHIFT_INT64, lshift, c5b);
        let r2 = b.vm_reg(2);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT64, r2, rshift);

        let c0 = b.const_uint(0);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT64 R0, 85i\n   STORE_INT64 R1, 255i\n   STORE_INT64 R2, 32i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
