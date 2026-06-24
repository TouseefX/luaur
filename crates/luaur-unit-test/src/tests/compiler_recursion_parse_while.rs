#[cfg(test)]
#[test]
fn compiler_recursion_parse_while() {
    use crate::functions::rep::rep;
    use crate::records::recursion_limit_fixture::RecursionLimitFixture;

    let mut fix = RecursionLimitFixture {
        bcb: luaur_bytecode::records::bytecode_builder::BytecodeBuilder::new(None),
        reps: 2380,
        find_limit: false,
    };

    let code =
        rep("while true do ", fix.reps as usize) + "print()" + &rep(" end", fix.reps as usize);

    fix.check_limit(
        &code,
        "Exceeded allowed recursion depth; simplify your expression to make the code compile",
    );
}
