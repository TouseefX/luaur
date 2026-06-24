#[cfg(test)]
#[test]
fn parser_read_write_table_properties() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let source = alloc::string::String::from(
        "type A = {read x: number}\n\
         type B = {write x: number}\n\
         type C = {read x: number, write x: number}\n\
         type D = {read: () -> string}\n\
         type E = {write: (string) -> ()}\n\
         type F = {read read: () -> string}\n\
         type G = {read write: (string) -> ()}\n\
         type H = {read [\"A\"]: number}\n\
         type I = {write [\"A\"]: string}\n\
         type J = {read [number]: number}\n\
         type K = {write [number]: string}",
    );
    let result = fixture.try_parse(
        &source,
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );
    assert_eq!(result.errors.len(), 0);
}
