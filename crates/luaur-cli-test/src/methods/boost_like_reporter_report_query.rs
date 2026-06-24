use crate::records::boost_like_reporter::BoostLikeReporter;
use core::ffi::CStr;

#[allow(non_camel_case_types)]
pub struct doctest_QueryData {
    pub num_data: core::ffi::c_uint,
    pub data: *const *const doctest_TestCaseData,
}

#[allow(non_camel_case_types)]
pub struct doctest_TestCaseData {
    pub m_test_suite: *const core::ffi::c_char,
    pub m_name: *const core::ffi::c_char,
}

pub fn boost_like_reporter_report_query(_reporter: &BoostLikeReporter, qd: &doctest_QueryData) {
    for i in 0..qd.num_data as usize {
        let tc = unsafe { &**qd.data.add(i) };

        let suite = unsafe { CStr::from_ptr(tc.m_test_suite).to_string_lossy() };
        let name = unsafe { CStr::from_ptr(tc.m_name).to_string_lossy() };

        eprintln!("{}/{}", suite, name);
    }

    eprintln!("Found {} tests.", qd.num_data);
}
