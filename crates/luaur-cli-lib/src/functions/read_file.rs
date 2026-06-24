use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::c_char;

pub fn read_file(name: &str) -> Option<String> {
    #[cfg(windows)]
    {
        let wide_path = crate::functions::from_utf_8::from_utf_8(name);
        let file = unsafe {
            extern "C" {
                fn _wfopen(wfilename: *const u16, mode: *const u16) -> *mut c_char;
            }
            _wfopen(wide_path.as_ptr(), 0x0000 as *const u16) // L"rb" encoded as UTF-16
        };
        if file.is_null() {
            return None;
        }
        process_file(file)
    }
    #[cfg(not(windows))]
    {
        extern "C" {
            fn fopen(filename: *const c_char, mode: *const c_char) -> *mut c_char;
        }
        let mode = b"rb\0" as *const u8 as *const c_char;
        // `fopen` expects a NUL-terminated C string; the incoming `&str` is not
        // guaranteed to have a trailing NUL in its backing buffer.
        let mut name_c: Vec<u8> = Vec::with_capacity(name.len() + 1);
        name_c.extend_from_slice(name.as_bytes());
        name_c.push(0);
        let name_ptr = name_c.as_ptr() as *const c_char;
        let file = unsafe { fopen(name_ptr, mode) };
        if file.is_null() {
            return None;
        }
        process_file(file)
    }
}

fn process_file(file: *mut c_char) -> Option<String> {
    unsafe {
        extern "C" {
            fn fseek(stream: *mut c_char, offset: i64, whence: i32) -> i32;
            fn ftell(stream: *mut c_char) -> i64;
            fn fread(ptr: *mut c_char, size: usize, nmemb: usize, stream: *mut c_char) -> usize;
            fn fclose(stream: *mut c_char) -> i32;
        }

        const SEEK_SET: i32 = 0;
        const SEEK_END: i32 = 2;

        if fseek(file, 0, SEEK_END) != 0 {
            fclose(file);
            return None;
        }
        let length = ftell(file);
        if length < 0 {
            fclose(file);
            return None;
        }
        if fseek(file, 0, SEEK_SET) != 0 {
            fclose(file);
            return None;
        }

        let len = length as usize;
        let mut buf: Vec<u8> = Vec::with_capacity(len);

        let read = fread(buf.as_mut_ptr() as *mut c_char, 1, len, file);
        fclose(file);

        if read != len {
            return None;
        }

        buf.set_len(len);

        let mut result = String::from_utf8_unchecked(buf);

        // Skip first line if it's a shebang
        if result.len() > 2 && result.as_bytes()[0] == b'#' && result.as_bytes()[1] == b'!' {
            if let Some(newline_pos) = result.find('\n') {
                result.drain(0..=newline_pos);
            }
        }

        Some(result)
    }
}
