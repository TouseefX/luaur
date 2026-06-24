use crate::records::subtype_fixture::SubtypeFixture;
use alloc::string::ToString;
use alloc::sync::Arc;
use luaur_analysis::records::generic_type::GenericType;
use luaur_analysis::records::scope::Scope;

#[cfg(test)]
#[test]
fn subtyping_unknown_x() {
    let mut fixture = SubtypeFixture::default();
    let child_scope = Arc::new(Scope::new(&fixture.root_scope, 0));
    let grand_child_scope = Arc::new(Scope::new(&child_scope, 0));

    let generic_x = fixture.arena.add_type(GenericType::generic_type_scope_name(
        Arc::as_ptr(&child_scope) as *mut Scope,
        &"X".to_string(),
    ));
    let unknown_ty = fixture.builtin_types.unknownType;

    let using_global_scope = fixture.is_subtype_type_id_type_id(unknown_ty, generic_x);
    assert!(!using_global_scope.is_subtype());

    let mut child_subtyping = fixture.mk_subtyping();
    let using_child_scope = child_subtyping.is_subtype_type_id_type_id_not_null_scope(
        unknown_ty,
        generic_x,
        Arc::as_ptr(&child_scope) as *mut Scope,
    );
    assert!(using_child_scope.is_subtype());

    let mut grand_child_subtyping = fixture.mk_subtyping();
    let using_grand_child_scope = grand_child_subtyping.is_subtype_type_id_type_id_not_null_scope(
        unknown_ty,
        generic_x,
        Arc::as_ptr(&grand_child_scope) as *mut Scope,
    );
    assert!(using_grand_child_scope.is_subtype());
}
