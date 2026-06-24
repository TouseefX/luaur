#[cfg(test)]
#[test]
fn compiler_encoded_type_table() {
    use crate::functions::compile_type_table::compile_type_table;

    let result1 = compile_type_table(
        "function myfunc(test: string, num: number)\n    print(test)\nend\n\nfunction myfunc2(test: number?)\nend\n\nfunction myfunc3(test: string, n: number)\nend\n\nfunction myfunc4(test: string | number, n: number)\nend\n\n-- Promoted to function(any, any) since general unions are not supported.\n-- Functions with all `any` parameters will have omitted type info.\nfunction myfunc5(test: string | number, n: number | boolean)\nend\n\nfunction myfunc6(test: (number) -> string)\nend\n\nfunction myfunc7(test: true)\nend\n\nfunction myfunc8(test: \"str\")\nend\n\nmyfunc('test')",
    );
    let expected1 = "\n0: function(string, number)\n1: function(number?)\n2: function(string, number)\n3: function(any, number)\n5: function(function)\n6: function(boolean)\n7: function(string)\n";
    assert_eq!(format!("\n{}", result1), expected1);

    let result2 = compile_type_table(
        "local Str = {\n    a = 1\n}\n\n-- Implicit `self` parameter is automatically assumed to be table type.\nfunction Str:test(n: number)\n    print(self.a, n)\nend\n\nStr:test(234)",
    );
    let expected2 = "\n0: function(table, number)\n";
    assert_eq!(format!("\n{}", result2), expected2);
}
