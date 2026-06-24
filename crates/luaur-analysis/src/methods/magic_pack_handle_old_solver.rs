use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::functions::reduce_union::reduce_union;
use crate::records::module::Module;
use crate::records::property_type::Property;
use crate::records::scope::Scope;
use crate::records::table_indexer::TableIndexer;
use crate::records::table_type::TableType;
use crate::records::type_checker::TypeChecker;
use crate::records::type_pack::TypePack;
use crate::records::union_type::UnionType;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::records::with_predicate::WithPredicate;
use crate::type_aliases::props_type::Props;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::string::ToString;
use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;
use luaur_ast::records::ast_expr_call::AstExprCall;

pub fn magic_pack_handle_old_solver(
    typechecker: &mut TypeChecker,
    scope: &Arc<Scope>,
    _expr: &AstExprCall,
    with_predicate: WithPredicate<TypePackId>,
) -> Option<WithPredicate<TypePackId>> {
    let param_pack = with_predicate.r#type;

    let module = typechecker.current_module.as_ref()?;
    let arena = unsafe { &mut (*(Arc::as_ptr(module) as *mut Module)).internal_types };

    let (param_types, param_tail) = flatten_type_pack_id(param_pack);

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

    // table.pack()         -> {| n: number, [number]: nil |}
    // table.pack(1)        -> {| n: number, [number]: number |}
    // table.pack(1, "foo") -> {| n: number, [number]: number | string |}
    let result = if options.is_empty() {
        typechecker.nil_type
    } else if options.len() == 1 {
        options[0]
    } else {
        arena.add_type(UnionType { options })
    };

    let mut props = Props::default();
    props.insert(
        "n".to_string(),
        Property::rw_type_id(typechecker.number_type),
    );

    let packed_table = arena.add_type(TableType {
        props,
        indexer: Some(TableIndexer {
            index_type: typechecker.number_type,
            index_result_type: result,
            is_read_only: false,
        }),
        state: crate::enums::table_state::TableState::Sealed,
        level: scope.level,
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

    let result_pack = arena.add_type_pack_t(TypePack {
        head: vec![packed_table],
        tail: None,
    });
    Some(WithPredicate::with_predicate_t(result_pack))
}
