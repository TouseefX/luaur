#[cfg(test)]
#[test]
fn compiler_builtin_type_vector() {
    use crate::functions::compile_type_table::compile_type_table;

    let actual = compile_type_table("function myfunc(test: Instance, pos: vector)\nend");
    let expected = "\n0: function(userdata, vector)\n";

    assert_eq!(format!("\n{}", actual), expected);
}
