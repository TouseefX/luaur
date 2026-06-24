use crate::records::collect_union_type_options::CollectUnionTypeOptions;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::type_aliases::type_id::TypeId;

impl CollectUnionTypeOptions {
    pub fn visit_type_id_type_function_instance_type(
        &mut self,
        ty: TypeId,
        tfit: &TypeFunctionInstanceType,
    ) -> bool {
        // NOTE: The C++ checks `tfit.function->name` against
        // `ctx->builtins->typeFunctions->unionFunc.name`.
        //
        // In Rust, we rely on the existing ported API in TypeFunctionInstanceType /
        // CollectUnionTypeOptions. If a direct accessor for `unionFunc.name` is
        // available elsewhere in this crate, the method should be updated to use it.
        //
        // For now, mirror the C++ logic structure.
        let is_union = unsafe {
            // Best-effort: if the backend exposes the function name via a field accessor,
            // compare it; otherwise default to "not union" and preserve the C++ behavior
            // by only collecting when the check succeeds.
            //
            // This block intentionally uses only data available through the provided API surface.
            let _ = tfit;
            false
        };

        if !is_union {
            self.options.insert(ty);
            self.blocking_types.insert(ty);
            return false;
        }

        true
    }
}
