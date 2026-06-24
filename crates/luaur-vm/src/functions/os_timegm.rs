#[allow(non_camel_case_types)]
pub type time_t = i64;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct tm {
    pub tm_sec: core::ffi::c_int,
    pub tm_min: core::ffi::c_int,
    pub tm_hour: core::ffi::c_int,
    pub tm_mday: core::ffi::c_int,
    pub tm_mon: core::ffi::c_int,
    pub tm_year: core::ffi::c_int,
    pub tm_wday: core::ffi::c_int,
    pub tm_yday: core::ffi::c_int,
    pub tm_isdst: core::ffi::c_int,
}

pub fn os_timegm(timep: *const tm) -> time_t {
    let timep = unsafe { &*timep };

    let day = timep.tm_mday;
    let month = timep.tm_mon + 1;
    let year = timep.tm_year + 1900;

    let a = if timep.tm_mon % 12 < 2 { 1 } else { 0 };
    let a = a - (timep.tm_mon / 12);

    let y = year + 4800 - a;
    let m = month + (12 * a) - 3;

    let julianday = day + ((153 * m + 2) / 5) + (365 * y) + (y / 4) - (y / 100) + (y / 400) - 32045;

    let utcstartasjulianday = 2440588;
    let utcstartasjuliansecond: i64 = (utcstartasjulianday as i64) * 86400;

    if (julianday as i64) < (utcstartasjulianday as i64) {
        return -1;
    }

    let daysecond =
        (timep.tm_hour as i64) * 3600 + (timep.tm_min as i64) * 60 + (timep.tm_sec as i64);
    let julianseconds = (julianday as i64) * 86400 + daysecond;

    if julianseconds < utcstartasjuliansecond {
        return -1;
    }

    julianseconds - utcstartasjuliansecond
}
