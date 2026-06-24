use crate::records::non_strict_context::NonStrictContext;
use crate::type_aliases::def_id_def::DefId;
use crate::type_aliases::type_id::TypeId;

use crate::functions::collect_operands::collect_operands;

impl NonStrictContext {
    pub fn add_context(&mut self, def: &DefId, ty: TypeId) {
        let mut defs: Vec<DefId> = Vec::new();
        collect_operands(*def, &mut defs);
        for def in defs {
            self.context
                .insert(def as *const crate::records::def::Def, ty);
        }
    }
}
