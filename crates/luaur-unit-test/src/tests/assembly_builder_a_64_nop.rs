#[cfg(test)]
#[test]
fn assembly_builder_a_64_nop() {
    use crate::records::assembly_builder_a_64_fixture::AssemblyBuilderA64Fixture;
    use luaur_code_gen::records::assembly_builder_a_64::AssemblyBuilderA64;

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), code: &[u32]) -> bool {
        let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
        f(&mut build);
        build.finalize();
        &build.code[..] == code
    }

    let mut fixture = AssemblyBuilderA64Fixture::default();

    // 0 bytes: no instructions emitted
    assert!(fixture.check(
        |b| {
            b.nop(0);
        },
        alloc::vec::Vec::new(),
        alloc::vec::Vec::new(),
        0,
    ));

    // Non-multiple of 4: rounds down to nearest multiple (7 -> 1 NOP = 4 bytes)
    assert!(fixture.check(
        |b| {
            b.nop(7);
        },
        alloc::vec![0xD503201F],
        alloc::vec::Vec::new(),
        0,
    ));

    // Exact multiples: 4 -> 1 NOP, 8 -> 2 NOPs, 12 -> 3 NOPs
    assert!(fixture.check(
        |b| {
            b.nop(4);
        },
        alloc::vec![0xD503201F],
        alloc::vec::Vec::new(),
        0,
    ));

    assert!(fixture.check(
        |b| {
            b.nop(8);
        },
        alloc::vec![0xD503201F, 0xD503201F],
        alloc::vec::Vec::new(),
        0,
    ));

    assert!(fixture.check(
        |b| {
            b.nop(12);
        },
        alloc::vec![0xD503201F, 0xD503201F, 0xD503201F],
        alloc::vec::Vec::new(),
        0,
    ));
}
