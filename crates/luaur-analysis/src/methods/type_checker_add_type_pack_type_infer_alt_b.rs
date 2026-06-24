use crate::records::type_checker::TypeChecker;
use crate::records::type_pack::TypePack;
use crate::records::type_pack_var::TypePackVar;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeChecker {
    /// C++ `TypePackId TypeChecker::addTypePack(TypePack&& tp)` (TypeInfer.cpp:5595):
    /// `return addTypePack(TypePackVar(std::move(tp)));`
    pub fn add_type_pack_type_pack(&mut self, tp: TypePack) -> TypePackId {
        self.add_type_pack_type_pack_var(TypePackVar::from(tp))
    }
}
