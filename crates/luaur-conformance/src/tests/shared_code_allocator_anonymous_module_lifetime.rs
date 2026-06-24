//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/SharedCodeAllocator.test.cpp:363:shared_code_allocator_anonymous_module_lifetime`
//! Source: `tests/SharedCodeAllocator.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/SharedCodeAllocator.test.cpp
//! - source_includes:
//!   - includes -> source_file CodeGen/include/Luau/SharedCodeAllocator.h
//!   - includes -> source_file CodeGen/include/Luau/CodeAllocator.h
//!   - includes -> source_file Compiler/include/luacode.h
//!   - includes -> source_file CodeGen/include/luacodegen.h
//!   - includes -> source_file VM/include/lualib.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/SharedCodeAllocator.test.cpp
//! - outgoing:
//!   - calls -> function luau_codegen_supported (CodeGen/src/lcodegen.cpp)
//!   - type_ref -> record CodeAllocator (CodeGen/include/Luau/CodeAllocator.h)
//!   - type_ref -> record SharedCodeAllocator (CodeGen/include/Luau/SharedCodeAllocator.h)
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - type_ref -> type_alias NativeProtoExecDataPtr (CodeGen/include/Luau/NativeProtoExecData.h)
//!   - type_ref -> record NativeModuleRef (CodeGen/include/Luau/SharedCodeAllocator.h)
//!   - calls -> method SharedCodeAllocator::insertAnonymousNativeModule (CodeGen/src/SharedCodeAllocator.cpp)
//!   - calls -> method NativeModule::getModuleBaseAddress (CodeGen/src/SharedCodeAllocator.cpp)
//!   - calls -> method NativeModule::tryGetNativeProto (CodeGen/src/SharedCodeAllocator.cpp)
//!   - calls -> method NativeModule::getRefcount (CodeGen/src/SharedCodeAllocator.cpp)
//!   - type_ref -> record NativeModule (CodeGen/include/Luau/SharedCodeAllocator.h)
//!   - calls -> method NativeModule::addRef (CodeGen/src/SharedCodeAllocator.cpp)
//!   - calls -> method NativeModuleRef::reset (CodeGen/src/SharedCodeAllocator.cpp)
//!   - translates_to -> rust_item shared_code_allocator_anonymous_module_lifetime

#[cfg(test)]
#[test]
fn shared_code_allocator_anonymous_module_lifetime() {
    use alloc::vec;
    use alloc::vec::Vec;
    use luaur_code_gen::functions::create_native_proto_exec_data_native_proto_exec_data::create_native_proto_exec_data_u32_u32;
    use luaur_code_gen::functions::get_native_proto_exec_data_header_native_proto_exec_data::get_native_proto_exec_data_header_mut;
    use luaur_code_gen::functions::luau_codegen_supported::luau_codegen_supported;
    use luaur_code_gen::records::code_allocator::CodeAllocator;
    use luaur_code_gen::records::shared_code_allocator::SharedCodeAllocator;

    if luau_codegen_supported() == 0 {
        return;
    }

    const K_BLOCK_SIZE: usize = 1024 * 1024;
    const K_MAX_TOTAL_SIZE: usize = 1024 * 1024;

    let mut code_allocator = CodeAllocator::default();
    code_allocator.code_allocator_usize_usize(K_BLOCK_SIZE, K_MAX_TOTAL_SIZE);

    let mut allocator = SharedCodeAllocator::default();
    allocator.shared_code_allocator_code_allocator(&mut code_allocator as *mut _);

    let data = vec![0u8; 8];
    let code = vec![0u8; 8];

    let mut native_protos = Vec::with_capacity(1);

    {
        let native_proto = create_native_proto_exec_data_u32_u32(2, 0);
        unsafe {
            let header = get_native_proto_exec_data_header_mut(native_proto.as_ptr());
            (*header).bytecode_id = 1;
            (*header).entry_offset_or_address = 0usize as *const u8;
            *native_proto.as_ptr().add(0) = 0;
            *native_proto.as_ptr().add(1) = 4;
        }
        native_protos.push(native_proto);
    }

    let mut mod_ref = allocator.insert_anonymous_native_module(
        native_protos,
        data.as_ptr(),
        data.len(),
        code.as_ptr(),
        code.len(),
    );
    assert!(!mod_ref.native_module_ref_empty());

    let module = mod_ref.native_module_ref_get();
    unsafe {
        assert!(!(*module).native_module_get_module_base_address().is_null());
        assert!(!(*module).native_module_try_get_native_proto(1).is_null());
        assert_eq!(1, (*module).native_module_get_refcount());
    }

    unsafe {
        (*module).native_module_add_ref();
        assert_eq!(2, (*module).native_module_get_refcount());
    }

    mod_ref.native_module_ref_reset();
    unsafe {
        assert_eq!(1, (*module).native_module_get_refcount());
        (*module).release();
    }
}
