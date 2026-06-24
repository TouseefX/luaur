use luaur_common::records::dense_hash_set::DenseHashSet;

use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_function_instance_type_pack::TypeFunctionInstanceTypePack;
use crate::records::type_once_visitor::TypeOnceVisitor;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

#[derive(Debug, Clone)]
pub struct TypeFunctionFinder {
    pub(crate) base: TypeOnceVisitor,
    pub(crate) mentioned_functions: DenseHashSet<TypeId>,
    pub(crate) mentioned_function_packs: DenseHashSet<TypePackId>,
}

impl TypeFunctionFinder {
    pub fn new() -> Self {
        let base = TypeOnceVisitor::new("TypeFunctionFinder".to_string(), true);

        Self {
            base,
            mentioned_functions: DenseHashSet::new(core::ptr::null_mut()),
            mentioned_function_packs: DenseHashSet::new(core::ptr::null_mut()),
        }
    }

    pub fn type_function_finder(&mut self) -> Self {
        TypeFunctionFinder::new()
    }

    pub fn visit_type_function_finder_type_id(
        &mut self,
        ty: TypeId,
        _instance: &TypeFunctionInstanceType,
    ) -> bool {
        self.mentioned_functions.insert(ty);
        true
    }

    pub fn visit_type_function_finder_type_pack_id(
        &mut self,
        tp: TypePackId,
        _instance: &TypeFunctionInstanceTypePack,
    ) -> bool {
        self.mentioned_function_packs.insert(tp);
        true
    }
}
