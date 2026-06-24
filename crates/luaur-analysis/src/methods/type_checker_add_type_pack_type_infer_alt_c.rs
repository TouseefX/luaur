use crate::records::type_checker::TypeChecker;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeChecker {
    /// C++ `TypePackId TypeChecker::addTypePack(const std::vector<TypeId>& ty)` (TypeInfer.cpp:5600):
    /// `return addTypePack(ty, std::nullopt);`
    pub fn add_type_pack_vector_type_id(&mut self, ty: &alloc::vec::Vec<TypeId>) -> TypePackId {
        self.add_type_pack_vector_type_id_optional_type_pack_id(ty, None)
    }
}
