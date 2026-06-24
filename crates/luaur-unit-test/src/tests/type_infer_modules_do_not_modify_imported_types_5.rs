//! Ported from `tests/TypeInfer.modules.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_modules_do_not_modify_imported_types_5() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
export type Type = {x: number, y: number}
local arrayops = {}
function arrayops.foo(x: Type) end
return arrayops
    "#,
        ),
    );
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
local arrayops = require(game.A)

local tbl = {}
tbl.a = 2
function tbl:foo(b: number, c: number)
    -- introduce boundTo TableType to imported type
    self.x.a = 2
    arrayops.foo(self.x)
end
-- this alias decreases function type level and causes a demotion of its type
type Table = typeof(tbl)
"#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
