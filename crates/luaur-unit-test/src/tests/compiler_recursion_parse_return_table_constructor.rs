#[cfg(test)]
#[test]
fn compiler_recursion_parse_return_table_constructor() {
    use crate::functions::rep::rep;
    use crate::records::recursion_limit_fixture::RecursionLimitFixture;

    let mut fixture = RecursionLimitFixture {
        bcb: luaur_bytecode::records::bytecode_builder::BytecodeBuilder::new(None),
        reps: 1510,
        find_limit: false,
    };

    let code = "return ".to_string()
        + &rep("{", fixture.reps as usize)
        + "42"
        + &rep("}", fixture.reps as usize);
    let message =
        "Exceeded allowed recursion depth; simplify your expression to make the code compile";

    fixture.check_limit(&code, message);
}
