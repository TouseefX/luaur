pub fn get_current_working_directory() -> Option<alloc::string::String> {
    const MAX_PATH_LENGTH: usize = 131072;
    const INITIAL_PATH_LENGTH: usize = 260;

    let mut buffer = alloc::vec![0u8; INITIAL_PATH_LENGTH];
    let mut current_len = INITIAL_PATH_LENGTH;

    loop {
        let result = unsafe {
            #[cfg(windows)]
            {
                use core::ffi::{c_char, c_int};
                extern "C" {
                    fn _getcwd(buffer: *mut c_char, maxlen: c_int) -> *mut c_char;
                }
                _getcwd(buffer.as_mut_ptr() as *mut c_char, buffer.len() as c_int)
            }
            #[cfg(not(windows))]
            {
                use core::ffi::{c_char, c_ulong};
                extern "C" {
                    fn getcwd(buffer: *mut c_char, size: usize) -> *mut c_char;
                }
                getcwd(buffer.as_mut_ptr() as *mut c_char, buffer.len())
            }
        };

        if !result.is_null() {
            let c_str = unsafe { core::ffi::CStr::from_ptr(result) };
            return Some(c_str.to_string_lossy().into_owned());
        }

        let err = std::io::Error::last_os_error();
        if err.raw_os_error() != Some(34) || current_len * 2 > MAX_PATH_LENGTH {
            return None;
        }

        current_len *= 2;
        buffer.resize(current_len, 0);
    }
}
