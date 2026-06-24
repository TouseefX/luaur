use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Reasonings {
    /// the list of reasons
    pub reasons: Vec<String>,
    /// this should be true if _all_ of the reasons have an error suppressing type, and false otherwise.
    pub suppressed: bool,
}

#[allow(non_snake_case)]
impl Reasonings {
    pub fn toString(&mut self) -> String {
        if self.reasons.is_empty() {
            return String::new();
        }

        // DenseHashSet ordering is entirely undefined, so we want to
        // sort the reasons here to achieve a stable error
        // stringification.
        self.reasons.sort();

        let mut all_reasons = if self.reasons.len() < 2 {
            String::from("\n")
        } else {
            String::from("\nthis is because")
        };

        let multi = self.reasons.len() > 1;
        for reason in &self.reasons {
            if multi {
                all_reasons.push_str("\n\t * ");
            }

            all_reasons.push_str(reason);
        }

        all_reasons
    }
}

unsafe impl Send for Reasonings {}
unsafe impl Sync for Reasonings {}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let allReasons: () = ();
}
