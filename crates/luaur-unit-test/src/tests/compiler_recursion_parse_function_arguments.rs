#[cfg(test)]
#[test]
fn compiler_recursion_parse_function_arguments() {
    use crate::functions::rep::rep;
    use crate::records::recursion_limit_fixture::RecursionLimitFixture;

    let mut fixture = RecursionLimitFixture {
        bcb: luaur_bytecode::records::bytecode_builder::BytecodeBuilder::new(None),
        reps: 1070,
        find_limit: false,
    };

    let code = rep("a(", fixture.reps as usize) + "42" + &rep(")", fixture.reps as usize);
    fixture.check_limit(
        &code,
        "Exceeded allowed recursion depth; simplify your expression to make the code compile",
    );
}
