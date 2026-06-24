#[cfg(test)]
#[test]
fn compiler_recursion_parse_do_block() {
    use crate::functions::rep::rep;
    use crate::records::recursion_limit_fixture::RecursionLimitFixture;

    let mut fix = RecursionLimitFixture {
        bcb: luaur_bytecode::records::bytecode_builder::BytecodeBuilder::new(None),
        reps: 2380,
        find_limit: false,
    };

    let reps = fix.reps as usize;
    let code = rep("do ", reps) + "print()" + &rep(" end", reps);
    fix.check_limit(
        &code,
        "Exceeded allowed recursion depth; simplify your block to make the code compile",
    );
}
