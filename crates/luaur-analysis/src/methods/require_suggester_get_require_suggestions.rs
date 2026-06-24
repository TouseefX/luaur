//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Method:Luau.Analysis:Analysis/src/FileResolver.cpp:148:require_suggester_get_require_suggestions`
//! Source: `Analysis/src/FileResolver.cpp`
//! Graph edges:
//! - declared_by: source_file Analysis/src/FileResolver.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/FileResolver.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Common/include/Luau/StringUtils.h
//! - incoming:
//!   - declares <- source_file Analysis/src/FileResolver.cpp
//!   - type_ref <- method FileResolver::getRequireSuggestions (Analysis/src/FileResolver.cpp)
//!   - calls <- method FileResolver::getRequireSuggestions (Analysis/src/FileResolver.cpp)
//! - outgoing:
//!   - type_ref -> type_alias RequireSuggestions (Analysis/include/Luau/FileResolver.h)
//!   - type_ref -> method FileResolver::getRequireSuggestions (Analysis/src/FileResolver.cpp)
//!   - calls -> function processRequireSuggestions (Analysis/src/FileResolver.cpp)
//!   - calls -> method RequireSuggester::getRequireSuggestionsImpl (Analysis/src/FileResolver.cpp)
//!   - type_ref -> record RequireSuggester (Analysis/include/Luau/FileResolver.h)
//!   - translates_to -> rust_item RequireSuggester::getRequireSuggestions

use crate::functions::process_require_suggestions::process_require_suggestions;
use crate::records::require_suggester::RequireSuggester;
use crate::type_aliases::module_name_file_resolver::ModuleName;
use crate::type_aliases::require_suggestions::RequireSuggestions;
use alloc::string::String;

impl RequireSuggester {
    /// C++ (FileResolver.cpp:148):
    /// `return processRequireSuggestions(getRequireSuggestionsImpl(requirer, path));`
    pub fn get_require_suggestions(
        &self,
        requirer: &ModuleName,
        path: &Option<String>,
    ) -> Option<RequireSuggestions> {
        process_require_suggestions(self.get_require_suggestions_impl(requirer, path))
    }
}
