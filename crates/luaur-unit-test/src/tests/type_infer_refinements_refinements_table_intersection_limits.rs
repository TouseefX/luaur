//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2454:type_infer_refinements_refinements_table_intersection_limits`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> function checkpoint (Analysis/src/ConstraintGenerator.cpp)
//!   - translates_to -> rust_item type_infer_refinements_refinements_table_intersection_limits

#[cfg(test)]
#[test]
fn type_infer_refinements_refinements_table_intersection_limits() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
--!strict
type Dir = {
    a: number?, b: number?, c: number?, d: number?, e: number?, f: number?,
    g: number?, h: number?, i: number?, j: number?, k: number?, l: number?,
    m: number?, n: number?, o: number?, p: number?, q: number?, r: number?,
}

local function test(dirs: {Dir})
    for k, dir in dirs
        local success, message = pcall(function()
            assert(dir.a == nil or type(dir.a) == "number")
            assert(dir.b == nil or type(dir.b) == "number")
            assert(dir.c == nil or type(dir.c) == "number")
            assert(dir.d == nil or type(dir.d) == "number")
            assert(dir.e == nil or type(dir.e) == "number")
            assert(dir.f == nil or type(dir.f) == "number")
            assert(dir.g == nil or type(dir.g) == "number")
            assert(dir.h == nil or type(dir.h) == "number")
            assert(dir.i == nil or type(dir.i) == "number")
            assert(dir.j == nil or type(dir.j) == "number")
            assert(dir.k == nil or type(dir.k) == "number")
            assert(dir.l == nil or type(dir.l) == "number")
            assert(dir.m == nil or type(dir.m) == "number")
            assert(dir.n == nil or type(dir.n) == "number")
            assert(dir.o == nil or type(dir.o) == "number")
            assert(dir.p == nil or type(dir.p) == "number")
            assert(dir.q == nil or type(dir.q) == "number")
            assert(dir.r == nil or type(dir.r) == "number")
            assert(dir.t == nil or type(dir.t) == "number")
            assert(dir.u == nil or type(dir.u) == "number")
            assert(dir.v == nil or type(dir.v) == "number")
            local checkpoint = dir

            checkpoint.w = 1
        end)
        assert(success)
    end
end
    "#,
        ),
        None,
    );
}
