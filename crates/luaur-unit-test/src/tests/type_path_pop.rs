use luaur_analysis::records::path::Path;

#[test]
fn type_path_pop() {
    let p = Path::default();
    assert!(p.path_empty());
    assert!(p.pop().path_empty());
}
