use core::ffi;
use std::io::Write;

pub fn team_city_reporter_test_case_end(
    self_: &mut crate::records::team_city_reporter::TeamCityReporter,
    in_data: *const ffi::c_void,
) {
    // SAFETY: in_data is a pointer to doctest::CurrentTestCaseStats, which is laid out as a C struct
    // We assume the layout matches the C++ definition for numAssertsCurrentTest, numAssertsFailedCurrentTest, seconds, and testCaseSuccess
    let stats = unsafe { &*(in_data as *const TeamCityReporterStats) };

    let current_test = unsafe { &*((*self_).current_test as *const TeamCityCurrentTest) };

    let suite = unsafe { ffi::CStr::from_ptr(current_test.m_test_suite).to_string_lossy() };
    let name = unsafe { ffi::CStr::from_ptr(current_test.m_name).to_string_lossy() };

    println!(
        "##teamcity[testMetadata testName='{}: {}' name='total_asserts' type='number' value='{}']",
        suite, name, stats.num_asserts_current_test
    );
    println!(
        "##teamcity[testMetadata testName='{}: {}' name='failed_asserts' type='number' value='{}']",
        suite, name, stats.num_asserts_failed_current_test
    );
    println!(
        "##teamcity[testMetadata testName='{}: {}' name='runtime' type='number' value='{}']",
        suite, name, stats.seconds
    );

    if !stats.test_case_success {
        println!("##teamcity[testFailed name='{}: {}']", suite, name);
    }

    println!("##teamcity[testFinished name='{}: {}']", suite, name);
    let _ = std::io::stdout().flush();
}

#[repr(C)]
struct TeamCityReporterStats {
    num_asserts_current_test: i32,
    num_asserts_failed_current_test: i32,
    seconds: f64,
    test_case_success: bool,
}

#[repr(C)]
struct TeamCityCurrentTest {
    m_test_suite: *const ffi::c_char,
    m_name: *const ffi::c_char,
}
