use crate::records::collect_union_type_options::CollectUnionTypeOptions;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;

impl CollectUnionTypeOptions {
    pub fn visit_type_id_union_type(&mut self, _ty: TypeId, _ut: &UnionType) -> bool {
        // If we have something like:
        //
        //  union<A | B, C | D>
        //
        // We probably just want to consider this to be the same as
        //
        //   union<A, B, C, D>
        true
    }
}
