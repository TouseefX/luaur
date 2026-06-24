#[allow(unused_macros)]
macro_rules! WRITE_PAIR_STRING {
    ($fp:expr, $stats:expr, $indent:expr, $name:ident, $format:expr) => {
        libc::fprintf(
            $fp,
            concat!($indent, "\"", stringify!($name), "\": ", $format).as_ptr()
                as *const core::ffi::c_char,
            $stats.$name.as_ptr(),
        )
    };
}

pub(crate) use WRITE_PAIR_STRING;
