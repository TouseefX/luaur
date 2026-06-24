//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/SharedCodeAllocator.test.cpp:410:shared_code_allocator_shared_allocation`
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
//!   - type_ref -> type_alias UniqueSharedCodeGenContext (CodeGen/include/Luau/CodeGen.h)
//!   - calls -> function luaL_newstate (VM/src/linit.cpp)
//!   - calls -> function lua_close (VM/src/lstate.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function luau_compile (Compiler/src/lcode.cpp)
//!   - calls -> function luau_load (VM/src/lvmload.cpp)
//!   - calls -> method NativeModuleRef::reset (CodeGen/src/SharedCodeAllocator.cpp)
//!   - type_ref -> type_alias ModuleId (CodeGen/include/Luau/CodeGen.h)
//!   - type_ref -> record CompilationOptions (CodeGen/include/Luau/CodeGenOptions.h)
//!   - type_ref -> record CompilationStats (CodeGen/include/Luau/CodeGen.h)
//!   - type_ref -> record CompilationResult (CodeGen/include/Luau/CodeGen.h)
//!   - calls -> method FeedbackVectorFixture::compile (tests/FeedbackVector.test.cpp)
//!   - type_ref -> enum CodeGenCompilationResult (CodeGen/include/Luau/CodeGen.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - translates_to -> rust_item shared_code_allocator_shared_allocation

#[cfg(test)]
#[test]
fn shared_code_allocator_shared_allocation() {
    use crate::functions::shared_code_allocator_module_id::shared_code_allocator_module_id as module_id;
    use crate::type_aliases::state_ref::StateRef;
    use core::ffi::{c_char, c_void};
    use luaur_code_gen::enums::code_gen_compilation_result::CodeGenCompilationResult;
    use luaur_code_gen::enums::code_gen_flags::CodeGenFlags;
    use luaur_code_gen::functions::compile_internal::compile_internal;
    use luaur_code_gen::functions::create_code_gen_context_alt_d::create;
    use luaur_code_gen::functions::create_shared_code_gen_context_code_gen_context::create_shared_code_gen_context;
    use luaur_code_gen::functions::destroy_shared_code_gen_context::destroy_shared_code_gen_context;
    use luaur_code_gen::functions::luau_codegen_supported::luau_codegen_supported;
    use luaur_code_gen::records::compilation_options::CompilationOptions;
    use luaur_code_gen::records::compilation_stats::CompilationStats;
    use luaur_code_gen::records::shared_code_gen_context::SharedCodeGenContext;
    use luaur_code_gen::type_aliases::unique_shared_code_gen_context::UniqueSharedCodeGenContext;
    use luaur_compiler::functions::luau_compile::luau_compile;
    use luaur_vm::functions::lua_l_newstate::lua_l_newstate;
    use luaur_vm::functions::luau_load::luau_load;

    extern "C" {
        fn free(ptr: *mut c_void);
    }

    struct SharedContextRef(UniqueSharedCodeGenContext);

    impl SharedContextRef {
        fn as_ptr(&self) -> *mut SharedCodeGenContext {
            self.0.as_ptr()
        }
    }

    impl Drop for SharedContextRef {
        fn drop(&mut self) {
            unsafe {
                destroy_shared_code_gen_context(self.as_ptr());
            }
        }
    }

    if luau_codegen_supported() == 0 {
        return;
    }

    let shared_code_gen_context = SharedContextRef(create_shared_code_gen_context());

    let state1 = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
    let state2 = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
    let l1 = state1.as_ptr();
    let l2 = state2.as_ptr();

    create(l1, shared_code_gen_context.as_ptr());
    create(l2, shared_code_gen_context.as_ptr());

    let source = r#"
        function add(x, y) return x + y end
        function sub(x, y) return x - y end
    "#;

    let mut bytecode_size = 0usize;
    let bytecode = luau_compile(
        source.as_ptr() as *const c_char,
        source.len(),
        core::ptr::null_mut(),
        &mut bytecode_size,
    );
    assert!(!bytecode.is_null());

    unsafe {
        let load_result1 = luau_load(l1, c"=Functions".as_ptr(), bytecode, bytecode_size, 0);
        let load_result2 = luau_load(l2, c"=Functions".as_ptr(), bytecode, bytecode_size, 0);
        free(bytecode as *mut c_void);

        assert_eq!(0, load_result1);
        assert_eq!(0, load_result2);
    }

    let module_id = module_id(0x01);

    let mut options = CompilationOptions::default();
    options.flags = CodeGenFlags::CodeGen_ColdFunctions as u32;

    let mut native_stats1 = CompilationStats::default();
    let mut native_stats2 = CompilationStats::default();
    let code_gen_result1 = unsafe {
        compile_internal(
            &Some(module_id),
            l1,
            -1,
            &options,
            &mut native_stats1 as *mut CompilationStats,
        )
    };
    let code_gen_result2 = unsafe {
        compile_internal(
            &Some(module_id),
            l2,
            -1,
            &options,
            &mut native_stats2 as *mut CompilationStats,
        )
    };

    assert_eq!(CodeGenCompilationResult::Success, code_gen_result1.result);
    assert_eq!(CodeGenCompilationResult::Success, code_gen_result2.result);

    assert_eq!(3, native_stats1.functions_total);
    assert_eq!(3, native_stats2.functions_total);

    assert_eq!(3, native_stats1.functions_compiled);
    assert_eq!(0, native_stats2.functions_compiled);

    assert_eq!(3, native_stats1.functions_bound);
    assert_eq!(3, native_stats2.functions_bound);
}
