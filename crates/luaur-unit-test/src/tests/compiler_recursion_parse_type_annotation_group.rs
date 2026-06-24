#[cfg(test)]
#[test]
fn compiler_recursion_parse_type_annotation_group() {
    use crate::functions::rep::rep;
    use crate::records::recursion_limit_fixture::RecursionLimitFixture;

    let mut fixture = RecursionLimitFixture {
        bcb: luaur_bytecode::records::bytecode_builder::BytecodeBuilder::new(None),
        reps: 1650,
        find_limit: false,
    };

    let code = "local f: ".to_string()
        + &rep("(", fixture.reps as usize)
        + "nil"
        + &rep(")", fixture.reps as usize);
    let message =
        "Exceeded allowed recursion depth; simplify your type annotation to make the code compile";

    fixture.check_limit(&code, message);
}
