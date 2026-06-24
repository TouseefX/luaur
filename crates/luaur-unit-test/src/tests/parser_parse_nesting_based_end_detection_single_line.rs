#[cfg(test)]
#[test]
fn parser_parse_nesting_based_end_detection_single_line() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let source = alloc :: string :: String :: from (
        "-- i am line 1\n\
         function ItemCheck(tree)\n\
           if tree[2] then return tree[1] + ItemCheck(tree[2]) - ItemCheck(tree[3]) else return tree[1]\n\
         end\n\
         \n\
         function BottomUpTree(item, depth)\n\
           if depth > 0 then\n\
             local i = item + item\n\
             depth = depth - 1\n\
             local left, right = BottomUpTree(i-1, depth), BottomUpTree(i, depth)\n\
             return { item, left, right }\n\
           else\n\
             return { item }\n\
           end\n\
         end\n\
         "
    ) ;
    let result = fixture.try_parse(
        &source,
        &luaur_ast::records::parse_options::ParseOptions::parse_options(),
    );

    assert_eq!(result.errors.len(), 1);
    let error_message = result.errors[0].get_message();
    assert_eq ! (
        & * error_message ,
        "Expected 'end' (to close 'function' at line 2), got <eof>; did you forget to close 'else' at line 3?"
    );
}
