use crate::records::boost_like_reporter::BoostLikeReporter;

impl BoostLikeReporter {
    pub fn test_case_reenter(&mut self, _test_case_data: *const core::ffi::c_void) {
        // This is an empty override of a doctest reporter method.
    }
}

#[allow(non_snake_case)]
pub fn boost_like_reporter_test_case_reenter(
    this: &mut BoostLikeReporter,
    test_case_data: *const core::ffi::c_void,
) {
    this.test_case_reenter(test_case_data);
}
