use crate::records::boost_like_reporter::BoostLikeReporter;

impl BoostLikeReporter {
    pub fn subcase_end(&mut self) {
        // The C++ implementation is empty: void subcase_end() override {}
    }
}

pub fn boost_like_reporter_subcase_end(this: &mut BoostLikeReporter) {
    this.subcase_end();
}
