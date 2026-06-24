use crate::type_aliases::scope_ptr_type::ScopePtr;
use luaur_ast::records::ast_name_table::AstNameTable;

/// C++ `static void fillBuiltinGlobals(LintContext& context, const AstNameTable& names, const ScopePtr& env)`.
pub fn fill_builtin_globals(
    context: &mut crate::records::lint_context::LintContext,
    names: &AstNameTable,
    env: &ScopePtr,
) {
    let mut current = env.clone();
    loop {
        for (symbol, binding) in &current.bindings {
            let name = names.get(symbol.c_str());

            if name.value.is_null() {
                continue;
            }

            let global = context.builtin_globals.get_or_insert(name);
            global.r#type = binding.type_id;

            if binding.deprecated {
                global.deprecated = if binding.deprecated_suggestion.is_empty() {
                    Some(core::ptr::null())
                } else {
                    Some(binding.deprecated_suggestion.as_ptr() as *const core::ffi::c_char)
                };
            }
        }

        if let Some(ref parent) = current.parent {
            current = parent.clone();
        } else {
            break;
        }
    }
}
