use crate::records::find_all_union_members::FindAllUnionMembers;
use crate::records::free_type::FreeType;
use crate::type_aliases::type_id::TypeId;

impl FindAllUnionMembers {
    pub fn visit_type_id_free_type(&mut self, _ty: TypeId, _ftv: &FreeType) -> bool {
        self.blocked_tys.insert_type_id(_ty);
        false
    }
}
