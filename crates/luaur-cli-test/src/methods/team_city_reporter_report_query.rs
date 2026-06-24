use crate::records::team_city_reporter::TeamCityReporter;

impl TeamCityReporter {
    pub fn report_query(&mut self, _query_data: *const core::ffi::c_void) {
        // This is an empty override of a doctest reporter method.
    }
}

#[allow(non_snake_case)]
pub fn team_city_reporter_report_query(
    this: &mut TeamCityReporter,
    query_data: *const core::ffi::c_void,
) {
    this.report_query(query_data);
}
