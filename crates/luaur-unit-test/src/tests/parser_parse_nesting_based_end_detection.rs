#[cfg(test)]
#[test]
fn parser_parse_nesting_based_end_detection() {
    use crate::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let code = alloc::string::String::from(
        "-- i am line 1
function BottomUpTree(item, depth)
  if depth > 0 then
    local i = item + item
    depth = depth - 1
    local left, right = BottomUpTree(i-1, depth), BottomUpTree(i, depth)
    return { item, left, right }
  else
    return { item }
end

function ItemCheck(tree)
  if tree[2] then
    return tree[1] + ItemCheck(tree[2]) - ItemCheck(tree[3])
  else
    return tree[1]
  end
end
",
    );
    let result = fix.try_parse(
        &code,
        &luaur_ast::records::parse_options::ParseOptions::parse_options(),
    );

    luaur_common::LUAU_ASSERT!(result.errors.len() == 1);
    let msg = unsafe {
        core::ffi::CStr::from_ptr(
            result.errors[0].get_message().as_ptr() as *const core::ffi::c_char
        )
    }
    .to_string_lossy();
    let expected_msg = "Expected 'end' (to close 'function' at line 2), got <eof>; did you forget to close 'else' at line 8?" ;
    assert_eq!(&*msg, expected_msg);
}
