use crate::records::binding::Binding;
use crate::records::frontend::Frontend;
use crate::records::scope::Scope;
use crate::records::source_module::SourceModule;
use crate::records::symbol::Symbol;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use alloc::sync::Arc;
use luaur_ast::records::location::Location;
use luaur_config::records::config::Config;
use std::ffi::CString;

impl Frontend {
    pub fn get_module_environment(
        &self,
        module: &SourceModule,
        config: &Config,
        for_autocomplete: bool,
    ) -> ScopePtr {
        let mut result = if for_autocomplete {
            self.globals_for_autocomplete.global_scope.clone()
        } else {
            self.globals.global_scope.clone()
        };

        if let Some(environment_name) = &module.environment_name {
            result = self.get_environment_scope(environment_name.clone());
        }

        if !config.globals.is_empty() {
            result = Arc::new(Scope::new(&result, 0));

            for global in &config.globals {
                let global_cstr =
                    CString::new(global.as_str()).expect("global names cannot contain NUL");
                let name = module.names.get(global_cstr.as_ptr());

                if !name.value.is_null() {
                    let scope = Arc::get_mut(&mut result)
                        .expect("new module environment scope is uniquely owned");
                    let binding =
                        scope
                            .bindings
                            .entry(Symbol::from_global(name))
                            .or_insert(Binding {
                                type_id: core::ptr::null(),
                                location: Location::default(),
                                deprecated: false,
                                deprecated_suggestion: alloc::string::String::new(),
                                documentation_symbol: None,
                            });
                    binding.type_id = unsafe { (*self.builtin_types).anyType };
                }
            }
        }

        result
    }
}
