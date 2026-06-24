use alloc::string::String;

use crate::records::reasonings::Reasonings;

impl Reasonings {
    pub fn to_string(&mut self) -> String {
        if self.reasons.is_empty() {
            return String::new();
        }

        self.reasons.sort();

        let mut all_reasons = if self.reasons.len() < 2 {
            String::from("\n")
        } else {
            String::from("\nthis is because ")
        };

        for reason in &self.reasons {
            if self.reasons.len() > 1 {
                all_reasons.push_str("\n\t * ");
            }
            all_reasons.push_str(reason);
        }

        all_reasons
    }
}
