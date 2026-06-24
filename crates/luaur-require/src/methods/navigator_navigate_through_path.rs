use crate::functions::split_path::split_path;
use crate::records::navigator::Navigator;
use crate::type_aliases::error_require_navigator::Error;

impl Navigator {
    pub fn navigate_through_path(&mut self, path: &str) -> Error {
        let (mut first, mut second) = split_path(path);
        if !path.is_empty() && path.starts_with('@') {
            let components = split_path(second);
            first = components.0;
            second = components.1;
        }

        let mut previous_component: Option<alloc::string::String> = None;
        while !(first.is_empty() && second.is_empty()) {
            if first == "." || first.is_empty() {
                let components = split_path(second);
                first = components.0;
                second = components.1;
                continue;
            } else if first == ".." {
                if let Some(error) = self.navigate_to_parent(previous_component.clone()) {
                    return Some(error);
                }
            } else {
                if let Some(error) = self.navigate_to_child(first) {
                    return Some(error);
                }
            }
            previous_component = Some(alloc::string::String::from(first));
            let components = split_path(second);
            first = components.0;
            second = components.1;
        }

        None
    }
}
