#[cfg(test)]
#[test]
fn assembly_builder_x_64_forms_of_imul() {
    use luaur_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64;
    use luaur_code_gen::records::operand_x_64::{qword, OperandX64};
    use luaur_code_gen::records::register_x_64::RegisterX64 as R;

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
        let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
        f(&mut build);
        build.finalize();
        assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }
    fn idx(prefix: OperandX64, addr: impl Into<OperandX64>) -> OperandX64 {
        prefix.operator_bracket(addr.into())
    }

    check(
        |b| b.imul_operand_x_64_operand_x_64(R::ecx.into(), R::esi.into()),
        &[0x0f, 0xaf, 0xce],
    );
    check(
        |b| b.imul_operand_x_64_operand_x_64(R::r12.into(), R::rax.into()),
        &[0x4c, 0x0f, 0xaf, 0xe0],
    );
    check(
        |b| b.imul_operand_x_64_operand_x_64(R::r12.into(), idx(qword, R::rdx + R::rdi)),
        &[0x4c, 0x0f, 0xaf, 0x24, 0x3a],
    );
    check(
        |b| b.imul_operand_x_64_operand_x_64_i32(R::ecx.into(), R::edx.into(), 8),
        &[0x6b, 0xca, 0x08],
    );
    check(
        |b| b.imul_operand_x_64_operand_x_64_i32(R::ecx.into(), R::r9d.into(), 0xabcd),
        &[0x41, 0x69, 0xc9, 0xcd, 0xab, 0x00, 0x00],
    );
    check(
        |b| b.imul_operand_x_64_operand_x_64_i32(R::r8d.into(), R::eax.into(), -9),
        &[0x44, 0x6b, 0xc0, 0xf7],
    );
    check(
        |b| b.imul_operand_x_64_operand_x_64_i32(R::rcx.into(), R::rdx.into(), 17),
        &[0x48, 0x6b, 0xca, 0x11],
    );
    check(
        |b| b.imul_operand_x_64_operand_x_64_i32(R::rcx.into(), R::r12.into(), 0xabcd),
        &[0x49, 0x69, 0xcc, 0xcd, 0xab, 0x00, 0x00],
    );
    check(
        |b| b.imul_operand_x_64_operand_x_64_i32(R::r12.into(), R::rax.into(), -13),
        &[0x4c, 0x6b, 0xe0, 0xf3],
    );
}
