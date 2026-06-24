use crate::functions::create_some_extern_types::create_some_extern_types;
use crate::records::fixture::Fixture;
use crate::records::simplify_fixture::SimplifyFixture;
use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use luaur_analysis::functions::try_get_global_binding::try_get_global_binding;
use luaur_analysis::records::blocked_type::BlockedType;
use luaur_analysis::records::function_type::FunctionType;
use luaur_analysis::records::generic_type::GenericType;
use luaur_analysis::records::pending_expansion_type::PendingExpansionType;
use luaur_analysis::records::scope::Scope;
use luaur_analysis::records::singleton_type::SingletonType;
use luaur_analysis::records::string_singleton::StringSingleton;
use luaur_analysis::records::to_string_options::ToStringOptions;
use luaur_analysis::records::type_arena::TypeArena;
use luaur_analysis::type_aliases::singleton_variant::SingletonVariant;
use luaur_ast::records::ast_name::AstName;
use luaur_common::FFlag;

impl SimplifyFixture {
    pub fn new() -> Self {
        let null_type = core::ptr::null();
        let null_pack = core::ptr::null();

        let mut fixture = Self {
            base: Fixture::fixture_bool(false),
            arena: TypeArena::default(),
            opts: ToStringOptions::default(),
            scope: Arc::new(Scope::scope_type_pack_id(null_pack)),
            any_ty: null_type,
            unknown_ty: null_type,
            never_ty: null_type,
            error_ty: null_type,
            function_ty: null_type,
            table_ty: null_type,
            number_ty: null_type,
            string_ty: null_type,
            boolean_ty: null_type,
            nil_ty: null_type,
            class_ty: null_type,
            true_ty: null_type,
            false_ty: null_type,
            truthy_ty: null_type,
            falsy_ty: null_type,
            free_ty: null_type,
            generic_ty: null_type,
            blocked_ty: null_type,
            pending_ty: null_type,
            hello_ty: null_type,
            world_ty: null_type,
            empty_type_pack: null_pack,
            fn1_ty: null_type,
            fn2_ty: null_type,
            parent_class_ty: null_type,
            child_class_ty: null_type,
            another_child_class_ty: null_type,
            unrelated_class_ty: null_type,
            sff_debug_luau_force_old_solver: ScopedFastFlag::new(
                &FFlag::DebugLuauForceOldSolver,
                false,
            ),
        };

        {
            let frontend = fixture.base.get_frontend();
            create_some_extern_types(frontend);
        }

        let (parent_class_ty, child_class_ty, another_child_class_ty, unrelated_class_ty) = {
            let frontend = fixture.base.get_frontend();
            let globals = &mut frontend.globals;

            (
                try_get_global_binding(globals, "Parent")
                    .expect("Parent global binding")
                    .type_id,
                try_get_global_binding(globals, "Child")
                    .expect("Child global binding")
                    .type_id,
                try_get_global_binding(globals, "AnotherChild")
                    .expect("AnotherChild global binding")
                    .type_id,
                try_get_global_binding(globals, "Unrelated")
                    .expect("Unrelated global binding")
                    .type_id,
            )
        };

        let builtins_ptr = fixture.base.builtin_types;
        let builtins = unsafe { &mut *builtins_ptr };
        let scope = Arc::new(Scope::scope_type_pack_id(builtins.anyTypePack));
        let scope_ptr = Arc::as_ptr(&scope) as *mut Scope;

        let free_ty = fixture
            .arena
            .fresh_type_not_null_builtin_types_scope(builtins, scope_ptr);
        let generic_ty = fixture.arena.add_type(GenericType::default());
        let blocked_ty = fixture.arena.add_type(BlockedType::default());
        let pending_ty = fixture.arena.add_type(PendingExpansionType {
            prefix: None,
            name: AstName::default(),
            type_arguments: Vec::new(),
            pack_arguments: Vec::new(),
            index: PendingExpansionType::fresh_index(),
        });
        let hello_ty = fixture
            .arena
            .add_type(SingletonType::singleton_type(SingletonVariant::V1(
                StringSingleton::new(String::from("hello")),
            )));
        let world_ty = fixture
            .arena
            .add_type(SingletonType::singleton_type(SingletonVariant::V1(
                StringSingleton::new(String::from("world")),
            )));

        let empty_type_pack = fixture.arena.add_type_pack_initializer_list_type_id(&[]);
        let fn1_ty = fixture.arena.add_type(FunctionType::function_type_new(
            empty_type_pack,
            empty_type_pack,
            None,
            false,
        ));
        let fn2_ty = fixture.arena.add_type(FunctionType::function_type_new(
            builtins.anyTypePack,
            empty_type_pack,
            None,
            false,
        ));

        fixture.scope = scope;
        fixture.any_ty = builtins.anyType;
        fixture.unknown_ty = builtins.unknownType;
        fixture.never_ty = builtins.neverType;
        fixture.error_ty = builtins.errorType;
        fixture.function_ty = builtins.functionType;
        fixture.table_ty = builtins.tableType;
        fixture.number_ty = builtins.numberType;
        fixture.string_ty = builtins.stringType;
        fixture.boolean_ty = builtins.booleanType;
        fixture.nil_ty = builtins.nilType;
        fixture.class_ty = builtins.externType;
        fixture.true_ty = builtins.trueType;
        fixture.false_ty = builtins.falseType;
        fixture.truthy_ty = builtins.truthyType;
        fixture.falsy_ty = builtins.falsyType;
        fixture.free_ty = free_ty;
        fixture.generic_ty = generic_ty;
        fixture.blocked_ty = blocked_ty;
        fixture.pending_ty = pending_ty;
        fixture.hello_ty = hello_ty;
        fixture.world_ty = world_ty;
        fixture.empty_type_pack = empty_type_pack;
        fixture.fn1_ty = fn1_ty;
        fixture.fn2_ty = fn2_ty;
        fixture.parent_class_ty = parent_class_ty;
        fixture.child_class_ty = child_class_ty;
        fixture.another_child_class_ty = another_child_class_ty;
        fixture.unrelated_class_ty = unrelated_class_ty;

        fixture
    }

    pub fn simplify_fixture(&mut self) {
        *self = Self::new();
    }
}
