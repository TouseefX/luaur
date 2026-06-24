use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::records::type_checker_2::TypeChecker2;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::ast_type_pack::AstTypePack;

impl TypeChecker2 {
    pub fn lookup_pack_annotation(&self, annotation: *mut AstTypePack) -> Option<TypePackId> {
        let tp = unsafe {
            (*self.module)
                .ast_resolved_type_packs
                .find(&(annotation as *const AstTypePack))
        };
        if let Some(tp) = tp {
            Some(unsafe { follow_type_pack_id(*tp) })
        } else {
            None
        }
    }
}
