#[cfg(test)]
#[test]
fn assembly_builder_a_64_binary_extended() {
    use luaur_code_gen::records::assembly_builder_a_64::AssemblyBuilderA64;
    use luaur_code_gen::records::register_a_64::RegisterA64;

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.add_register_a_64_register_a_64_register_a_64_i32(
        RegisterA64::x0,
        RegisterA64::x1,
        RegisterA64::w2,
        3,
    );
    build.finalize();
    assert_eq!(build.code[0], 0x8B224C20, "instruction word mismatch");

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.sub_register_a_64_register_a_64_register_a_64_i32(
        RegisterA64::x0,
        RegisterA64::x1,
        RegisterA64::w2,
        3,
    );
    build.finalize();
    assert_eq!(build.code[0], 0xCB224C20, "instruction word mismatch");
}
