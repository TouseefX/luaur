use luaur_compiler::functions::luau_set_compile_constant_boolean::luau_set_compile_constant_boolean;
use luaur_compiler::functions::luau_set_compile_constant_nil::luau_set_compile_constant_nil;
use luaur_compiler::functions::luau_set_compile_constant_number::luau_set_compile_constant_number;
use luaur_compiler::functions::luau_set_compile_constant_string::luau_set_compile_constant_string;
use luaur_compiler::functions::luau_set_compile_constant_vector::luau_set_compile_constant_vector;
use luaur_compiler::functions::set_compile_constant_vector::set_compile_constant_vector;
use luaur_compiler::type_aliases::compile_constant::CompileConstant;

pub fn luau_library_constant_lookup(
    library: *const core::ffi::c_char,
    member: *const core::ffi::c_char,
    constant: *mut CompileConstant,
) {
    if unsafe { core::ffi::CStr::from_ptr(library) }.to_str().ok() == Some("vector") {
        if unsafe { core::ffi::CStr::from_ptr(member) }.to_str().ok() == Some("zero") {
            set_compile_constant_vector(
                constant as *mut core::ffi::c_void,
                0.0f32,
                0.0f32,
                0.0f32,
                0.0f32,
            );
            return;
        }

        if unsafe { core::ffi::CStr::from_ptr(member) }.to_str().ok() == Some("one") {
            set_compile_constant_vector(
                constant as *mut core::ffi::c_void,
                1.0f32,
                1.0f32,
                1.0f32,
                0.0f32,
            );
            return;
        }
    }

    if unsafe { core::ffi::CStr::from_ptr(library) }.to_str().ok() == Some("Vector3") {
        if unsafe { core::ffi::CStr::from_ptr(member) }.to_str().ok() == Some("xAxis") {
            set_compile_constant_vector(
                constant as *mut core::ffi::c_void,
                1.0f32,
                0.0f32,
                0.0f32,
                0.0f32,
            );
            return;
        }

        if unsafe { core::ffi::CStr::from_ptr(member) }.to_str().ok() == Some("yAxis") {
            set_compile_constant_vector(
                constant as *mut core::ffi::c_void,
                0.0f32,
                1.0f32,
                0.0f32,
                0.0f32,
            );
            return;
        }
    }

    if unsafe { core::ffi::CStr::from_ptr(library) }.to_str().ok() == Some("test") {
        if unsafe { core::ffi::CStr::from_ptr(member) }.to_str().ok() == Some("some_nil") {
            luau_set_compile_constant_nil(constant as *mut core::ffi::c_void);
            return;
        }

        if unsafe { core::ffi::CStr::from_ptr(member) }.to_str().ok() == Some("some_boolean") {
            luau_set_compile_constant_boolean(constant as *mut core::ffi::c_void, true);
            return;
        }

        if unsafe { core::ffi::CStr::from_ptr(member) }.to_str().ok() == Some("some_number") {
            unsafe {
                luau_set_compile_constant_number(constant as *mut *mut core::ffi::c_void, 4.75f64)
            };
            return;
        }

        if unsafe { core::ffi::CStr::from_ptr(member) }.to_str().ok() == Some("some_vector") {
            luau_set_compile_constant_vector(
                constant as *mut core::ffi::c_void,
                1.0f32,
                2.0f32,
                4.0f32,
                8.0f32,
            );
            return;
        }

        if unsafe { core::ffi::CStr::from_ptr(member) }.to_str().ok() == Some("some_string") {
            let s = c"test".as_ptr();
            let l = 4usize;
            luau_set_compile_constant_string(constant as *mut core::ffi::c_void, s, l);
            return;
        }
    }
}
