#[cfg(test)]
#[test]
fn assembly_builder_x_64_forms_of_lea() {
    use luaur_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64;
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
        |b| b.lea_operand_x_64_operand_x_64(R::rax.into(), idx(addr, R::rdx + R::rcx)),
        &[0x48, 0x8d, 0x04, 0x0a],
    );
    check(
        |b| b.lea_operand_x_64_operand_x_64(R::rax.into(), idx(addr, R::rdx + R::rax * 4)),
        &[0x48, 0x8d, 0x04, 0x82],
    );
    check(
        |b| b.lea_operand_x_64_operand_x_64(R::rax.into(), idx(addr, R::r13 + R::r12 * 4 + 4)),
        &[0x4b, 0x8d, 0x44, 0xa5, 0x04],
    );
}
