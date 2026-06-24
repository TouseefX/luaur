use crate::enums::condition_x_64::ConditionX64;
use crate::functions::luau_reg_tag::luau_reg_tag;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::label::Label;
use luaur_vm::enums::lua_type::lua_Type;

pub fn jump_if_tag_is(build: &mut AssemblyBuilderX64, ri: i32, tag: lua_Type, label: &mut Label) {
    build.cmp(luau_reg_tag(ri), (tag as i32).into());
    build.jcc(ConditionX64::Equal, label);
}
