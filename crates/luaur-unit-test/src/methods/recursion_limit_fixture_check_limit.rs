//! Port of `RecursionLimitFixture::checkLimit` (tests/Compiler.test.cpp:162).
use crate::records::recursion_limit_fixture::RecursionLimitFixture;
use crate::type_aliases::scoped_fast_int::ScopedFastInt;
use alloc::string::{String, ToString};
use luaur_ast::records::parse_errors::ParseErrors;
use luaur_ast::records::parse_options::ParseOptions;
use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;
use luaur_compiler::records::compile_error::CompileError;
use luaur_compiler::records::compile_options::CompileOptions;

impl RecursionLimitFixture {
    pub fn check_limit(&mut self, code: &str, message: &str) {
        // findLimit is a manual diagnostic (sweep LuauRecursionLimit to discover the
        // current limit); it is never enabled by the ported tests, all of which take
        // the `else` branch below: `CHECK_THROWS_AS_MESSAGE(compileOrThrow(bcb, code),
        // std::exception, message)`.
        if self.find_limit {
            unimplemented!("RecursionLimitFixture::checkLimit findLimit mode is unused by tests");
        }

        // The fixture forcibly pushes the parser's recursion depth toward its limit.
        // In an unoptimized build each native frame is large, so C++ lowers the limit
        // (`#elif defined(_NOOPT) || defined(_DEBUG): ScopedFastInt{LuauRecursionLimit, 300}`)
        // to raise the ParseError before the C stack overflows. We do the same: tests run
        // as debug builds on small (2MB) libtest thread stacks, and 1000 frames overflow.
        let _recursion_limit = ScopedFastInt::new(&luaur_common::FInt::LuauRecursionLimit, 300);

        let bcb = &mut self.bcb;
        let code = code.to_string();
        let options = CompileOptions::default();
        let parse_options = ParseOptions::default();

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            compile_or_throw_bytecode_builder_string_compile_options_parse_options(
                bcb,
                &code,
                &options,
                &parse_options,
            );
        }));

        assert!(
            result.is_err(),
            "expected compileOrThrow to throw with message: {message}"
        );

        // C++ checks the thrown std::exception's `.what()` equals `message`. The
        // panic payload is the typed exception object (ParseErrors / CompileError),
        // both of which render their message via Display.
        let payload = result.unwrap_err();
        let what = if let Some(e) = payload.downcast_ref::<ParseErrors>() {
            alloc::format!("{e}")
        } else if let Some(e) = payload.downcast_ref::<CompileError>() {
            alloc::format!("{e}")
        } else if let Some(s) = payload.downcast_ref::<String>() {
            s.clone()
        } else if let Some(s) = payload.downcast_ref::<&'static str>() {
            s.to_string()
        } else {
            panic!("checkLimit: unexpected panic payload type");
        };

        assert_eq!(what, message, "exception message mismatch");
    }
}
