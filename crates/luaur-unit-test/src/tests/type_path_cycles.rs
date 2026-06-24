use crate::records::fixture::Fixture;
use luaur_analysis::enums::type_field::TypeField;
use luaur_analysis::functions::get_mutable_type::get_mutable;
use luaur_analysis::functions::traverse_for_type_type_path::traverse_for_type;
use luaur_analysis::records::blocked_type::BlockedType;
use luaur_analysis::records::path::Path;
use luaur_analysis::records::property_type::Property;
use luaur_analysis::records::property_type_path::Property as PathProperty;
use luaur_analysis::records::r#type::Type;
use luaur_analysis::records::table_type::TableType;
use luaur_analysis::records::type_arena::TypeArena;
use luaur_analysis::type_aliases::component::Component;
use luaur_analysis::type_aliases::type_variant::TypeVariant;
use std::panic::{catch_unwind, AssertUnwindSafe};

#[test]
fn type_path_cycles() {
    let mut fixture = Fixture::default();
    let builtins = fixture.get_builtins() as *mut _;

    let mut arena = TypeArena::default();
    let a = arena.add_type(BlockedType::default());
    let b = arena.add_type(Type {
        ty: TypeVariant::Bound(a),
        persistent: false,
        documentation_symbol: None,
        owning_arena: core::ptr::null_mut(),
    });
    unsafe {
        (*(a as *mut Type)).ty = TypeVariant::Bound(b);
    }

    let panic = catch_unwind(AssertUnwindSafe(|| {
        let _ = traverse_for_type(
            a,
            &Path::from_component(Component::TypeField(TypeField::IndexResult)),
            unsafe { &*builtins },
            &mut arena,
        );
    }));
    assert!(panic.is_err());

    let mut arena = TypeArena::default();
    let tbl = arena.add_type(TableType::table_type());
    unsafe {
        let table = get_mutable::<TableType>(tbl);
        assert!(!table.is_null());
        (*table)
            .props
            .insert(String::from("a"), Property::readonly(tbl));
    }

    let path = Path::from_component(Component::Property(PathProperty::property_string_bool(
        "a", true,
    )));
    assert_eq!(
        traverse_for_type(tbl, &path, unsafe { &*builtins }, &mut arena),
        Some(tbl)
    );
}
