//! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_intersection_types_overload_is_not_a_function() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
--!nonstrict
function _(...):((typeof(not _))&(typeof(not _)))&((typeof(not _))&(typeof(not _)))
_(...)(setfenv,_,not _,"")[_] = nil
end
do end
_(...)(...,setfenv,_):_G()
"#,
        ),
        None,
    );
}
