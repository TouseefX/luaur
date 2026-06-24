use crate::records::type_function_serializer::TypeFunctionSerializer;

impl TypeFunctionSerializer {
    pub fn has_exceeded_iteration_limit(&self) -> bool {
        let limit = luaur_common::DFInt::LuauTypeFunctionSerdeIterationLimit.get();
        limit != 0 && self.steps + self.queue.len() as i32 >= limit
    }
}
