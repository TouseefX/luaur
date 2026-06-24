use crate::records::subtyping_result::SubtypingResult;
use crate::type_aliases::component::Component;

impl SubtypingResult {
    pub fn with_both_component(&mut self, component: Component) -> &mut Self {
        self.with_sub_component(component.clone())
            .with_super_component(component)
    }
}
