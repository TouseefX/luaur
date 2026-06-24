#[cfg(test)]
#[test]
fn assembly_builder_x_64_forms_of_setcc() {
    use luaur_code_gen::enums::condition_x_64::ConditionX64;
    use luaur_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64;
    use luaur_code_gen::records::operand_x_64::{byte, OperandX64};
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
        |b| b.setcc(ConditionX64::NotEqual, R::bl.into()),
        &[0x0f, 0x95, 0xc3],
    );
    check(
        |b| b.setcc(ConditionX64::NotEqual, R::dil.into()),
        &[0x40, 0x0f, 0x95, 0xc7],
    );
    check(
        |b| b.setcc(ConditionX64::BelowEqual, idx(byte, R::rcx)),
        &[0x0f, 0x96, 0x01],
    );
}
