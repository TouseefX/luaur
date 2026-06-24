#[cfg(test)]
#[test]
fn compiler_dump_constants_tables() {
    use crate::functions::compile_function_0_constants::compile_function_0_constants;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag::LuauCompileDuptableConstantPack2;

    let _flag = ScopedFastFlag::new(&LuauCompileDuptableConstantPack2, true);

    let actual = compile_function_0_constants(
        r#"return {a=1,b=2,c=3}, {only=42}, {first=10, second=20, third=30}"#,
    );
    let expected = "\n\
K0: 'a'\n\
K1: 1\n\
K2: 'b'\n\
K3: 2\n\
K4: 'c'\n\
K5: 3\n\
K6: {['a'] = 1 #0, ['b'] = 2 #3, ['c'] = 3 #2} sizenode=4\n\
K7: 'only'\n\
K8: 42\n\
K9: {['only'] = 42 #0} sizenode=1\n\
K10: 'first'\n\
K11: 10\n\
K12: 'second'\n\
K13: 20\n\
K14: 'third'\n\
K15: 30\n\
K16: {['first'] = 10 #1, ['second'] = 20 #3, ['third'] = 30 #3 (conflict)} sizenode=4\n\
DUPTABLE R0 6\n\
DUPTABLE R1 9\n\
DUPTABLE R2 16\n\
RETURN R0 3\n";

    assert_eq!("\n".to_string() + &actual, expected);
}
