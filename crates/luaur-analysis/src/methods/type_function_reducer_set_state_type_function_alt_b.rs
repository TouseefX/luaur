use crate::enums::type_function_instance_state::TypeFunctionInstanceState;
use crate::records::type_function_reducer::TypeFunctionReducer;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeFunctionReducer {
    pub fn set_state_type_pack_id_type_function_instance_state(
        &self,
        _tp: TypePackId,
        _state: TypeFunctionInstanceState,
    ) {
    }
}
