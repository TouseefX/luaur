//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_exhaustive_to_string_of_cyclic_table() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::to_string_options::ToStringOptions;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        local Vec3 = {}
        Vec3.__index = Vec3
        function Vec3.new()
            return setmetatable({x=0, y=0, z=0}, Vec3)
        end

        export type Vec3 = typeof(Vec3.new())

        local thefun: any = function(self, o) return self end

        local multiply: ((Vec3, Vec3) -> Vec3) & ((Vec3, number) -> Vec3) = thefun

        Vec3.__mul = multiply

        local a = Vec3.new()
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut opts = ToStringOptions::to_string_options(true);
    let a = to_string_type_id_to_string_options(
        fixture.base.require_type_string(&String::from("a")),
        &mut opts,
    );

    assert_eq!(None, a.find("CYCLE"));
    assert_eq!(None, a.find("TRUNCATED"));

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "t2 where t1 = { __index: t1, __mul: ((t2, number) -> t2) & ((t2, t2) -> t2), new: () -> t2 } ; t2 = { @metatable t1, { x: number, y: number, z: number } }",
            a
        );
    } else {
        assert_eq!(
            "t2 where t1 = {| __index: t1, __mul: ((t2, number) -> t2) & ((t2, t2) -> t2), new: () -> t2 |} ; t2 = { @metatable t1, { x: number, y: number, z: number } }",
            a
        );
    }
}
