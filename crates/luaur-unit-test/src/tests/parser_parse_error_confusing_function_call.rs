#[cfg(test)]
#[test]
fn parser_parse_error_confusing_function_call() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fix = Fixture::default();

    let code1 = String::from(
        "function add(x, y) return x + y end\n\
         add\n\
         (4, 7)",
    );
    let message1 = String::from(
        "Ambiguous syntax: this looks like an argument list for a function call, but could also be a start of new statement; use ';' to separate statements",
    );
    let result1 = fix.match_parse_error(&code1, &message1, None);
    assert_eq!(result1.errors.len(), 1);

    let code2 = String::from(
        "function add(x, y) return x + y end\n\
         local f = add\n\
         (f :: any)['x'] = 2",
    );
    let message2 = String::from(
        "Ambiguous syntax: this looks like an argument list for a function call, but could also be a start of new statement; use ';' to separate statements",
    );
    let result2 = fix.match_parse_error(&code2, &message2, None);
    assert_eq!(result2.errors.len(), 1);

    let code3 = String::from(
        "local x = {}\n\
         function x:add(a, b) return a + b end\n\
         x:add\n\
         (1, 2)",
    );
    let message3 = String::from(
        "Ambiguous syntax: this looks like an argument list for a function call, but could also be a start of new statement; use ';' to separate statements",
    );
    let result3 = fix.match_parse_error(&code3, &message3, None);
    assert_eq!(result3.errors.len(), 1);

    let code4 = String::from(
        "local t = {}\n\
         function f() return t end\n\
         t.x, (f)\n\
         ().y = 5, 6",
    );
    let message4 = String::from(
        "Ambiguous syntax: this looks like an argument list for a function call, but could also be a start of new statement; use ';' to separate statements",
    );
    let result4 = fix.match_parse_error(&code4, &message4, None);
    assert_eq!(result4.errors.len(), 1);
}
