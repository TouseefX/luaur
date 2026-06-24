use crate::records::dcr_logger::DcrLogger;
use crate::records::generalize_step_snapshot::GeneralizeStepSnapshot;
use crate::type_aliases::step_snapshot::StepSnapshot;
use luaur_common::records::variant::Variant2;

impl DcrLogger {
    pub fn commit_step_snapshot(&mut self, snapshot: StepSnapshot) {
        if let Variant2::V1(eg) = &snapshot {
            if eg.before == eg.after {
                return;
            }
        }

        self.solve_log.step_states.push(snapshot);
    }
}
