#[cfg(test)]
#[test]
fn compiler_recursion_parse_group_expr() {
    use crate::functions::rep::rep;
    use crate::records::recursion_limit_fixture::RecursionLimitFixture;

    let mut fixture = RecursionLimitFixture {
        bcb: luaur_bytecode::records::bytecode_builder::BytecodeBuilder::new(None),
        reps: 1590,
        find_limit: false,
    };

    let reps = fixture.reps as usize;
    let code = format!("a={}{}1{}", rep("(", reps), rep(")", reps), rep(")", reps));
    fixture.check_limit(
        &code,
        "Exceeded allowed recursion depth; simplify your expression to make the code compile",
    );
}
