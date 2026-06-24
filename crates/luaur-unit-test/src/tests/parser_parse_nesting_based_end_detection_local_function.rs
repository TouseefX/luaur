#[cfg(test)]
#[test]
fn parser_parse_nesting_based_end_detection_local_function() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from("-- i am line 1\nlocal function BottomUpTree(item, depth)\n  if depth > 0 then\n    local i = item + item\n    depth = depth - 1\n    local left, right = BottomUpTree(i-1, depth), BottomUpTree(i, depth)\n    return { item, left, right }\n  else\n    return { item }\nend\n\nlocal function ItemCheck(tree)\n  if tree[2] then\n    return tree[1] + ItemCheck(tree[2]) - ItemCheck(tree[3])\n  else\n    return tree[1]\n  end\nend\n        "),
        &alloc::string::String::from("Expected 'end' (to close 'function' at line 2), got <eof>; did you forget to close 'else' at line 8?"),
        None,
    );
}
