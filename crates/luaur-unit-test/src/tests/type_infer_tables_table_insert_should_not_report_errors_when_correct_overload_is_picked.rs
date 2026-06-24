//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5161:type_infer_tables_table_insert_should_not_report_errors_when_correct_overload_is_picked`
//! Source: `tests/TypeInfer.tables.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.tables.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.tables.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_tables_table_insert_should_not_report_errors_when_correct_overload_is_picked

#[cfg(test)]
#[test]
fn type_infer_tables_table_insert_should_not_report_errors_when_correct_overload_is_picked() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
type cs = { GetTagged : (cs, string) -> any}
local destroyQueue: {any} = {} -- pair of (time, coin)
local tick : () -> any
local CS : cs
local DESTROY_DELAY
local function SpawnCoin()
	local spawns = CS:GetTagged('CoinSpawner')
	local n : any
	local StartPos = spawns[n].CFrame
	local Coin = script.Coin:Clone()
	Coin.CFrame = StartPos
	Coin.Parent = workspace.Coins

	table.insert(destroyQueue, {tick() + DESTROY_DELAY, Coin})
end
"#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
