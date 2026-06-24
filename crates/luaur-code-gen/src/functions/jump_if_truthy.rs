use crate::enums::condition_x_64::ConditionX64;
use crate::functions::jump_if_tag_is::jump_if_tag_is;
use crate::functions::jump_if_tag_is_not::jump_if_tag_is_not;
use crate::functions::luau_reg_value_int::luau_reg_value_int;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::label::Label;
use luaur_vm::enums::lua_type::lua_Type;

pub fn jump_if_truthy(
    build: &mut AssemblyBuilderX64,
    ri: i32,
    target: &mut Label,
    fallthrough: &mut Label,
) {
    jump_if_tag_is(build, ri, lua_Type::LUA_TNIL, fallthrough); // false if nil
    jump_if_tag_is_not(build, ri, lua_Type::LUA_TBOOLEAN, target); // true if not nil or boolean

    build.cmp(luau_reg_value_int(ri), 0.into());
    build.jcc(ConditionX64::NotEqual, target); // true if boolean value is 'true'
}
