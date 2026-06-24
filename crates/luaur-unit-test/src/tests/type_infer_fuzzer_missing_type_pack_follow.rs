//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2548:type_infer_fuzzer_missing_type_pack_follow`
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
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - translates_to -> rust_item type_infer_fuzzer_missing_type_pack_follow

#[cfg(test)]
#[test]
fn type_infer_fuzzer_missing_type_pack_follow() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
local _ = {[0]=_,}
while _ do
do
local l2 = require(module0)
end
end
do end
function _(l0:typeof(_),l0,l0)
local l0 = require(module0)
_()(l0(),_,_(_())((_)))
do end
end
_()(_(if nil then _))("",_,_(_,(_)))
do end
    "#,
        ),
        None,
    );
    assert!(!result.errors.is_empty(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
local _ = {_,}
while _ do
do
do end
end
end
_ = nil
function _(l0,l0,l0)
local l0 = require(module0)
_()(_(),_,_(_())(_,true)(_,_),l0)
do end
end
_()(_())("",_.n0,_,_(_,true,(_)))
do end
    "#,
        ),
        None,
    );
    assert!(!result.errors.is_empty(), "{:?}", result.errors);
}
