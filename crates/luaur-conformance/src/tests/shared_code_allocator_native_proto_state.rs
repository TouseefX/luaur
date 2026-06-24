//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/SharedCodeAllocator.test.cpp:304:shared_code_allocator_native_proto_state`
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
//!   - calls -> method SharedCodeAllocator::getOrInsertNativeModule (CodeGen/src/SharedCodeAllocator.cpp)
//!   - type_ref -> type_alias ModuleId (CodeGen/include/Luau/CodeGen.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - calls -> method NativeModule::getModuleBaseAddress (CodeGen/src/SharedCodeAllocator.cpp)
//!   - calls -> method NativeModule::tryGetNativeProto (CodeGen/src/SharedCodeAllocator.cpp)
//!   - translates_to -> rust_item shared_code_allocator_native_proto_state

#[cfg(test)]
#[test]
fn shared_code_allocator_native_proto_state() {
    use crate::functions::shared_code_allocator_module_id::shared_code_allocator_module_id as module_id;
    use alloc::vec;
    use alloc::vec::Vec;
    use luaur_code_gen::functions::create_native_proto_exec_data_native_proto_exec_data::create_native_proto_exec_data_u32_u32;
    use luaur_code_gen::functions::get_native_proto_exec_data_header_native_proto_exec_data::get_native_proto_exec_data_header_mut;
    use luaur_code_gen::functions::get_native_proto_exec_data_header_native_proto_exec_data_alt_b::get_native_proto_exec_data_header;
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

    let data = vec![0u8; 16];
    let code = vec![0u8; 16];

    let mut native_protos = Vec::with_capacity(2);

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

    {
        let native_proto = create_native_proto_exec_data_u32_u32(2, 0);
        unsafe {
            let header = get_native_proto_exec_data_header_mut(native_proto.as_ptr());
            (*header).bytecode_id = 3;
            (*header).entry_offset_or_address = 0x08usize as *const u8;
            *native_proto.as_ptr().add(0) = 8;
            *native_proto.as_ptr().add(1) = 12;
        }
        native_protos.push(native_proto);
    }

    let mod_ref_a = allocator
        .get_or_insert_native_module(
            &module_id(0x0a),
            native_protos,
            data.as_ptr(),
            data.len(),
            code.as_ptr(),
            code.len(),
        )
        .0;
    assert!(!mod_ref_a.native_module_ref_empty());

    let module = unsafe { &*mod_ref_a.native_module_ref_get() };
    let module_base_address = module.native_module_get_module_base_address();
    assert!(!module_base_address.is_null());

    let proto1 = module.native_module_try_get_native_proto(1);
    assert!(!proto1.is_null());
    unsafe {
        let header = get_native_proto_exec_data_header(proto1);
        assert_eq!(1, (*header).bytecode_id);
        assert_eq!(
            module_base_address.add(0x00),
            (*header).entry_offset_or_address
        );
        assert_eq!(0, *proto1.add(0));
        assert_eq!(4, *proto1.add(1));
    }

    let proto3 = module.native_module_try_get_native_proto(3);
    assert!(!proto3.is_null());
    unsafe {
        let header = get_native_proto_exec_data_header(proto3);
        assert_eq!(3, (*header).bytecode_id);
        assert_eq!(
            module_base_address.add(0x08),
            (*header).entry_offset_or_address
        );
        assert_eq!(8, *proto3.add(0));
        assert_eq!(12, *proto3.add(1));
    }

    assert!(module.native_module_try_get_native_proto(0).is_null());
    assert!(module.native_module_try_get_native_proto(2).is_null());
    assert!(module.native_module_try_get_native_proto(4).is_null());
}
