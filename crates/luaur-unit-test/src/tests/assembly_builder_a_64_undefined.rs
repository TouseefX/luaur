#[cfg(test)]
#[test]
fn assembly_builder_a_64_undefined() {
    use luaur_code_gen::records::assembly_builder_a_64::AssemblyBuilderA64;

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.udf();
    build.finalize();
    assert_eq!(build.code[0], 0x00000000, "instruction byte mismatch");
}
