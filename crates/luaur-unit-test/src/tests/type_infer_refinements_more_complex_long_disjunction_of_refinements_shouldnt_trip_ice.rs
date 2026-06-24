//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2392:type_infer_refinements_more_complex_long_disjunction_of_refinements_shouldnt_trip_ice`
//! Source: `tests/TypeInfer.refinements.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
//! - outgoing:
//!   - calls -> method SubtypeFixture::obj (tests/Subtyping.test.cpp)
//!   - translates_to -> rust_item type_infer_refinements_more_complex_long_disjunction_of_refinements_shouldnt_trip_ice

#[cfg(test)]
#[test]
fn type_infer_refinements_more_complex_long_disjunction_of_refinements_shouldnt_trip_ice() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
script:connect(function(obj)
	if script.Parent.SeatNumber.Value == "1D" or
    script.Parent.SeatNumber.Value == "2D" or
    script.Parent.SeatNumber.Value == "3D" or
    script.Parent.SeatNumber.Value == "4D" or
    script.Parent.SeatNumber.Value == "5D" or
    script.Parent.SeatNumber.Value == "6D" or
    script.Parent.SeatNumber.Value == "7D" or
    script.Parent.SeatNumber.Value == "8D" or
    script.Parent.SeatNumber.Value == "9D" or
    script.Parent.SeatNumber.Value == "10D" or
    script.Parent.SeatNumber.Value == "11D" or
    script.Parent.SeatNumber.Value == "12D" or
    script.Parent.SeatNumber.Value == "13D" or
    script.Parent.SeatNumber.Value == "14D" or
    script.Parent.SeatNumber.Value == "15D" or
    script.Parent.SeatNumber.Value == "16D" or
    script.Parent.SeatNumber.Value == "1C" or
    script.Parent.SeatNumber.Value == "2C" or
    script.Parent.SeatNumber.Value == "3C" or
    script.Parent.SeatNumber.Value == "4C" or
    script.Parent.SeatNumber.Value == "5C" or
    script.Parent.SeatNumber.Value == "6C" or
    script.Parent.SeatNumber.Value == "7C" or
    script.Parent.SeatNumber.Value == "8C" or
    script.Parent.SeatNumber.Value == "9C" or
    script.Parent.SeatNumber.Value == "10C" or
    script.Parent.SeatNumber.Value == "11C" or
    script.Parent.SeatNumber.Value == "12C" or
    script.Parent.SeatNumber.Value == "13C" or
    script.Parent.SeatNumber.Value == "14C" or
    script.Parent.SeatNumber.Value == "15C" or
    script.Parent.SeatNumber.Value == "16C" then
    end)
"#,
        ),
        None,
    );
}
