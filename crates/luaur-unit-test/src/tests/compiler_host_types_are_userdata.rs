#[cfg(test)]
#[test]
fn compiler_host_types_are_userdata() {
    use crate::functions::compile_type_table::compile_type_table;

    let result = compile_type_table(
        r#"function myfunc(test: string, num: number)
    print(test)
end

function myfunc2(test: Instance, num: number)
end

type Foo = string

function myfunc3(test: string, n: Foo)
end

function myfunc4<Bar>(test: Bar, n: Part)
end
"#,
    );

    let expected = "\n0: function(string, number)\n1: function(userdata, number)\n2: function(string, string)\n3: function(any, userdata)\n";
    assert_eq!(format!("\n{}", result), expected);
}
