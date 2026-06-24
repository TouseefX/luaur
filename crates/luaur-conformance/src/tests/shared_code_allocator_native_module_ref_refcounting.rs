//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/SharedCodeAllocator.test.cpp:28:shared_code_allocator_native_module_ref_refcounting`
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
//!   - calls -> method SharedCodeAllocator::tryGetNativeModule (CodeGen/src/SharedCodeAllocator.cpp)
//!   - type_ref -> type_alias ModuleId (CodeGen/include/Luau/CodeGen.h)
//!   - type_ref -> record NativeModuleRef (CodeGen/include/Luau/SharedCodeAllocator.h)
//!   - calls -> method SharedCodeAllocator::getOrInsertNativeModule (CodeGen/src/SharedCodeAllocator.cpp)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - calls -> method NativeModule::getRefcount (CodeGen/src/SharedCodeAllocator.cpp)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - calls -> method NativeModuleRef::reset (CodeGen/src/SharedCodeAllocator.cpp)
//!   - calls -> method NativeModuleRef::swap (CodeGen/src/SharedCodeAllocator.cpp)
//!   - calls -> method Path::last (Analysis/src/TypePath.cpp)
//!   - translates_to -> rust_item shared_code_allocator_native_module_ref_refcounting

#[cfg(test)]
#[test]
fn shared_code_allocator_native_module_ref_refcounting() {
    use crate::functions::shared_code_allocator_module_id::shared_code_allocator_module_id as module_id;
    use alloc::vec::Vec;
    use luaur_code_gen::functions::luau_codegen_supported::luau_codegen_supported;
    use luaur_code_gen::records::code_allocator::CodeAllocator;
    use luaur_code_gen::records::native_module_ref::NativeModuleRef;
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

    let refcount = |module_ref: &NativeModuleRef| unsafe {
        (*module_ref.native_module_ref_get()).native_module_get_refcount()
    };

    assert!(allocator
        .try_get_native_module(&module_id(0x0a))
        .native_module_ref_empty());

    let mut mod_ref_a = allocator
        .get_or_insert_native_module(
            &module_id(0x0a),
            Vec::new(),
            core::ptr::null(),
            0,
            FAKE_CODE.as_ptr(),
            FAKE_CODE.len(),
        )
        .0;
    assert!(!mod_ref_a.native_module_ref_empty());

    assert_eq!(
        allocator
            .try_get_native_module(&module_id(0x0a))
            .native_module_ref_get(),
        mod_ref_a.native_module_ref_get()
    );

    assert_eq!(
        allocator
            .get_or_insert_native_module(
                &module_id(0x0a),
                Vec::new(),
                core::ptr::null(),
                0,
                FAKE_CODE.as_ptr(),
                FAKE_CODE.len(),
            )
            .0
            .native_module_ref_get(),
        mod_ref_a.native_module_ref_get()
    );

    assert!(allocator
        .try_get_native_module(&module_id(0x0b))
        .native_module_ref_empty());

    let mod_ref_b = allocator
        .get_or_insert_native_module(
            &module_id(0x0b),
            Vec::new(),
            core::ptr::null(),
            0,
            FAKE_CODE.as_ptr(),
            FAKE_CODE.len(),
        )
        .0;
    assert!(!mod_ref_b.native_module_ref_empty());
    assert_ne!(
        mod_ref_b.native_module_ref_get(),
        mod_ref_a.native_module_ref_get()
    );

    assert_eq!(1, refcount(&mod_ref_a));
    assert_eq!(1, refcount(&mod_ref_b));

    {
        let mod_ref1 = mod_ref_a.clone();
        assert_eq!(
            mod_ref1.native_module_ref_get(),
            mod_ref_a.native_module_ref_get()
        );
        assert_eq!(2, refcount(&mod_ref_a));
    }

    assert_eq!(1, refcount(&mod_ref_a));
    assert_eq!(1, refcount(&mod_ref_b));

    {
        let mod_ref1 = NativeModuleRef::default();
        let mod_ref2 = mod_ref1.clone();
        assert!(mod_ref1.native_module_ref_empty());
        assert!(mod_ref2.native_module_ref_empty());
    }

    assert_eq!(1, refcount(&mod_ref_a));
    assert_eq!(1, refcount(&mod_ref_b));

    {
        let mut mod_ref1 = mod_ref_a.clone();
        let mod_ref2 = NativeModuleRef::native_module_ref_native_module_ref_mut(&mut mod_ref1);
        assert!(mod_ref1.native_module_ref_empty());
        assert_eq!(
            mod_ref2.native_module_ref_get(),
            mod_ref_a.native_module_ref_get()
        );
        assert_eq!(2, refcount(&mod_ref_a));
    }

    assert_eq!(1, refcount(&mod_ref_a));
    assert_eq!(1, refcount(&mod_ref_b));

    {
        let mut mod_ref1 = NativeModuleRef::default();
        let mod_ref2 = NativeModuleRef::native_module_ref_native_module_ref_mut(&mut mod_ref1);
        assert!(mod_ref1.native_module_ref_empty());
        assert!(mod_ref2.native_module_ref_empty());
    }

    assert_eq!(1, refcount(&mod_ref_a));
    assert_eq!(1, refcount(&mod_ref_b));

    {
        let mut mod_ref1 = NativeModuleRef::default();
        mod_ref1.native_module_ref_operator_assign(mod_ref_a.clone());
        assert_eq!(
            mod_ref1.native_module_ref_get(),
            mod_ref_a.native_module_ref_get()
        );
        assert_eq!(2, refcount(&mod_ref_a));
    }

    assert_eq!(1, refcount(&mod_ref_a));
    assert_eq!(1, refcount(&mod_ref_b));

    {
        let mod_ref1 = NativeModuleRef::default();
        let mut mod_ref2 = NativeModuleRef::default();
        mod_ref2.native_module_ref_operator_assign(mod_ref1.clone());
        assert!(mod_ref1.native_module_ref_empty());
        assert!(mod_ref2.native_module_ref_empty());
    }

    assert_eq!(1, refcount(&mod_ref_a));
    assert_eq!(1, refcount(&mod_ref_b));

    {
        let mut mod_ref1 = mod_ref_a.clone();
        mod_ref1.native_module_ref_operator_assign(mod_ref1.clone());
        assert_eq!(
            mod_ref1.native_module_ref_get(),
            mod_ref_a.native_module_ref_get()
        );
        assert_eq!(2, refcount(&mod_ref_a));
    }

    assert_eq!(1, refcount(&mod_ref_a));
    assert_eq!(1, refcount(&mod_ref_b));

    {
        let mod_ref1 = mod_ref_a.clone();
        let mut mod_ref2 = mod_ref_b.clone();
        mod_ref2.native_module_ref_operator_assign(mod_ref1.clone());
        assert_eq!(
            mod_ref1.native_module_ref_get(),
            mod_ref_a.native_module_ref_get()
        );
        assert_eq!(
            mod_ref2.native_module_ref_get(),
            mod_ref_a.native_module_ref_get()
        );
        assert_eq!(3, refcount(&mod_ref_a));
        assert_eq!(1, refcount(&mod_ref_b));
    }

    assert_eq!(1, refcount(&mod_ref_a));
    assert_eq!(1, refcount(&mod_ref_b));

    {
        let mut mod_ref1 = mod_ref_a.clone();
        let mut mod_ref2 = NativeModuleRef::default();
        let moved = NativeModuleRef::native_module_ref_native_module_ref_mut(&mut mod_ref1);
        mod_ref2.native_module_ref_operator_assign(moved);
        assert!(mod_ref1.native_module_ref_empty());
        assert_eq!(
            mod_ref2.native_module_ref_get(),
            mod_ref_a.native_module_ref_get()
        );
        assert_eq!(2, refcount(&mod_ref_a));
    }

    assert_eq!(1, refcount(&mod_ref_a));
    assert_eq!(1, refcount(&mod_ref_b));

    {
        let mut mod_ref1 = NativeModuleRef::default();
        let mut mod_ref2 = NativeModuleRef::default();
        let moved = NativeModuleRef::native_module_ref_native_module_ref_mut(&mut mod_ref1);
        mod_ref2.native_module_ref_operator_assign(moved);
        assert!(mod_ref1.native_module_ref_empty());
        assert!(mod_ref2.native_module_ref_empty());
    }

    assert_eq!(1, refcount(&mod_ref_a));
    assert_eq!(1, refcount(&mod_ref_b));

    #[cfg(not(target_os = "linux"))]
    {
        let mut mod_ref1 = mod_ref_a.clone();
        let mod_ref1_ptr: *mut NativeModuleRef = &mut mod_ref1;
        unsafe {
            let moved =
                NativeModuleRef::native_module_ref_native_module_ref_mut(&mut *mod_ref1_ptr);
            (*mod_ref1_ptr).native_module_ref_operator_assign(moved);
        }
        assert_eq!(
            mod_ref1.native_module_ref_get(),
            mod_ref_a.native_module_ref_get()
        );
        assert_eq!(2, refcount(&mod_ref_a));
    }

    assert_eq!(1, refcount(&mod_ref_a));
    assert_eq!(1, refcount(&mod_ref_b));

    {
        let mut mod_ref1 = mod_ref_a.clone();
        let mut mod_ref2 = mod_ref_b.clone();
        let moved = NativeModuleRef::native_module_ref_native_module_ref_mut(&mut mod_ref1);
        mod_ref2.native_module_ref_operator_assign(moved);
        assert!(mod_ref1.native_module_ref_empty());
        assert_eq!(
            mod_ref2.native_module_ref_get(),
            mod_ref_a.native_module_ref_get()
        );
        assert_eq!(2, refcount(&mod_ref_a));
        assert_eq!(1, refcount(&mod_ref_b));
    }

    assert_eq!(1, refcount(&mod_ref_a));
    assert_eq!(1, refcount(&mod_ref_b));

    {
        let mut mod_ref1 = NativeModuleRef::default();
        mod_ref1.native_module_ref_reset();
        assert!(mod_ref1.native_module_ref_empty());
    }

    assert_eq!(1, refcount(&mod_ref_a));
    assert_eq!(1, refcount(&mod_ref_b));

    {
        let mut mod_ref1 = mod_ref_a.clone();
        mod_ref1.native_module_ref_reset();
        assert!(mod_ref1.native_module_ref_empty());
        assert_eq!(1, refcount(&mod_ref_a));
    }

    assert_eq!(1, refcount(&mod_ref_a));
    assert_eq!(1, refcount(&mod_ref_b));

    {
        let mut mod_ref1 = mod_ref_a.clone();
        let mut mod_ref2 = mod_ref_b.clone();
        mod_ref1.native_module_ref_swap(&mut mod_ref2);
        assert_eq!(
            mod_ref1.native_module_ref_get(),
            mod_ref_b.native_module_ref_get()
        );
        assert_eq!(
            mod_ref2.native_module_ref_get(),
            mod_ref_a.native_module_ref_get()
        );
        assert_eq!(2, refcount(&mod_ref_a));
        assert_eq!(2, refcount(&mod_ref_b));
    }

    assert_eq!(1, refcount(&mod_ref_a));
    assert_eq!(1, refcount(&mod_ref_b));

    mod_ref_a.native_module_ref_reset();
    assert!(allocator
        .try_get_native_module(&module_id(0x0a))
        .native_module_ref_empty());
}
