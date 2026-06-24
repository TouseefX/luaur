use crate::records::boost_like_reporter::BoostLikeReporter;
use core::ffi::c_void;

#[repr(C)]
struct TestCaseData {
    m_test_suite: *const core::ffi::c_char,
    m_name: *const core::ffi::c_char,
}

#[repr(C)]
struct QueryData {
    _reporter: *const c_void,
    num_data: core::ffi::c_uint,
    data: *const *const TestCaseData,
}

impl BoostLikeReporter {
    pub fn report_query(&mut self, qd: *const c_void) {
        let qd = unsafe { &*(qd as *const QueryData) };

        for i in 0..qd.num_data {
            let tc = unsafe { &**qd.data.add(i as usize) };
            let test_suite =
                unsafe { core::ffi::CStr::from_ptr(tc.m_test_suite).to_string_lossy() };
            let name = unsafe { core::ffi::CStr::from_ptr(tc.m_name).to_string_lossy() };
            eprintln!("{}/{}", test_suite, name);
        }

        eprintln!("Found {} tests.", qd.num_data);
    }
}
