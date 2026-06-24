#[cfg(test)]
#[test]
fn compiler_recursion_parse_function_name_index() {
    use crate::functions::rep::rep;
    use crate::records::recursion_limit_fixture::RecursionLimitFixture;

    let mut fix = RecursionLimitFixture {
        bcb: luaur_bytecode::records::bytecode_builder::BytecodeBuilder::new(None),
        reps: 1500,
        find_limit: false,
    };
    let reps = fix.reps as usize;
    let code = format!("function a{}() end", rep(".a", reps));
    let message =
        "Exceeded allowed recursion depth; simplify your function name to make the code compile";
    fix.check_limit(&code, message);
}
