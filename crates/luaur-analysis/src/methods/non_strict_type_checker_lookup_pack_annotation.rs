use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::ast_type_pack::AstTypePack;

impl NonStrictTypeChecker {
    pub fn lookup_pack_annotation(&self, annotation: *mut AstTypePack) -> Option<TypePackId> {
        let module = unsafe { &*self.module };
        let tp = module
            .ast_resolved_type_packs
            .find(&(annotation as *const AstTypePack));
        if let Some(tp) = tp {
            Some(unsafe { follow_type_pack_id(*tp) })
        } else {
            None
        }
    }
}
