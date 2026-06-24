//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/SharedCodeAllocator.test.cpp:253:shared_code_allocator_native_proto_refcounting`
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
//!   - type_ref -> type_alias NativeProtoExecDataPtr (CodeGen/include/Luau/NativeProtoExecData.h)
//!   - type_ref -> record NativeModuleRef (CodeGen/include/Luau/SharedCodeAllocator.h)
//!   - calls -> method SharedCodeAllocator::getOrInsertNativeModule (CodeGen/src/SharedCodeAllocator.cpp)
//!   - type_ref -> type_alias ModuleId (CodeGen/include/Luau/CodeGen.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - calls -> method NativeModule::getRefcount (CodeGen/src/SharedCodeAllocator.cpp)
//!   - calls -> method NativeModule::addRef (CodeGen/src/SharedCodeAllocator.cpp)
//!   - calls -> method NativeModule::addRefs (CodeGen/src/SharedCodeAllocator.cpp)
//!   - calls -> method NativeModuleRef::reset (CodeGen/src/SharedCodeAllocator.cpp)
//!   - calls -> method SharedCodeAllocator::tryGetNativeModule (CodeGen/src/SharedCodeAllocator.cpp)
//!   - calls -> method Path::last (Analysis/src/TypePath.cpp)
//!   - type_ref -> record NativeModule (CodeGen/include/Luau/SharedCodeAllocator.h)
//!   - translates_to -> rust_item shared_code_allocator_native_proto_refcounting

#[cfg(test)]
#[test]
fn shared_code_allocator_native_proto_refcounting() {
    use crate::functions::shared_code_allocator_module_id::shared_code_allocator_module_id as module_id;
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
    const FAKE_CODE: [u8; 1] = [0x00];

    let mut code_allocator = CodeAllocator::default();
    code_allocator.code_allocator_usize_usize(K_BLOCK_SIZE, K_MAX_TOTAL_SIZE);

    let mut allocator = SharedCodeAllocator::default();
    allocator.shared_code_allocator_code_allocator(&mut code_allocator as *mut _);

    let mut native_protos = Vec::with_capacity(1);
    let native_proto = create_native_proto_exec_data_u32_u32(0, 0);
    unsafe {
        (*get_native_proto_exec_data_header_mut(native_proto.as_ptr())).bytecode_id = 0x01;
    }
    native_protos.push(native_proto);

    let mut mod_ref_a = allocator
        .get_or_insert_native_module(
            &module_id(0x0a),
            native_protos,
            core::ptr::null(),
            0,
            FAKE_CODE.as_ptr(),
            FAKE_CODE.len(),
        )
        .0;
    assert!(!mod_ref_a.native_module_ref_empty());
    assert_eq!(1, unsafe {
        (*mod_ref_a.native_module_ref_get()).native_module_get_refcount()
    });

    unsafe {
        (*mod_ref_a.native_module_ref_get()).native_module_add_ref();
    }
    assert_eq!(2, unsafe {
        (*mod_ref_a.native_module_ref_get()).native_module_get_refcount()
    });

    unsafe {
        (*mod_ref_a.native_module_ref_get()).native_module_add_refs(2);
    }
    assert_eq!(4, unsafe {
        (*mod_ref_a.native_module_ref_get()).native_module_get_refcount()
    });

    unsafe {
        (*mod_ref_a.native_module_ref_get()).release();
    }
    assert_eq!(3, unsafe {
        (*mod_ref_a.native_module_ref_get()).native_module_get_refcount()
    });

    unsafe {
        (*mod_ref_a.native_module_ref_get()).release();
    }
    assert_eq!(2, unsafe {
        (*mod_ref_a.native_module_ref_get()).native_module_get_refcount()
    });

    mod_ref_a.native_module_ref_reset();

    mod_ref_a = allocator.try_get_native_module(&module_id(0x0a));
    assert!(!mod_ref_a.native_module_ref_empty());
    assert_eq!(2, unsafe {
        (*mod_ref_a.native_module_ref_get()).native_module_get_refcount()
    });

    let raw_mod_a = mod_ref_a.native_module_ref_get();

    mod_ref_a.native_module_ref_reset();
    unsafe {
        (*raw_mod_a).release();
    }
    assert!(allocator
        .try_get_native_module(&module_id(0x0a))
        .native_module_ref_empty());
}
