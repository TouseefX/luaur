#[cfg(test)]
#[test]
fn compiler_recursion_parse_binary_op() {
    use crate::functions::rep::rep;
    use crate::records::recursion_limit_fixture::RecursionLimitFixture;

    let mut fixture = RecursionLimitFixture {
        bcb: luaur_bytecode::records::bytecode_builder::BytecodeBuilder::new(None),
        reps: 1000,
        find_limit: false,
    };

    let reps = fixture.reps as usize;
    let code = "a=1".to_string() + &rep("+1", reps);
    let message =
        "Exceeded allowed recursion depth; simplify your expression to make the code compile";

    fixture.check_limit(&code, message);
}
