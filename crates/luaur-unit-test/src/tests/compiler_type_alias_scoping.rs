#[cfg(test)]
#[test]
fn compiler_type_alias_scoping() {
    use crate::functions::compile_type_table::compile_type_table;

    let actual = compile_type_table(
        r#"do
    type Part = number
end

function myfunc1(test: Part, num: number)
end

do
    type Part = number

    function myfunc2(test: Part, num: number)
    end
end

repeat
    type Part = number
until (function(test: Part, num: number) end)()

function myfunc4(test: Instance, num: number)
end

type Instance = string
"#,
    );

    let expected = "\n0: function(userdata, number)\n1: function(number, number)\n2: function(number, number)\n3: function(string, number)\n";

    assert_eq!(format!("\n{}", actual), expected);
}
