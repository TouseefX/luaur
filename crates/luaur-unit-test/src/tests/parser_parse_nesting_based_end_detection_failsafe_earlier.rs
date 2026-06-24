#[cfg(test)]
#[test]
fn parser_parse_nesting_based_end_detection_failsafe_earlier() {
    use crate::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let code = alloc::string::String::from(
        r#"-- i am line 1
local function ItemCheck(tree)
  if tree[2] then
    return tree[1] + ItemCheck(tree[2]) - ItemCheck(tree[3])
  else
    return tree[1]
      end
end

local function BottomUpTree(item, depth)
  if depth > 0 then
    local i = item + item
    depth = depth - 1
    local left, right = BottomUpTree(i-1, depth), BottomUpTree(i, depth)
    return { item, left, right }
  else
    return { item }
  end
"#,
    );
    let result = fix.try_parse(
        &code,
        &luaur_ast::records::parse_options::ParseOptions::parse_options(),
    );
    if result.errors.is_empty() {
        panic!("Expected ParseErrors to be thrown");
    }
    let first_error = &result.errors[0];
    assert_eq!(
        first_error.get_message().as_str(),
        "Expected 'end' (to close 'function' at line 10), got <eof>"
    );
}
