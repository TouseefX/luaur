use crate::records::boost_like_reporter::BoostLikeReporter;

impl BoostLikeReporter {
    pub fn test_run_start(&mut self) {
        // C++ implementation is empty: void test_run_start() override {}
    }
}

#[allow(non_snake_case)]
pub fn boost_like_reporter_test_run_start(this: &mut BoostLikeReporter) {
    this.test_run_start();
}
