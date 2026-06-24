use crate::enums::subtyping_variance::SubtypingVariance;
use crate::functions::merge_reasonings::k_empty_reasoning;
use crate::records::subtyping_reasoning::SubtypingReasoning;
use crate::records::subtyping_result::SubtypingResult;
use crate::type_aliases::component::Component;
use crate::type_aliases::path::Path;
use crate::type_aliases::subtyping_reasonings::SubtypingReasonings;

impl SubtypingResult {
    pub fn with_sub_component(&mut self, component: Component) -> &mut Self {
        if self.reasoning.empty() {
            self.reasoning.insert(SubtypingReasoning {
                sub_path: Path::from_component(component),
                super_path: Path::default(),
                variance: SubtypingVariance::Covariant,
                is_property_modifier_violation: false,
            });
        } else {
            let mut updated = SubtypingReasonings::new(k_empty_reasoning());
            for r in self.reasoning.iter() {
                let mut r = r.clone();
                r.sub_path = r.sub_path.push_front(component.clone());
                updated.insert(r);
            }
            self.reasoning = updated;
        }

        self
    }
}
