#[cfg(test)]
#[test]
fn compiler_recursion_parse_for() {
    let mut fixture = crate::records::recursion_limit_fixture::RecursionLimitFixture {
        bcb: luaur_bytecode::records::bytecode_builder::BytecodeBuilder::new(None),
        reps: 1130,
        find_limit: false,
    };

    let reps_str = "for i=1,1 do ";
    let end_str = " end";
    let code = crate::functions::rep::rep(reps_str, fixture.reps as usize)
        + "print()"
        + &crate::functions::rep::rep(end_str, fixture.reps as usize);

    fixture.check_limit(
        &code,
        "Exceeded allowed recursion depth; simplify your expression to make the code compile",
    );
}
