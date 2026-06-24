use crate::records::dfg_scope::DfgScope;
use crate::records::push_scope::PushScope;
use crate::type_aliases::scope_stack::ScopeStack;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl PushScope {
    pub fn push_scope(_stack: &mut ScopeStack, _scope: *mut DfgScope) -> Self {
        // `scope` should never be `nullptr` here.
        LUAU_ASSERT!(!_scope.is_null());

        let previous_size = _stack.len();
        _stack.push(_scope);

        PushScope {
            stack: _stack,
            previous_size,
        }
    }
}
