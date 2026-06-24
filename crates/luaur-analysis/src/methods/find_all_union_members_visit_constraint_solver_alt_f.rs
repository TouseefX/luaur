use crate::records::find_all_union_members::FindAllUnionMembers;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;

impl FindAllUnionMembers {
    pub fn visit_type_id_union_type(&mut self, _ty: TypeId, _ut: &UnionType) -> bool {
        true
    }
}
