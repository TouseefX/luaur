#[cfg(test)]
#[test]
fn compiler_type_union_intersection() {
    use crate::functions::compile_type_table::compile_type_table;

    let actual = compile_type_table(
        r#"function myfunc(test: string | nil, foo: nil)
end

function myfunc2(test: string & nil, foo: nil)
end

function myfunc3(test: string | number, foo: nil)
end

function myfunc4(test: string & number, foo: nil)
end
"#,
    );

    let expected = "\n0: function(string?, nil)\n1: function(any, nil)\n2: function(any, nil)\n3: function(any, nil)\n";

    assert_eq!(format!("\n{}", actual), expected);
}
