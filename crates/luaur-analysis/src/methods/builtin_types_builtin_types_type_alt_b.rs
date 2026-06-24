//! C++ `BuiltinTypes::BuiltinTypes()` (Type.cpp:843-880). Allocates the builtin
//! arena, mints every persistent builtin type/singleton/pack, then freezes the
//! arena. `typeFunctions` is `make_unique<BuiltinTypeFunctions>()`.
use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec;

use crate::enums::table_state::TableState;
use crate::functions::freeze::freeze;
use crate::records::any_type::AnyType;
use crate::records::boolean_singleton::BooleanSingleton;
use crate::records::builtin_type_functions::BuiltinTypeFunctions;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::extern_type::ExternType;
use crate::records::negation_type::NegationType;
use crate::records::never_type::NeverType;
use crate::records::no_refine_type::NoRefineType;
use crate::records::primitive_type::PrimitiveType;
use crate::records::r#type::Type;
use crate::records::singleton_type::SingletonType;
use crate::records::table_type::TableType;
use crate::records::type_arena::TypeArena;
use crate::records::type_level::TypeLevel;
use crate::records::type_pack::TypePack;
use crate::records::type_pack_var::TypePackVar;
use crate::records::unifiable::Error;
use crate::records::union_type::UnionType;
use crate::records::unknown_type::UnknownType;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::singleton_variant::SingletonVariant;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::type_pack_variant::TypePackVariant;
use crate::type_aliases::type_variant::TypeVariant;
use luaur_common::FFlag;

impl BuiltinTypes {
    /// C++ `BuiltinTypes::BuiltinTypes()`.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self::builtin_types()
    }

    /// C++ `BuiltinTypes::BuiltinTypes()` — the real default ctor.
    #[allow(non_snake_case)]
    pub fn builtin_types() -> Self {
        let mut arena: Box<TypeArena> = Box::new(TypeArena::default());
        let debug_freeze_arena = FFlag::DebugLuauFreezeArena.get();

        // Persistent type/pack minting helpers (mirror `arena->addType(Type{v,
        // /*persistent*/ true})` / `addTypePack(TypePackVar{v, true})`).
        fn add_persistent_type(arena: &mut TypeArena, v: TypeVariant) -> TypeId {
            arena.add_tv(Type::new_with_persistence(v, true))
        }
        fn add_persistent_pack(arena: &mut TypeArena, v: TypePackVariant) -> TypePackId {
            arena.add_type_pack_type_pack_var(TypePackVar {
                ty: v,
                persistent: true,
                owningArena: core::ptr::null_mut(),
            })
        }

        let arena_ref = &mut *arena;

        let nil_type = add_persistent_type(
            arena_ref,
            TypeVariant::Primitive(PrimitiveType {
                r#type: PrimitiveType::NilType,
                metatable: None,
            }),
        );
        let number_type = add_persistent_type(
            arena_ref,
            TypeVariant::Primitive(PrimitiveType {
                r#type: PrimitiveType::Number,
                metatable: None,
            }),
        );
        let integer_type = add_persistent_type(
            arena_ref,
            TypeVariant::Primitive(PrimitiveType {
                r#type: PrimitiveType::Integer,
                metatable: None,
            }),
        );
        let string_type = add_persistent_type(
            arena_ref,
            TypeVariant::Primitive(PrimitiveType {
                r#type: PrimitiveType::String,
                metatable: None,
            }),
        );
        let boolean_type = add_persistent_type(
            arena_ref,
            TypeVariant::Primitive(PrimitiveType {
                r#type: PrimitiveType::Boolean,
                metatable: None,
            }),
        );
        let thread_type = add_persistent_type(
            arena_ref,
            TypeVariant::Primitive(PrimitiveType {
                r#type: PrimitiveType::Thread,
                metatable: None,
            }),
        );
        let buffer_type = add_persistent_type(
            arena_ref,
            TypeVariant::Primitive(PrimitiveType {
                r#type: PrimitiveType::Buffer,
                metatable: None,
            }),
        );
        let function_type = add_persistent_type(
            arena_ref,
            TypeVariant::Primitive(PrimitiveType {
                r#type: PrimitiveType::Function,
                metatable: None,
            }),
        );

        // ExternType{"userdata"|"object"|"class", {}, nullopt, nullopt, {}, {}, {}, {}}
        let make_extern = |name: &str| ExternType {
            name: name.to_string(),
            props: Default::default(),
            parent: None,
            metatable: None,
            tags: Default::default(),
            user_data: None,
            definition_module_name: Default::default(),
            definition_location: None,
            indexer: None,
            relation: None,
        };
        let extern_type =
            add_persistent_type(arena_ref, TypeVariant::Extern(make_extern("userdata")));
        let object_type =
            add_persistent_type(arena_ref, TypeVariant::Extern(make_extern("object")));
        let class_type = add_persistent_type(arena_ref, TypeVariant::Extern(make_extern("class")));

        let table_type_id = add_persistent_type(
            arena_ref,
            TypeVariant::Primitive(PrimitiveType {
                r#type: PrimitiveType::Table,
                metatable: None,
            }),
        );

        // TableType{TableState::Sealed, TypeLevel{}, nullptr}
        let empty_table_type = add_persistent_type(
            arena_ref,
            TypeVariant::Table(TableType::table_type_table_state_type_level_scope(
                TableState::Sealed,
                TypeLevel::default(),
                core::ptr::null_mut(),
            )),
        );

        let true_type = add_persistent_type(
            arena_ref,
            TypeVariant::Singleton(SingletonType::singleton_type(SingletonVariant::V0(
                BooleanSingleton::new(true),
            ))),
        );
        let false_type = add_persistent_type(
            arena_ref,
            TypeVariant::Singleton(SingletonType::singleton_type(SingletonVariant::V0(
                BooleanSingleton::new(false),
            ))),
        );

        let any_type = add_persistent_type(arena_ref, TypeVariant::Any(AnyType::default()));
        let unknown_type =
            add_persistent_type(arena_ref, TypeVariant::Unknown(UnknownType::default()));
        let never_type = add_persistent_type(arena_ref, TypeVariant::Never(NeverType::default()));
        let error_type = add_persistent_type(arena_ref, TypeVariant::Error(Error::new()));
        let no_refine_type =
            add_persistent_type(arena_ref, TypeVariant::NoRefine(NoRefineType::default()));

        let falsy_type = add_persistent_type(
            arena_ref,
            TypeVariant::Union(UnionType {
                options: vec![false_type, nil_type],
            }),
        );
        let truthy_type = add_persistent_type(
            arena_ref,
            TypeVariant::Negation(NegationType { ty: falsy_type }),
        );
        let not_nil_type = add_persistent_type(
            arena_ref,
            TypeVariant::Negation(NegationType { ty: nil_type }),
        );
        let optional_number_type = add_persistent_type(
            arena_ref,
            TypeVariant::Union(UnionType {
                options: vec![number_type, nil_type],
            }),
        );
        let optional_string_type = add_persistent_type(
            arena_ref,
            TypeVariant::Union(UnionType {
                options: vec![string_type, nil_type],
            }),
        );

        let empty_type_pack = add_persistent_pack(
            arena_ref,
            TypePackVariant::TypePack(TypePack {
                head: vec![],
                tail: None,
            }),
        );
        let any_type_pack = add_persistent_pack(
            arena_ref,
            TypePackVariant::Variadic(VariadicTypePack {
                ty: any_type,
                hidden: false,
            }),
        );
        let unknown_type_pack = add_persistent_pack(
            arena_ref,
            TypePackVariant::Variadic(VariadicTypePack {
                ty: unknown_type,
                hidden: false,
            }),
        );
        let never_type_pack = add_persistent_pack(
            arena_ref,
            TypePackVariant::Variadic(VariadicTypePack {
                ty: never_type,
                hidden: false,
            }),
        );
        let uninhabitable_type_pack = add_persistent_pack(
            arena_ref,
            TypePackVariant::TypePack(TypePack {
                head: vec![never_type],
                tail: Some(never_type_pack),
            }),
        );
        let error_type_pack = add_persistent_pack(arena_ref, TypePackVariant::Error(Error::new()));

        freeze(&mut arena);

        BuiltinTypes {
            arena,
            debugFreezeArena: debug_freeze_arena,
            typeFunctions: Box::new(BuiltinTypeFunctions::new()),
            nilType: nil_type,
            numberType: number_type,
            integerType: integer_type,
            stringType: string_type,
            booleanType: boolean_type,
            threadType: thread_type,
            bufferType: buffer_type,
            functionType: function_type,
            externType: extern_type,
            objectType: object_type,
            classType: class_type,
            tableType: table_type_id,
            emptyTableType: empty_table_type,
            trueType: true_type,
            falseType: false_type,
            anyType: any_type,
            unknownType: unknown_type,
            neverType: never_type,
            errorType: error_type,
            noRefineType: no_refine_type,
            falsyType: falsy_type,
            truthyType: truthy_type,
            notNilType: not_nil_type,
            optionalNumberType: optional_number_type,
            optionalStringType: optional_string_type,
            emptyTypePack: empty_type_pack,
            anyTypePack: any_type_pack,
            unknownTypePack: unknown_type_pack,
            neverTypePack: never_type_pack,
            uninhabitableTypePack: uninhabitable_type_pack,
            errorTypePack: error_type_pack,
        }
    }
}
