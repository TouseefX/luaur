#[cfg(test)]
#[test]
fn compiler_recursion_parse_function_statement() {
    use crate::functions::rep::rep;
    use crate::records::recursion_limit_fixture::RecursionLimitFixture;

    let mut fix = RecursionLimitFixture {
        bcb: luaur_bytecode::records::bytecode_builder::BytecodeBuilder::new(None),
        reps: 1150,
        find_limit: false,
    };

    let reps = fix.reps as usize;
    let code = rep("function a() ", reps) + "print()" + &rep(" end", reps);
    let message = "Exceeded allowed recursion depth; simplify your block to make the code compile";

    fix.check_limit(&code, message);
}
