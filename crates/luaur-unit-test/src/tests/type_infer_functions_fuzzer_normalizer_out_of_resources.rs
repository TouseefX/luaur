//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2947:type_infer_functions_fuzzer_normalizer_out_of_resources`
//! Source: `tests/TypeInfer.functions.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.functions.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.functions.test.cpp
//! - outgoing:
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
//!   - translates_to -> rust_item type_infer_functions_fuzzer_normalizer_out_of_resources

#[cfg(test)]
#[test]
fn type_infer_functions_fuzzer_normalizer_out_of_resources() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _ = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
 Module 'l0':
local _ = true,...,_
if ... then
while _:_(_._G) do
do end
_ = _ and _
_ = 0 and {# _,}
local _ = "CCCCCCCCCCCCCCCCCCCCCCCCCCC"
local l0 = require(module0)
end
local function l0()
end
elseif _ then
l0 = _
end
do end
while _ do
_ = if _ then _ elseif _ then _,if _ then _ else _
_ = _()
do end
do end
if _ then
end
end
_ = _,{}

    "#,
        ),
        None,
    );
}
