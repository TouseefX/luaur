#[cfg(test)]
#[test]
fn ir_builder_numeric() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use luaur_code_gen::enums::include_use_info::IncludeUseInfo;
    use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::functions::to_string_ir_dump_alt_g::to_string;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;

    const TNIL: u8 = 0;
    const TNUMBER: u8 = 3;
    const TBOOLEAN: u8 = 1;

    let mut fix = IrBuilderFixture::new();
    {
        let b = &mut fix.build;
        let block = b.block(IrBlockKind::Internal);
        b.begin_block(block);

        // Binary integer/number folds: STORE R{reg}, CMD(a, b)
        macro_rules! bin_i {
            ($store:expr, $reg:expr, $cmd:expr, $a:expr, $bb:expr) => {{
                let ca = b.const_int($a);
                let cb = b.const_int($bb);
                let op = b.inst_ir_cmd_ir_op_ir_op($cmd, ca, cb);
                let r = b.vm_reg($reg);
                b.inst_ir_cmd_ir_op_ir_op($store, r, op);
            }};
        }
        macro_rules! bin_d {
            ($store:expr, $reg:expr, $cmd:expr, $a:expr, $bb:expr) => {{
                let ca = b.const_double($a);
                let cb = b.const_double($bb);
                let op = b.inst_ir_cmd_ir_op_ir_op($cmd, ca, cb);
                let r = b.vm_reg($reg);
                b.inst_ir_cmd_ir_op_ir_op($store, r, op);
            }};
        }
        macro_rules! un_d {
            ($store:expr, $reg:expr, $cmd:expr, $a:expr) => {{
                let ca = b.const_double($a);
                let op = b.inst_ir_cmd_ir_op($cmd, ca);
                let r = b.vm_reg($reg);
                b.inst_ir_cmd_ir_op_ir_op($store, r, op);
            }};
        }
        macro_rules! un_i {
            ($store:expr, $reg:expr, $cmd:expr, $a:expr) => {{
                let ca = b.const_int($a);
                let op = b.inst_ir_cmd_ir_op($cmd, ca);
                let r = b.vm_reg($reg);
                b.inst_ir_cmd_ir_op_ir_op($store, r, op);
            }};
        }

        bin_i!(IrCmd::STORE_INT, 0, IrCmd::ADD_INT, 10, 20);
        bin_i!(IrCmd::STORE_INT, 1, IrCmd::ADD_INT, i32::MAX, 1);
        bin_i!(IrCmd::STORE_INT, 2, IrCmd::SUB_INT, 10, 20);
        bin_i!(IrCmd::STORE_INT, 3, IrCmd::SUB_INT, i32::MIN, 1);

        bin_d!(IrCmd::STORE_DOUBLE, 4, IrCmd::ADD_NUM, 2.0, 5.0);
        bin_d!(IrCmd::STORE_DOUBLE, 5, IrCmd::SUB_NUM, 2.0, 5.0);
        bin_d!(IrCmd::STORE_DOUBLE, 6, IrCmd::MUL_NUM, 2.0, 5.0);
        bin_d!(IrCmd::STORE_DOUBLE, 7, IrCmd::DIV_NUM, 2.0, 5.0);
        bin_d!(IrCmd::STORE_DOUBLE, 8, IrCmd::MOD_NUM, 5.0, 2.0);
        bin_d!(IrCmd::STORE_DOUBLE, 10, IrCmd::MIN_NUM, 5.0, 2.0);
        bin_d!(IrCmd::STORE_DOUBLE, 11, IrCmd::MAX_NUM, 5.0, 2.0);

        un_d!(IrCmd::STORE_DOUBLE, 12, IrCmd::UNM_NUM, 5.0);
        un_d!(IrCmd::STORE_DOUBLE, 13, IrCmd::FLOOR_NUM, 2.5);
        un_d!(IrCmd::STORE_DOUBLE, 14, IrCmd::CEIL_NUM, 2.5);
        un_d!(IrCmd::STORE_DOUBLE, 15, IrCmd::ROUND_NUM, 2.5);
        un_d!(IrCmd::STORE_DOUBLE, 16, IrCmd::SQRT_NUM, 16.0);
        un_d!(IrCmd::STORE_DOUBLE, 17, IrCmd::ABS_NUM, -4.0);

        // NOT_ANY(tag, LOAD_DOUBLE(R1)) and NOT_ANY(tag, const)
        {
            let tag = b.const_tag(TNIL);
            let r1 = b.vm_reg(1);
            let load = b.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, r1);
            let op = b.inst_ir_cmd_ir_op_ir_op(IrCmd::NOT_ANY, tag, load);
            let r = b.vm_reg(18);
            b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, r, op);
        }
        {
            let tag = b.const_tag(TNUMBER);
            let r1 = b.vm_reg(1);
            let load = b.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, r1);
            let op = b.inst_ir_cmd_ir_op_ir_op(IrCmd::NOT_ANY, tag, load);
            let r = b.vm_reg(19);
            b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, r, op);
        }
        {
            let tag = b.const_tag(TBOOLEAN);
            let c0 = b.const_int(0);
            let op = b.inst_ir_cmd_ir_op_ir_op(IrCmd::NOT_ANY, tag, c0);
            let r = b.vm_reg(20);
            b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, r, op);
        }
        {
            let tag = b.const_tag(TBOOLEAN);
            let c1 = b.const_int(1);
            let op = b.inst_ir_cmd_ir_op_ir_op(IrCmd::NOT_ANY, tag, c1);
            let r = b.vm_reg(21);
            b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, r, op);
        }

        un_d!(IrCmd::STORE_DOUBLE, 22, IrCmd::SIGN_NUM, -4.0);

        un_i!(IrCmd::STORE_INT, 23, IrCmd::SEXTI8_INT, 0x7f);
        un_i!(IrCmd::STORE_INT, 24, IrCmd::SEXTI8_INT, 0xf1);
        un_i!(IrCmd::STORE_INT, 25, IrCmd::SEXTI16_INT, 0x7fff);
        un_i!(IrCmd::STORE_INT, 26, IrCmd::SEXTI16_INT, 0xf111);

        let c0 = b.const_uint(0);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c0);
    }

    update_use_counts(&mut fix.build.function);
    fix.constant_fold();

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT R0, 30i\n   STORE_INT R1, -2147483648i\n   STORE_INT R2, -10i\n   STORE_INT R3, 2147483647i\n   STORE_DOUBLE R4, 7\n   STORE_DOUBLE R5, -3\n   STORE_DOUBLE R6, 10\n   STORE_DOUBLE R7, 0.40000000000000002\n   STORE_DOUBLE R8, 1\n   STORE_DOUBLE R10, 2\n   STORE_DOUBLE R11, 5\n   STORE_DOUBLE R12, -5\n   STORE_DOUBLE R13, 2\n   STORE_DOUBLE R14, 3\n   STORE_DOUBLE R15, 3\n   STORE_DOUBLE R16, 4\n   STORE_DOUBLE R17, 4\n   STORE_INT R18, 1i\n   STORE_INT R19, 0i\n   STORE_INT R20, 1i\n   STORE_INT R21, 0i\n   STORE_DOUBLE R22, -1\n   STORE_INT R23, 127i\n   STORE_INT R24, -15i\n   STORE_INT R25, 32767i\n   STORE_INT R26, -3823i\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
