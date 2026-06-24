use crate::records::team_city_reporter::TeamCityReporter;
use core::ffi::c_void;

pub fn team_city_reporter_report_query(team_city_reporter: &mut TeamCityReporter, _query: &c_void) {
    let _ = team_city_reporter.report_query(_query);
}
