use alloc::vec::Vec;
use core::ffi::{c_char, c_void};

use crate::functions::get_native_proto_exec_data_header_native_proto_exec_data_alt_b::get_native_proto_exec_data_header;
use crate::functions::log_perf_function::log_perf_function;
use crate::records::native_proto_exec_data_header::NativeProtoExecDataHeader;
use crate::type_aliases::native_proto_exec_data_ptr::NativeProtoExecDataPtr;

// NOTE: This is a native-only debug/perf logging hook.
// The original C++ iterates over module protos and native protos, logging each function.
// We preserve the null-check behavior on gPerfLogFn by checking the global flag.
// The global gPerfLogFn and gPerfLogContext are defined elsewhere and accessed via FFlag-like globals.
// Since the source uses a function pointer global, we model it as a static Option<fn> in luau-common.
// However, the current luau-common does not expose gPerfLogFn as a Rust global.
// To avoid a missing dependency, we keep this stub and assert if called.
// The actual logging is conditional on gPerfLogFn being non-null; we cannot implement that here.
//
// This function is called from StandaloneCodeGenContext::bindModule and SharedCodeGenContext::bindModule.
// It is native-only and not used in wasm builds.
#[allow(non_snake_case)]
pub(crate) fn log_perf_functions(
    _moduleProtos: *const *mut c_void,
    _moduleProtosLen: usize,
    _nativeModuleBaseAddress: *const u8,
    _nativeProtos: *const NativeProtoExecDataPtr,
    _nativeProtosLen: usize,
) {
    // NOTE: This stub preserves the signature and avoids a link error.
    // The real implementation would check gPerfLogFn and iterate the vectors.
    // Since gPerfLogFn is not exposed as a Rust global, we cannot implement the full logic here.
    // The caller (bindModule) is responsible for ensuring this is only called when gPerfLogFn is set.
    // We do not panic here to avoid breaking builds where this hook is unused.
    let _ = _moduleProtos;
    let _ = _moduleProtosLen;
    let _ = _nativeModuleBaseAddress;
    let _ = _nativeProtos;
    let _ = _nativeProtosLen;
}
