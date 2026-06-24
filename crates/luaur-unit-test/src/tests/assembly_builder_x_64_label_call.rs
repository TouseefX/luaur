#[cfg(test)]
#[test]
fn assembly_builder_x_64_label_call() {
    use luaur_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64;
    use luaur_code_gen::records::label::Label;
    use luaur_code_gen::records::operand_x_64::{addr, OperandX64};
    use luaur_code_gen::records::register_x_64::RegisterX64 as R;

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
        let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
        f(&mut build);
        build.finalize();
        assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }
    fn idx(prefix: OperandX64, address: impl Into<OperandX64>) -> OperandX64 {
        prefix.operator_bracket(address.into())
    }

    check(
        |b| {
            let mut fn_b = Label::default();
            b.and_(R::rcx.into(), 0x3ei32.into());
            b.call_label(&mut fn_b);
            b.ret();
            b.set_label_label(&mut fn_b);
            b.lea_operand_x_64_operand_x_64(R::rax.into(), idx(addr, R::rcx + 0x1f));
            b.ret();
        },
        &[
            0x48, 0x83, 0xe1, 0x3e, 0xe8, 0x01, 0x00, 0x00, 0x00, 0xc3, 0x48, 0x8d, 0x41, 0x1f,
            0xc3,
        ],
    );
}
