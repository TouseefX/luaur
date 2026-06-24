//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_gh_1632_no_infinite_recursion_in_normalization() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Node<T> = {
            value: T,
            next: Node<T>?,
            -- remove `prev`, solves issue
            prev: Node<T>?,
        };

        type List<T> = {
            head: Node<T>?
        }

        local function IsFront(list: List<any>, nodeB: Node<any>)
            -- remove if statement below, solves issue
            if (list.head == nodeB) then
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
