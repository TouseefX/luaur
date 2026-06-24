use crate::records::type_function_context::TypeFunctionContext;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_once_visitor::TypeOnceVisitor;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;
use core::ffi::c_void;
use core::ptr::NonNull;
use luaur_common::records::dense_hash_set::DenseHashSet;

#[derive(Debug, Clone)]
pub struct CollectUnionTypeOptions {
    pub base: TypeOnceVisitor,
    pub(crate) ctx: NonNull<TypeFunctionContext>,
    pub(crate) options: DenseHashSet<TypeId>,
    pub(crate) blocking_types: DenseHashSet<TypeId>,
}

impl CollectUnionTypeOptions {
    pub fn collect_union_type_options(ctx: NonNull<TypeFunctionContext>) -> Self {
        Self {
            base: TypeOnceVisitor::new("CollectUnionTypeOptions".to_string(), true),
            ctx,
            options: DenseHashSet::new(core::ptr::null_mut()),
            blocking_types: DenseHashSet::new(core::ptr::null_mut()),
        }
    }

    pub fn visit_collect_union_type_options_type_id(&mut self, ty: TypeId) -> bool {
        self.options.insert(ty);
        // if (isPending(ty, ctx->solver))
        //     blockingTypes.insert(ty);
        // This depends on isPending which is not yet translated; keep conservative.
        let _ = ty;
        false
    }

    pub fn visit_collect_union_type_options_type_pack_id(&mut self, _tp: TypePackId) -> bool {
        false
    }

    pub fn visit_collect_union_type_options_type_id_union_type(
        &mut self,
        _ty: TypeId,
        _ut: &crate::records::union_type::UnionType,
    ) -> bool {
        true
    }

    pub fn visit_collect_union_type_options_type_id_type_function_instance_type(
        &mut self,
        ty: TypeId,
        tfit: &TypeFunctionInstanceType,
    ) -> bool {
        // if (tfit.function->name != ctx->builtins->typeFunctions->unionFunc.name)
        // {
        //     options.insert(ty);
        //     blockingTypes.insert(ty);
        //     return false;
        // }
        //
        // This depends on builtin type function names and unionFunc; keep conservative.
        let _ = (ty, tfit);
        true
    }
}
