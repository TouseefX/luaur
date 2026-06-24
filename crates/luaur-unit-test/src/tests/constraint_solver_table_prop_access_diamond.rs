//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/ConstraintSolver.test.cpp:51:constraint_solver_table_prop_access_diamond`
//! Source: `tests/ConstraintSolver.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/ConstraintSolver.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/ConstraintSolver.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item constraint_solver_table_prop_access_diamond

#[cfg(test)]
#[test]
fn constraint_solver_table_prop_access_diamond() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        export type ItemDetails = { Id: number }

        export type AssetDetails = ItemDetails & {}
        export type BundleDetails = ItemDetails & {}

        export type CatalogPage = { AssetDetails | BundleDetails }

        local function isRestricted(item: number) end

        -- Clear all item tiles and create new ones for the items in the specified page
        local function displayPage(catalogPage: CatalogPage)
            for _, itemDetails in catalogPage do
                if isRestricted(itemDetails.Id) then
                    continue
                end
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
