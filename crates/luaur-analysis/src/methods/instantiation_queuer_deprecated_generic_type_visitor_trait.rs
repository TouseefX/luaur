use crate::records::extern_type::ExternType;
use crate::records::generic_type_visitor::{GenericTypeVisitor, GenericTypeVisitorTrait};
use crate::records::instantiation_queuer_deprecated::InstantiationQueuerDeprecated;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::type_aliases::type_id::TypeId;
use core::ffi::c_void;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl GenericTypeVisitorTrait for InstantiationQueuerDeprecated {
    type Seen = DenseHashSet<*mut c_void>;

    fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
        &mut self.base.base
    }

    fn visit_type_id_pending_expansion_type(
        &mut self,
        ty: TypeId,
        petv: &PendingExpansionType,
    ) -> bool {
        InstantiationQueuerDeprecated::visit_type_id_pending_expansion_type(self, ty, petv)
    }

    fn visit_type_id_type_function_instance_type(
        &mut self,
        ty: TypeId,
        tfit: &TypeFunctionInstanceType,
    ) -> bool {
        InstantiationQueuerDeprecated::visit_type_id_type_function_instance_type(self, ty, tfit)
    }

    fn visit_type_id_extern_type(&mut self, ty: TypeId, etv: &ExternType) -> bool {
        InstantiationQueuerDeprecated::visit_type_id_extern_type(self, ty, etv)
    }
}
