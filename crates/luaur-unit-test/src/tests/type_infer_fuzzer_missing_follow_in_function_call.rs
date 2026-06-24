//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2838:type_infer_fuzzer_missing_follow_in_function_call`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function write (tests/JsonEmitter.test.cpp)
//!   - translates_to -> rust_item type_infer_fuzzer_missing_follow_in_function_call

#[cfg(test)]
#[test]
fn type_infer_fuzzer_missing_follow_in_function_call() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        do end
        _ = if _ then true elseif _ then if _ then _ elseif _ then 2 .. {} elseif _._ then l0 else _ elseif _ then if ... then _ elseif {} then `` elseif _ then {_G=_,}
        type t0<),A,)...> = ({_G:any,write n0:any,write _:any<<A...>()->()>,write [any]:""""""""""""""""""""userda290013136ta:(0x000062900131369029001313690"""})|(l0.any)
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
}
