#[cfg(test)]
#[test]
fn compiler_host_types_vector() {
    use crate::functions::compile_type_table::compile_type_table;

    let actual = compile_type_table(
        r#"function myfunc(test: Instance, pos: Vector3)
end

function myfunc2<Vector3>(test: Instance, pos: Vector3)
end

do
    type Vector3 = number

    function myfunc3(test: Instance, pos: Vector3)
    end
end
"#,
    );
    let expected = "\n0: function(userdata, vector)\n1: function(userdata, any)\n2: function(userdata, number)\n";
    assert_eq!(format!("\n{}", actual), expected);
}
