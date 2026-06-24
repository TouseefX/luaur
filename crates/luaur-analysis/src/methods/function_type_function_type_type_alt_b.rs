use crate::records::function_definition::FunctionDefinition;
use crate::records::function_type::FunctionType;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::type_pack_id::TypePackId;

impl FunctionType {
    pub fn function_type_type_level_type_pack_id_type_pack_id_optional_function_definition_bool(
        &mut self,
        level: TypeLevel,
        arg_types: TypePackId,
        ret_types: TypePackId,
        defn: Option<FunctionDefinition>,
        has_self: bool,
    ) {
        self.level = level;
        self.arg_types = arg_types;
        self.ret_types = ret_types;
        self.definition = defn;
        self.has_self = has_self;
    }
}
