#[cfg(test)]
#[test]
fn compiler_type_alias_resolve() {
    use crate::functions::compile_type_table::compile_type_table;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag::LuauCompileTypeAliases;

    let _scoped_flag = ScopedFastFlag::new(&LuauCompileTypeAliases, true);

    let actual = compile_type_table(
        r#"type Foo1 = number
type Foo2 = { number }
type Foo3 = Part
type Foo4 = Foo1
type Foo5<X> = X

function myfunc(f1: Foo1, f2: Foo2, f3: Foo3, f4: Foo4, f5: Foo5<number>)
end

function myfuncerr(f1: Foo1<string>, f2: Foo5)
end
"#,
    );

    let expected = r#"
0: function(number, table, userdata, number, any)
1: function(number, any)
"#;

    assert_eq!(format!("\n{}", actual), expected);
}
