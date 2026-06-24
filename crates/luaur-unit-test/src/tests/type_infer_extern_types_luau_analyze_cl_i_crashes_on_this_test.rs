//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:22:type_infer_extern_types_luau_analyze_cl_i_crashes_on_this_test`
//! Source: `tests/TypeInfer.externTypes.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_extern_types_luau_analyze_cl_i_crashes_on_this_test

#[cfg(test)]
#[test]
fn type_infer_extern_types_luau_analyze_cli_crashes_on_this_test() {
    use crate::records::extern_type_fixture::ExternTypeFixture;
    use alloc::string::String;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local CircularQueue = {}
CircularQueue.__index = CircularQueue

function CircularQueue:new()
	local newCircularQueue = {
		head = nil,
	}
	setmetatable(newCircularQueue, CircularQueue)

	return newCircularQueue
end

function CircularQueue:push()
	local newListNode

	if self.head then
		newListNode = {
			prevNode = self.head.prevNode,
			nextNode = self.head,
		}
		newListNode.prevNode.nextNode = newListNode
		newListNode.nextNode.prevNode = newListNode
	end
end

return CircularQueue

    "#,
        ),
        None,
    );
}
