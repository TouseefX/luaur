use crate::records::fixture::Fixture;
use alloc::boxed::Box;
use luaur_analysis::records::type_function::TypeFunction;

#[derive(Debug)]
#[repr(C)]
pub struct TypeFunctionFixture {
    pub base: Fixture,
    pub swap_function: Box<TypeFunction>,
}
