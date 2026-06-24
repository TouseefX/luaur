use crate::functions::baseof::baseof;
use crate::functions::get_base_symbol::get_base_symbol;
use crate::records::field::Field;
use crate::records::symbol::Symbol;
use crate::records::type_checker::TypeChecker;
use crate::type_aliases::l_value::{LValue, LValueMember};
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeChecker {
    pub fn resolve_l_value_scope_ptr_l_value(
        &mut self,
        scope: ScopePtr,
        lvalue: &LValue,
    ) -> Option<TypeId> {
        // We want to be walking the Scope parents.
        // We'll also want to walk up the LValue path. As we do this, we need to save each LValue because we must walk back.
        let symbol: Symbol = get_base_symbol(lvalue);

        let mut current_scope: Option<ScopePtr> = Some(scope.clone());
        while let Some(cur) = current_scope.clone() {
            let mut found: Option<TypeId> = None;

            let mut top_lvalue: *const LValue = core::ptr::null();

            {
                let mut curr: *const LValue = lvalue;
                while !curr.is_null() {
                    if let Some(it) = cur.refinements.get(unsafe { &*curr }) {
                        found = Some(*it);
                        top_lvalue = curr;
                        break;
                    }
                    curr = baseof(unsafe { &*curr });
                }
                if found.is_none() {
                    // top_lvalue stays null (the loop above terminated without a match).
                    top_lvalue = curr;
                }
            }

            if found.is_none() {
                // Should not be using scope->lookup. This is already recursive.
                if let Some(binding) = cur.bindings.get(&symbol) {
                    found = Some(binding.type_id);
                } else {
                    // Nothing exists in this Scope. Just skip and try the parent one.
                    current_scope = cur.parent.clone();
                    continue;
                }
            }

            // We need to walk the l-value path in reverse, so we collect components into a vector
            let mut child_keys: alloc::vec::Vec<*const LValue> = alloc::vec::Vec::new();

            {
                let mut curr: *const LValue = lvalue;
                while curr != top_lvalue {
                    child_keys.push(curr);
                    curr = baseof(unsafe { &*curr });
                }
            }

            for &key_ptr in child_keys.iter().rev() {
                let key: &LValue = unsafe { &*key_ptr };

                // Symbol can happen. Skip.
                if !crate::functions::get_l_value::get_l_value::<Symbol>(key).is_null() {
                    continue;
                } else if let Some(field) = <Field as LValueMember>::get_if(key) {
                    found = self.get_index_type_from_type(
                        scope.clone(),
                        found.unwrap(),
                        &field.key,
                        &Location::default(),
                        false,
                    );
                    if found.is_none() {
                        return None; // Turns out this type doesn't have the property at all. We're done.
                    }
                } else {
                    LUAU_ASSERT!(false); // "New LValue alternative not handled here."
                }
            }

            return found;
        }

        // No entry for it at all. Can happen when LValue root is a global.
        None
    }
}
