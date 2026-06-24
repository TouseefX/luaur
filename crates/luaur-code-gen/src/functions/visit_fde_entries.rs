#[allow(non_snake_case)]
extern "C" {
    static __unw_add_dynamic_fde: *const core::ffi::c_void;
}

pub unsafe fn visit_fde_entries(
    pos: *mut core::ffi::c_char,
    cb: unsafe extern "C" fn(*const core::ffi::c_void),
) {
    if __unw_add_dynamic_fde.is_null() {
        return cb(pos as *const core::ffi::c_void);
    }

    let mut current_pos = pos;
    loop {
        let mut part_length: u32 = 0;
        core::ptr::copy_nonoverlapping(
            current_pos as *const u8,
            &mut part_length as *mut u32 as *mut u8,
            core::mem::size_of::<u32>(),
        );

        if part_length == 0 {
            break;
        }

        let mut part_id: u32 = 0;
        core::ptr::copy_nonoverlapping(
            current_pos.add(4) as *const u8,
            &mut part_id as *mut u32 as *mut u8,
            core::mem::size_of::<u32>(),
        );

        if part_id != 0 {
            cb(current_pos as *const core::ffi::c_void);
        }

        current_pos = current_pos.add(part_length as usize + 4);
    }
}
