#[cfg(test)]
#[test]
fn parser_error_on_non_utf_8_sequence() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let expected = alloc::string::String::from(
        "Expected identifier when parsing expression, got invalid UTF-8 sequence",
    );

    // C++ source used raw bytes \xFF and \xE2 which are invalid UTF-8. A lossy
    // conversion would replace those bytes with U+FFFD (valid UTF-8), so the lexer
    // would never see an invalid sequence. The lexer reads the buffer byte-by-byte
    // (via a raw *const c_char), so we build the String from the exact bytes with
    // `from_utf8_unchecked` to keep the raw 0xFF / 0xE2 intact, matching C++.
    let source1 = unsafe {
        alloc::string::String::from_utf8_unchecked(alloc::vec![
            b'l', b'o', b'c', b'a', b'l', b' ', b'p', b'i', b' ', b'=', b' ', 0xFF, b'!',
        ])
    };
    fixture.match_parse_error(&source1, &expected, None);
    let source2 = unsafe {
        alloc::string::String::from_utf8_unchecked(alloc::vec![
            b'l', b'o', b'c', b'a', b'l', b' ', b'p', b'i', b' ', b'=', b' ', 0xE2, b'!',
        ])
    };
    fixture.match_parse_error(&source2, &expected, None);
}
