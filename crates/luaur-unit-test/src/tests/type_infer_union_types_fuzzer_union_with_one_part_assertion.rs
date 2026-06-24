//! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_union_types_fuzzer_union_with_one_part_assertion() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);

    let _result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
local _ = {},nil
repeat

_,_ = if _.number == "" or _.number or _._ then
             _
      elseif _.__index == _._G then
            tostring
      elseif _ then
             _
      else
           ``,_._G

until _._
    "#,
        ),
        None,
    );
}
