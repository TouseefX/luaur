#[cfg(test)]
#[test]
fn assembly_builder_x_64_forms_of_cmov() {
    use luaur_code_gen::enums::condition_x_64::ConditionX64;
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
        |b| b.cmov(ConditionX64::LessEqual, R::ebx, R::eax.into()),
        &[0x0f, 0x4e, 0xd8],
    );
    check(
        |b| b.cmov(ConditionX64::NotZero, R::rbx, idx(qword, R::rax)),
        &[0x48, 0x0f, 0x45, 0x18],
    );
    check(
        |b| b.cmov(ConditionX64::Zero, R::rbx, idx(qword, R::rax + R::rcx)),
        &[0x48, 0x0f, 0x44, 0x1c, 0x08],
    );
    check(
        |b| b.cmov(ConditionX64::BelowEqual, R::r14d, R::r15d.into()),
        &[0x45, 0x0f, 0x46, 0xf7],
    );
}
