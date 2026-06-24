//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2232:type_infer_regexp_hang`
//! Source: `tests/TypeInfer.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.test.cpp
//! - outgoing:
//!   - calls -> method SubtypeFixture::negate (tests/Subtyping.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_regexp_hang

#[cfg(test)]
#[test]
fn type_infer_regexp_hang() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
local outln, group_id, verb_flags = {}, {}, {
    newline = 1,
    newline_seq = 1,
    not_empty = 0
}
if not escape_c then
elseif escape_c >= 48 and escape_c <= 57 then
elseif escape_c == 69 then
elseif escape_c == 81 then
elseif escape_c == 78 then
    if codes[i] ~= 125 or i == start_i then
    end
    table.insert(outln, code_point)
elseif escape_c == 80 or escape_c == 112 then
    if script_set then
    elseif not valid_categories[c_name]then
    else
        table.insert(outln, { 'category', negate, c_name })
    end
elseif escape_c == 103 and (codes[i + 1] == 123 or codes[i + 1] >= 48 and codes[i + 1] <= 57)then
elseif escape_c == 111 then
elseif escape_c == 120 then
else
    table.insert(outln, esc_char or escape_c)
end

for i, v in ipairs(outln)do
    if type(v) == 'table' and (v[1] == 40 or v[1] == 'quantifier' and type(v[5]) == 'table' and v[5][1] == 40)then
        v = v[5]
    elseif type(v) == 'table' and (v[1] == 'backref' or v[1] == 'recurmatch')then
        for i1, v1 in ipairs(outln)do
            break
        end
    end
end
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
}
