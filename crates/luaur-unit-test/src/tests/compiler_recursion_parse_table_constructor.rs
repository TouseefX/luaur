#[cfg(test)]
#[test]
fn compiler_recursion_parse_table_constructor() {
    use crate::functions::rep::rep;
    use crate::records::recursion_limit_fixture::RecursionLimitFixture;

    let mut fixture = RecursionLimitFixture {
        bcb: luaur_bytecode::records::bytecode_builder::BytecodeBuilder::new(None),
        reps: 1510,
        find_limit: false,
    };

    let code = "a=".to_string() + &rep("{", 1510) + &rep("}", 1510);
    fixture.check_limit(
        &code,
        "Exceeded allowed recursion depth; simplify your expression to make the code compile",
    );
}
