use crate::functions::as_mutable_type_pack_alt_d::as_mutable_type_pack;
use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::functions::reduce_union::reduce_union;
use crate::records::magic_function_call_context::MagicFunctionCallContext;
use crate::records::property_type::Property;
use crate::records::table_indexer::TableIndexer;
use crate::records::table_type::TableType;
use crate::records::type_pack::TypePack;
use crate::records::union_type::UnionType;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::props_type::Props;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_variant::TypePackVariant;
use alloc::vec;
use alloc::vec::Vec;

pub fn magic_pack_infer(context: &MagicFunctionCallContext) -> bool {
    let solver = unsafe { context.solver.as_ref() };
    let arena = unsafe { &mut *solver.arena };
    let builtin_types = unsafe { &*solver.builtin_types };

    let (param_types, param_tail) = flatten_type_pack_id(context.arguments);

    let mut options: Vec<TypeId> = Vec::new();
    options.reserve(param_types.len());
    for ty in param_types {
        options.push(ty);
    }

    if let Some(param_tail) = param_tail {
        let vtp = unsafe { get_type_pack_id::<VariadicTypePack>(param_tail) };
        if !vtp.is_null() {
            options.push(unsafe { (*vtp).ty });
        }
    }

    let options = reduce_union(&options);

    let result = if options.is_empty() {
        builtin_types.nilType
    } else if options.len() == 1 {
        options[0]
    } else {
        arena.add_type(UnionType { options })
    };

    let number_type = builtin_types.numberType;
    let mut props = Props::default();
    props.insert("n".to_string(), Property::rw_type_id(number_type));

    let packed_table = arena.add_type(TableType {
        props,
        indexer: Some(TableIndexer {
            index_type: number_type,
            index_result_type: result,
            is_read_only: false,
        }),
        state: crate::enums::table_state::TableState::Sealed,
        level: crate::records::type_level::TypeLevel::default(),
        scope: core::ptr::null_mut(),
        name: None,
        synthetic_name: None,
        instantiated_type_params: Vec::new(),
        instantiated_type_pack_params: Vec::new(),
        definition_module_name: Default::default(),
        definition_location: Default::default(),
        bound_to: None,
        tags: Default::default(),
        remaining_props: 0,
    });

    let table_type_pack = arena.add_type_pack_t(TypePack {
        head: vec![packed_table],
        tail: None,
    });
    let result_mut = as_mutable_type_pack(context.result);
    unsafe {
        (*result_mut).ty = TypePackVariant::Bound(table_type_pack);
    }

    true
}
