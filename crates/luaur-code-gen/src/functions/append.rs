use core::fmt::Write;

pub(crate) fn append(result: &mut alloc::string::String, args: core::fmt::Arguments<'_>) {
    // write! returns fmt::Result; truncation is acceptable (the C++ version
    // also truncated to a 256-byte buffer).
    let _ = result.write_fmt(args);
}
