//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:213:type_infer_builtins_lua_51_exported_globals_all_exist`
//! Source: `tests/TypeInfer.builtins.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
//! - outgoing:
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function rep (tests/Fixture.cpp)
//!   - calls -> function match (VM/src/lstrlib.cpp)
//!   - calls -> function gmatch (VM/src/lstrlib.cpp)
//!   - calls -> function format (tests/StringUtils.test.cpp)
//!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
//!   - calls -> macro tostring (VM/src/lvm.h)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - calls -> macro tonumber (VM/src/lvm.h)
//!   - calls -> function write (tests/JsonEmitter.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function min (Analysis/include/Luau/Unifiable.h)
//!   - calls -> macro setupvalue (VM/src/lobject.h)
//!   - calls -> function getn (VM/src/ltablib.cpp)
//!   - calls -> function foreachi (VM/src/ltablib.cpp)
//!   - calls -> function maxn (VM/src/ltablib.cpp)
//!   - calls -> function foreach (VM/src/ltablib.cpp)
//!   - calls -> method TxnLog::concat (Analysis/src/TxnLog.cpp)
//!   - calls -> function resume (VM/src/ldo.cpp)
//!   - calls -> function load (Config/src/LuauConfig.cpp)
//!   - translates_to -> rust_item type_infer_builtins_lua_51_exported_globals_all_exist

#[cfg(test)]
#[test]
fn type_infer_builtins_lua_51_exported_globals_all_exist() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local v__G = _G
        local v_string_sub = string.sub
        local v_string_upper = string.upper
        local v_string_len = string.len
        local v_string_rep = string.rep
        local v_string_find = string.find
        local v_string_match = string.match
        local v_string_char = string.char
        local v_string_gmatch = string.gmatch
        local v_string_reverse = string.reverse
        local v_string_byte = string.byte
        local v_string_format = string.format
        local v_string_gsub = string.gsub
        local v_string_lower = string.lower

        local v_xpcall = xpcall

        --local v_package_loadlib = package.loadlib
        --local v_package_loaders_1_ = package.loaders[1]
        --local v_package_loaders_2_ = package.loaders[2]
        --local v_package_loaders_3_ = package.loaders[3]
        --local v_package_loaders_4_ = package.loaders[4]

        local v_tostring = tostring
        local v_print = print

        --local v_os_exit = os.exit
        --local v_os_setlocale = os.setlocale
        local v_os_date = os.date
        --local v_os_getenv = os.getenv
        local v_os_difftime = os.difftime
        --local v_os_remove = os.remove
        local v_os_time = os.time
        --local v_os_clock = os.clock
        --local v_os_tmpname = os.tmpname
        --local v_os_rename = os.rename
        --local v_os_execute = os.execute

        local v_unpack = unpack
        local v_require = require
        local v_getfenv = getfenv
        local v_setmetatable = setmetatable
        local v_next = next
        local v_assert = assert
        local v_tonumber = tonumber

        --local v_io_lines = io.lines
        --local v_io_write = io.write
        --local v_io_close = io.close
        --local v_io_flush = io.flush
        --local v_io_open = io.open
        --local v_io_output = io.output
        --local v_io_type = io.type
        --local v_io_read = io.read
        --local v_io_stderr = io.stderr
        --local v_io_stdin = io.stdin
        --local v_io_input = io.input
        --local v_io_stdout = io.stdout
        --local v_io_popen = io.popen
        --local v_io_tmpfile = io.tmpfile

        local v_rawequal = rawequal
        --local v_collectgarbage = collectgarbage
        local v_getmetatable = getmetatable
        local v_rawset = rawset

        local v_math_log = math.log
        local v_math_max = math.max
        local v_math_acos = math.acos
        local v_math_huge = math.huge
        local v_math_ldexp = math.ldexp
        local v_math_pi = math.pi
        local v_math_cos = math.cos
        local v_math_tanh = math.tanh
        local v_math_pow = math.pow
        local v_math_deg = math.deg
        local v_math_tan = math.tan
        local v_math_cosh = math.cosh
        local v_math_sinh = math.sinh
        local v_math_random = math.random
        local v_math_randomseed = math.randomseed
        local v_math_frexp = math.frexp
        local v_math_ceil = math.ceil
        local v_math_floor = math.floor
        local v_math_rad = math.rad
        local v_math_abs = math.abs
        local v_math_sqrt = math.sqrt
        local v_math_modf = math.modf
        local v_math_asin = math.asin
        local v_math_min = math.min
        --local v_math_mod = math.mod
        local v_math_fmod = math.fmod
        local v_math_log10 = math.log10
        local v_math_atan2 = math.atan2
        local v_math_exp = math.exp
        local v_math_sin = math.sin
        local v_math_atan = math.atan

        --local v_debug_getupvalue = debug.getupvalue
        --local v_debug_debug = debug.debug
        --local v_debug_sethook = debug.sethook
        --local v_debug_getmetatable = debug.getmetatable
        --local v_debug_gethook = debug.gethook
        --local v_debug_setmetatable = debug.setmetatable
        --local v_debug_setlocal = debug.setlocal
        --local v_debug_traceback = debug.traceback
        --local v_debug_setfenv = debug.setfenv
        --local v_debug_getinfo = debug.getinfo
        --local v_debug_setupvalue = debug.setupvalue
        --local v_debug_getlocal = debug.getlocal
        --local v_debug_getregistry = debug.getregistry
        --local v_debug_getfenv = debug.getfenv

        local v_pcall = pcall

        --local v_table_setn = table.setn
        local v_table_insert = table.insert
        --local v_table_getn = table.getn
        --local v_table_foreachi = table.foreachi
        local v_table_maxn = table.maxn
        --local v_table_foreach = table.foreach
        local v_table_concat = table.concat
        local v_table_sort = table.sort
        local v_table_remove = table.remove

        local v_newproxy = newproxy
        local v_type = type

        local v_coroutine_resume = coroutine.resume
        local v_coroutine_yield = coroutine.yield
        local v_coroutine_status = coroutine.status
        local v_coroutine_wrap = coroutine.wrap
        local v_coroutine_create = coroutine.create
        local v_coroutine_running = coroutine.running

        local v_select = select
        local v_gcinfo = gcinfo
        local v_pairs = pairs
        local v_rawget = rawget
        local v_loadstring = loadstring
        local v_ipairs = ipairs
        local v__VERSION = _VERSION
        --local v_dofile = dofile
        local v_setfenv = setfenv
        --local v_load = load
        local v_error = error
        --local v_loadfile = loadfile
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
