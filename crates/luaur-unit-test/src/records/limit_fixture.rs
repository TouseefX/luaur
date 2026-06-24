use crate::records::builtins_fixture::BuiltinsFixture;
use crate::type_aliases::scoped_fast_int::ScopedFastInt;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct LimitFixture {
    #[cfg(debug_assertions)]
    pub luau_type_infer_recursion_limit: ScopedFastInt,
    pub base: BuiltinsFixture,
}

impl Default for LimitFixture {
    fn default() -> Self {
        Self {
            #[cfg(debug_assertions)]
            luau_type_infer_recursion_limit: ScopedFastInt::new(
                &luaur_common::FInt::LuauTypeInferRecursionLimit,
                100,
            ),
            base: BuiltinsFixture::default(),
        }
    }
}
