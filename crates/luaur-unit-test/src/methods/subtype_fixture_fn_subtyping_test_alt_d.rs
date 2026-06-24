use crate::records::subtype_fixture::SubtypeFixture;
use luaur_analysis::records::function_type::FunctionType;
use luaur_analysis::type_aliases::type_id::TypeId;
use luaur_analysis::type_aliases::type_pack_variant::TypePackVariant;

impl SubtypeFixture {
    pub fn fn_item_initializer_list_type_id_type_pack_variant_initializer_list_type_id_type_pack_variant(
        &mut self,
        arg_head: Vec<TypeId>,
        arg_tail: TypePackVariant,
        ret_head: Vec<TypeId>,
        ret_tail: TypePackVariant,
    ) -> TypeId {
        let arg_pack = self.pack_initializer_list_type_id_type_pack_variant(arg_head, arg_tail);
        let ret_pack = self.pack_initializer_list_type_id_type_pack_variant(ret_head, ret_tail);
        self.arena.add_type(FunctionType::function_type_new(
            arg_pack, ret_pack, None, false,
        ))
    }
}
