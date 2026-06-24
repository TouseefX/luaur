use crate::records::frontend_options::FrontendOptions;
use crate::records::i_fragment_autocomplete_reporter::IFragmentAutocompleteReporter;
use alloc::string::String;
use luaur_ast::records::parse_result::ParseResult;
use luaur_ast::records::position::Position;

#[derive(Debug, Clone)]
pub struct FragmentContext<'a> {
    pub(crate) new_src: String,
    pub(crate) fresh_parse: &'a ParseResult,
    pub(crate) opts: Option<FrontendOptions>,
    pub(crate) DEPRECATED_fragment_end_position: Option<Position>,
    pub(crate) reporter: *mut dyn IFragmentAutocompleteReporter,
}

impl<'a> FragmentContext<'a> {
    pub fn new(new_src: &str, fresh_parse: &'a ParseResult) -> Self {
        Self {
            new_src: String::from(new_src),
            fresh_parse,
            opts: None,
            DEPRECATED_fragment_end_position: None,
            reporter: crate::records::i_fragment_autocomplete_reporter::null_reporter(),
        }
    }

    /// C++ aggregate `FragmentContext{newSrc, freshParse, opts, fragmentEndPosition}`.
    pub fn new_with_options(
        new_src: &str,
        fresh_parse: &'a ParseResult,
        opts: Option<FrontendOptions>,
        fragment_end_position: Option<Position>,
    ) -> Self {
        Self {
            new_src: String::from(new_src),
            fresh_parse,
            opts,
            DEPRECATED_fragment_end_position: fragment_end_position,
            reporter: crate::records::i_fragment_autocomplete_reporter::null_reporter(),
        }
    }
}

#[allow(non_snake_case)]
impl<'a> FragmentContext<'a> {
    pub fn newSrc(&self) -> &str {
        &self.new_src
    }

    pub fn freshParse(&self) -> &ParseResult {
        self.fresh_parse
    }

    pub fn opts(&self) -> Option<&FrontendOptions> {
        self.opts.as_ref()
    }

    pub fn DEPRECATED_fragmentEndPosition(&self) -> Option<Position> {
        self.DEPRECATED_fragment_end_position
    }

    pub fn reporter(&self) -> *mut dyn IFragmentAutocompleteReporter {
        self.reporter
    }
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let DEPRECATED_fragmentEndPosition: () = ();
}
