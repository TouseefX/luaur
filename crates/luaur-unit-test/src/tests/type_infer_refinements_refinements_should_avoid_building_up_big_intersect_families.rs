//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2432:type_infer_refinements_refinements_should_avoid_building_up_big_intersect_families`
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
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - translates_to -> rust_item type_infer_refinements_refinements_should_avoid_building_up_big_intersect_families

#[cfg(test)]
#[test]
fn type_infer_refinements_refinements_should_avoid_building_up_big_intersect_families() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
script:connect(function(obj)
	if script.Parent.SeatNumber.Value == "1D" or script.Parent.SeatNumber.Value == "2D" or script.Parent.SeatNumber.Value == "3D" or script.Parent.SeatNumber.Value == "4D" or script.Parent.SeatNumber.Value == "5D" or script.Parent.SeatNumber.Value == "6D" or script.Parent.SeatNumber.Value == "7D" or script.Parent.SeatNumber.Value == "8D" or script.Parent.SeatNumber.Value == "9D" or script.Parent.SeatNumber.Value == "10D" or script.Parent.SeatNumber.Value == "11D" or script.Parent.SeatNumber.Value == "12D" or script.Parent.SeatNumber.Value == "13D" or script.Parent.SeatNumber.Value == "14D" or script.Parent.SeatNumber.Value == "15D" or script.Parent.SeatNumber.Value == "16D" or script.Parent.SeatNumber.Value == "1C" or script.Parent.SeatNumber.Value == "2C" or script.Parent.SeatNumber.Value == "3C" or script.Parent.SeatNumber.Value == "4C" or script.Parent.SeatNumber.Value == "5C" or script.Parent.SeatNumber.Value == "6C" or script.Parent.SeatNumber.Value == "7C" or script.Parent.SeatNumber.Value == "8C" or script.Parent.SeatNumber.Value == "9C" or script.Parent.SeatNumber.Value == "10C" or script.Parent.SeatNumber.Value == "11C" or script.Parent.SeatNumber.Value == "12C" or script.Parent.SeatNumber.Value == "13C" or script.Parent.SeatNumber.Value == "14C" or script.Parent.SeatNumber.Value == "15C" or script.Parent.SeatNumber.Value == "16C" then
		if p.Name == script.Parent.Parent.Parent.Parent.Parent.Parent.MainParts.CD.SurfaceGui[script.Parent.SeatNumber.Value].Player.Value or script.Parent.Parent.Parent.Parent.Parent.Parent.MainParts.CD.SurfaceGui[script.Parent.SeatNumber.Value].Player.Value == "" then
		else
			if script.Parent:FindFirstChild("SeatWeld") then
			end
		end
	else
		if p.Name == script.Parent.Parent.Parent.Parent.Parent.Parent.MainParts.AB.SurfaceGui[script.Parent.SeatNumber.Value].Player.Value or script.Parent.Parent.Parent.Parent.Parent.Parent.MainParts.AB.SurfaceGui[script.Parent.SeatNumber.Value].Player.Value == "" then
			print("Allowed")
		else
			if script.Parent:FindFirstChild("SeatWeld") then
			end
		end
	end
end)
"#,
        ),
        None,
    );
}
