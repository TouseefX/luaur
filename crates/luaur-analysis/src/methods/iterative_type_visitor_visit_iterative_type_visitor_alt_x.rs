use crate::records::free_type_pack::FreeTypePack;
use crate::records::iterative_type_visitor::IterativeTypeVisitor;
use crate::type_aliases::type_pack_id::TypePackId;

impl IterativeTypeVisitor {
    pub fn visit_type_pack_id_free_type_pack(
        &mut self,
        tp: TypePackId,
        _ftp: &FreeTypePack,
    ) -> bool {
        self.visit_type_pack_id(tp)
    }
}
