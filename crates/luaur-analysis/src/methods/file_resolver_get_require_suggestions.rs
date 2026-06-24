//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Method:Luau.Analysis:Analysis/src/FileResolver.cpp:153:file_resolver_get_require_suggestions`
//! Source: `Analysis/src/FileResolver.cpp`
//! Graph edges:
//! - declared_by: source_file Analysis/src/FileResolver.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/FileResolver.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Common/include/Luau/StringUtils.h
//! - incoming:
//!   - declares <- source_file Analysis/src/FileResolver.cpp
//!   - type_ref <- method RequireSuggester::getRequireSuggestions (Analysis/src/FileResolver.cpp)
//! - outgoing:
//!   - type_ref -> type_alias RequireSuggestions (Analysis/include/Luau/FileResolver.h)
//!   - type_ref -> method RequireSuggester::getRequireSuggestions (Analysis/src/FileResolver.cpp)
//!   - calls -> method RequireSuggester::getRequireSuggestions (Analysis/src/FileResolver.cpp)
//!   - type_ref -> record FileResolver (Analysis/include/Luau/FileResolver.h)
//!   - translates_to -> rust_item FileResolver::getRequireSuggestions

use crate::records::file_resolver::FileResolver;
use crate::type_aliases::module_name_file_resolver::ModuleName;
use crate::type_aliases::require_suggestions::RequireSuggestions;
use alloc::string::String;

impl FileResolver {
    /// C++ (FileResolver.cpp:153):
    /// `return requireSuggester ? requireSuggester->getRequireSuggestions(requirer, path) : std::nullopt;`
    pub fn get_require_suggestions(
        &self,
        requirer: &ModuleName,
        path: &Option<String>,
    ) -> Option<RequireSuggestions> {
        match &self.require_suggester {
            Some(suggester) => suggester.get_require_suggestions(requirer, path),
            None => None,
        }
    }
}
