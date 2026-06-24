use crate::records::team_city_reporter::TeamCityReporter;

impl TeamCityReporter {
    pub fn test_run_start(&mut self) {
        // C++ implementation is empty: void test_run_start() override {}
    }
}

#[allow(non_snake_case)]
pub fn team_city_reporter_test_run_start(this: &mut TeamCityReporter) {
    this.test_run_start();
}
