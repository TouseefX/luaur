use crate::records::type_function_type_pack_var::TypeFunctionTypePackVar;

impl TypeFunctionTypePackVar {
    pub fn operator_eq(&self, rhs: &TypeFunctionTypePackVar) -> bool {
        let mut seen = crate::records::are_equal_state::AreEqualState {
            seen: Default::default(),
            recursion_count: 0,
        };
        crate::functions::are_equal_type_function_runtime_alt_p::are_equal_are_equal_state_type_function_type_pack_var_type_function_type_pack_var(
            &mut seen,
            self,
            rhs,
        )
    }
}
