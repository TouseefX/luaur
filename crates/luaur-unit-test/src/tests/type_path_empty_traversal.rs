use crate::records::fixture::Fixture;
use luaur_analysis::functions::traverse_for_type_type_path::traverse_for_type;
use luaur_analysis::records::path::Path;
use luaur_analysis::records::type_arena::TypeArena;

#[test]
fn type_path_empty_traversal() {
    let mut fixture = Fixture::default();
    let number_type = fixture.get_builtins().numberType;
    let builtins = fixture.get_builtins() as *mut _;
    let mut arena = TypeArena::default();

    assert_eq!(
        traverse_for_type(
            number_type,
            &Path::default(),
            unsafe { &*builtins },
            &mut arena
        ),
        Some(number_type)
    );
}
