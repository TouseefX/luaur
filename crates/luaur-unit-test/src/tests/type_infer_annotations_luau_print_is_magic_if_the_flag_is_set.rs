//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_luau_print_is_magic_if_the_flag_is_set() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use core::sync::atomic::{AtomicUsize, Ordering};
    use luaur_analysis::functions::reset_print_line::reset_print_line;
    use luaur_analysis::functions::set_print_line::setPrintLine;
    use luaur_common::FFlag;

    static OUTPUT_COUNT: AtomicUsize = AtomicUsize::new(0);

    extern "C" fn capture_print_line(_line: &String) {
        OUTPUT_COUNT.fetch_add(1, Ordering::SeqCst);
    }

    struct ResetPrintLineGuard;

    impl Drop for ResetPrintLineGuard {
        fn drop(&mut self) {
            reset_print_line();
        }
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    OUTPUT_COUNT.store(0, Ordering::SeqCst);
    setPrintLine(Some(capture_print_line));
    let _guard = ResetPrintLineGuard;

    let _sffs = ScopedFastFlag::new(&FFlag::DebugLuauMagicTypes, true);
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local a: _luau_print<typeof(math.abs)>
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(1, OUTPUT_COUNT.load(Ordering::SeqCst));
}
