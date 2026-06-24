use crate::functions::compare_member_name::compare_member_name;

const LBC_TYPE_VECTOR: i32 = 8;
const LBC_TYPE_ANY: i32 = 15;

pub fn luau_library_type_lookup(
    library: *const core::ffi::c_char,
    member: *const core::ffi::c_char,
) -> i32 {
    if compare_member_name(
        library,
        unsafe { core::ffi::CStr::from_ptr(library) }
            .to_bytes()
            .len(),
        c"Vector3".as_ptr(),
    ) {
        if compare_member_name(
            member,
            unsafe { core::ffi::CStr::from_ptr(member) }
                .to_bytes()
                .len(),
            c"xAxis".as_ptr(),
        ) {
            return LBC_TYPE_VECTOR;
        }

        if compare_member_name(
            member,
            unsafe { core::ffi::CStr::from_ptr(member) }
                .to_bytes()
                .len(),
            c"yAxis".as_ptr(),
        ) {
            return LBC_TYPE_VECTOR;
        }
    }

    LBC_TYPE_ANY
}
