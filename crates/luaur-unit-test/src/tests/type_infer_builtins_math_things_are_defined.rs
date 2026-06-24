//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:20:type_infer_builtins_math_things_are_defined`
//! Source: `tests/TypeInfer.builtins.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function min (Analysis/include/Luau/Unifiable.h)
//!   - translates_to -> rust_item type_infer_builtins_math_things_are_defined

#[cfg(test)]
#[test]
fn type_infer_builtins_math_things_are_defined() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local a00 = math.frexp
        local a01 = math.ldexp
        local a02 = math.fmod
        local a03 = math.modf
        local a04 = math.pow
        local a05 = math.exp
        local a06 = math.floor
        local a07 = math.abs
        local a08 = math.sqrt
        local a09 = math.log
        local a10 = math.log10
        local a11 = math.rad
        local a12 = math.deg
        local a13 = math.sin
        local a14 = math.cos
        local a15 = math.tan
        local a16 = math.sinh
        local a17 = math.cosh
        local a18 = math.tanh
        local a19 = math.atan
        local a20 = math.acos
        local a21 = math.asin
        local a22 = math.atan2
        local a23 = math.ceil
        local a24 = math.min
        local a25 = math.max
        local a26 = math.pi
        local a27 = math.huge
        local a28 = math.nan
        local a29 = math.e
        local a30 = math.phi
        local a31 = math.sqrt2
        local a32 = math.tau
        local a33 = math.randomseed
        local a34 = math.random
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
