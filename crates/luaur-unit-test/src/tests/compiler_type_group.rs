#[cfg(test)]
#[test]
fn compiler_type_group() {
    use crate::functions::compile_type_table::compile_type_table;

    let actual = compile_type_table(
        r#"function myfunc(test: (string), foo: nil)
end

function myfunc2(test: (string | nil), foo: nil)
end
"#,
    );
    let expected = "\n0: function(string, nil)\n1: function(string?, nil)\n";
    assert_eq!(format!("\n{}", actual), expected);
}
