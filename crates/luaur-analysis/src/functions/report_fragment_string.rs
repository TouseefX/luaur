use crate::records::i_fragment_autocomplete_reporter::IFragmentAutocompleteReporter;

#[allow(non_snake_case)]
pub(crate) fn report_fragment_string(
    reporter: *mut dyn IFragmentAutocompleteReporter,
    fragment: &str,
) {
    if reporter.is_null() {
        return;
    }

    unsafe {
        (*reporter).report_fragment_string(fragment);
    }
}
