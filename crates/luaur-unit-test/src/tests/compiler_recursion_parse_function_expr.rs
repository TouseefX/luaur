#[cfg(test)]
#[test]
fn compiler_recursion_parse_function_expr() {
    use crate::functions::rep::rep;
    use crate::records::recursion_limit_fixture::RecursionLimitFixture;

    let mut fix = RecursionLimitFixture {
        bcb: luaur_bytecode::records::bytecode_builder::BytecodeBuilder::new(None),
        reps: 1500,
        find_limit: false,
    };
    // NOTE(2025-11-25) Limit of 1380 on VS2022 optimized build
    let reps = 1380;
    let code = format!(
        "return {}42{}",
        rep("function() return ", reps),
        rep(" end", reps)
    );
    let message = "Exceeded allowed recursion depth; simplify your block to make the code compile";
    fix.check_limit(&code, message);
}
