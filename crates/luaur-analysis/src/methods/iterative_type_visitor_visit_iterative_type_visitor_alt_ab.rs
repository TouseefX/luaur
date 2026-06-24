use crate::records::iterative_type_visitor::IterativeTypeVisitor;
use crate::type_aliases::type_pack_id::TypePackId;

impl IterativeTypeVisitor {
    pub fn visit_type_pack_id_variadic_type_pack(
        &mut self,
        tp: TypePackId,
        _vtp: &crate::records::variadic_type_pack::VariadicTypePack,
    ) -> bool {
        self.visit_type_pack_id(tp)
    }
}
