//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_another_thing_from_roact() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Map<K, V> = { [K]: V }
        type Set<T> = { [T]: boolean }

        type FiberRoot = {
            pingCache: Map<Wakeable, (Set<any> | Map<Wakeable, Set<any>>)> | nil,
        }

        type Wakeable = {
            andThen: (self: Wakeable) -> nil | Wakeable,
        }

        local function attachPingListener(root: FiberRoot, wakeable: Wakeable, lanes: number)
            local pingCache: Map<Wakeable, (Set<any> | Map<Wakeable, Set<any>>)> | nil = root.pingCache
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
