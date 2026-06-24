use crate::enums::context_error::Context;
use crate::records::unifier::Unifier;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl Unifier {
    pub fn unifier_mismatch_context(&mut self) -> Context {
        match self.variance {
            crate::enums::variance::Variance::Covariant => Context::CovariantContext,
            crate::enums::variance::Variance::Invariant => Context::InvariantContext,
            _ => {
                LUAU_ASSERT!(false);
                Context::CovariantContext
            }
        }
    }
}
