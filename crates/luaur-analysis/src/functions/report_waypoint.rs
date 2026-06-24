use crate::enums::fragment_autocomplete_waypoint::FragmentAutocompleteWaypoint;
use crate::records::i_fragment_autocomplete_reporter::IFragmentAutocompleteReporter;

#[allow(non_snake_case)]
pub(crate) fn report_waypoint(
    reporter: *mut dyn IFragmentAutocompleteReporter,
    r#type: FragmentAutocompleteWaypoint,
) {
    if reporter.is_null() {
        return;
    }

    unsafe {
        (*reporter).report_waypoint(r#type);
    }
}
