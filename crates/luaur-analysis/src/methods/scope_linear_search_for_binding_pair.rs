use crate::records::binding::Binding;
use crate::records::scope::Scope;
use crate::records::symbol::Symbol;
use alloc::string::String;

impl Scope {
    pub fn linear_search_for_binding_pair(
        &self,
        name: &String,
        traverse_scope_chain: bool,
    ) -> Option<(Symbol, Binding)> {
        let mut scope: Option<&Scope> = Some(self);

        while let Some(current_scope) = scope {
            for (symbol, binding) in &current_scope.bindings {
                let sym_name =
                    unsafe { core::ffi::CStr::from_ptr(symbol.c_str()).to_string_lossy() };
                if sym_name == *name {
                    return Some((symbol.clone(), binding.clone()));
                }
            }

            if !traverse_scope_chain {
                break;
            }

            scope = current_scope.parent.as_ref().map(|p| p.as_ref());
        }

        None
    }
}
