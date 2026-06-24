use luaur_ast::records::ast_stat::AstStat;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

use crate::type_aliases::module_ptr_module::ModulePtr;
use crate::type_aliases::scope_ptr_type::ScopePtr;

pub fn find_closest_scope_deprecated(
    module: &ModulePtr,
    nearest_statement: *const AstStat,
) -> ScopePtr {
    LUAU_ASSERT!(module.has_module_scope());

    let mut closest: ScopePtr = module.get_module_scope();

    let nearest_location = unsafe { (*nearest_statement).base.location };

    // find the scope the nearest statement belonged to.
    for (loc, sc) in &module.scopes {
        if loc.encloses(&nearest_location) && closest.location.begin <= loc.begin {
            closest = sc.clone();
        }
    }

    closest
}
