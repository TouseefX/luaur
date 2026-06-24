use crate::records::function_definition::FunctionDefinition;
use crate::records::function_type::FunctionType;
use crate::type_aliases::type_pack_id::TypePackId;

impl FunctionType {
    pub fn function_type_type_pack_id_type_pack_id_optional_function_definition_bool(
        &mut self,
        arg_types: TypePackId,
        ret_types: TypePackId,
        defn: Option<FunctionDefinition>,
        has_self: bool,
    ) {
        self.definition = defn;
        self.arg_types = arg_types;
        self.ret_types = ret_types;
        self.has_self = has_self;
    }
}
