use crate::functions::split_string_by_slashes::split_string_by_slashes;
use crate::records::test_require_node::TestRequireNode;
use luaur_analysis::records::require_node::RequireNode;

use luaur_common::macros::luau_assert::LUAU_ASSERT;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

impl TestRequireNode {
    pub fn resolve_path_to_node(&self, path: &str) -> Option<Box<dyn RequireNode>> {
        let components = split_string_by_slashes(path);
        LUAU_ASSERT!((components.is_empty() || components[0] == "." || components[0] == ".."));

        let mut normalized_components: Vec<&str> =
            split_string_by_slashes(self.module_name.as_str());
        normalized_components.pop();
        LUAU_ASSERT!(!normalized_components.is_empty());

        for component in components {
            if component == ".." {
                if normalized_components.is_empty() {
                    LUAU_ASSERT!(false);
                } else {
                    normalized_components.pop();
                }
            } else if !component.is_empty() && component != "." {
                normalized_components.push(component);
            }
        }

        let mut module_name = String::new();
        for (i, component) in normalized_components.iter().enumerate() {
            if i > 0 {
                module_name.push('/');
            }
            module_name.push_str(component);
        }

        let all_sources = unsafe { &*self.all_sources };
        if all_sources.get(&module_name).is_none() {
            return None;
        }

        Some(Box::new(TestRequireNode {
            module_name,
            all_sources: self.all_sources,
        }))
    }
}
