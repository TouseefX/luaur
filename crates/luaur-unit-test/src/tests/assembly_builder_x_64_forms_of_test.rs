#[cfg(test)]
#[test]
fn assembly_builder_x_64_forms_of_test() {
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

    check(|b| b.test(R::al.into(), 8i32.into()), &[0xf6, 0xc0, 0x08]);
    check(
        |b| b.test(R::eax.into(), 8i32.into()),
        &[0xf7, 0xc0, 0x08, 0x00, 0x00, 0x00],
    );
    check(
        |b| b.test(R::rax.into(), 8i32.into()),
        &[0x48, 0xf7, 0xc0, 0x08, 0x00, 0x00, 0x00],
    );
    check(
        |b| b.test(R::rcx.into(), 0xababi32.into()),
        &[0x48, 0xf7, 0xc1, 0xab, 0xab, 0x00, 0x00],
    );
    check(
        |b| b.test(R::rcx.into(), R::rax.into()),
        &[0x48, 0x85, 0xc8],
    );
    check(
        |b| b.test(R::rax.into(), idx(qword, R::rcx)),
        &[0x48, 0x85, 0x01],
    );
    check(|b| b.test(R::al.into(), R::cl.into()), &[0x84, 0xc1]);
    check(|b| b.test(R::al.into(), R::sil.into()), &[0x40, 0x84, 0xc6]);
    check(
        |b| b.test(R::cl.into(), R::r12b.into()),
        &[0x41, 0x84, 0xcc],
    );
    check(
        |b| b.test(R::sil.into(), R::dil.into()),
        &[0x40, 0x84, 0xf7],
    );
}
