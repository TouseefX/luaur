use crate::records::type_checker::TypeChecker;
use crate::records::type_pack::TypePack;
use crate::records::type_pack_var::TypePackVar;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeChecker {
    /// C++ `TypePackId TypeChecker::addTypePack(const std::vector<TypeId>& ty, std::optional<TypePackId> tail)`
    /// (TypeInfer.cpp:5605): `return addTypePack(TypePackVar(TypePack{ty, tail}));`
    pub fn add_type_pack_vector_type_id_optional_type_pack_id(
        &mut self,
        ty: &alloc::vec::Vec<TypeId>,
        tail: Option<TypePackId>,
    ) -> TypePackId {
        self.add_type_pack_type_pack_var(TypePackVar::from(TypePack {
            head: ty.clone(),
            tail,
        }))
    }
}
