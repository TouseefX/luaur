use crate::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
use crate::functions::to_string_to_string_alt_n::to_string_type_pack_id_to_string_options;
use crate::functions::to_string_to_string_alt_q::to_string_constraint_to_string_options;
use crate::records::constraint::Constraint;
use crate::records::constraint_graph::ConstraintGraph;
use crate::records::to_string_options::ToStringOptions;
use crate::type_aliases::constraint_vertex::ConstraintVertex;
use alloc::vec::Vec;
use core::ptr::NonNull;

impl ConstraintGraph {
    pub fn dump_with(
        &mut self,
        unsolved_constraints: &Vec<NonNull<Constraint>>,
        opts: &mut ToStringOptions,
    ) {
        // TODO: It might be nice to *also* dump the types here.
        std::print!("constraints:\n");
        for c in unsolved_constraints.iter() {
            let c_ptr = c.as_ptr() as *const Constraint;
            let deps = self.find_dependency_list(ConstraintVertex::V2(c_ptr));
            let deps_ref = unsafe { deps.as_ref() };
            std::print!(
                "\t{}\t{}\n",
                deps_ref.size(),
                to_string_constraint_to_string_options(unsafe { &*c_ptr }, opts)
            );

            for dep in deps_ref.order.iter() {
                // The C++ `for (auto dep : *deps)` iterates only present entries.
                if !deps_ref.contains(dep.clone()) {
                    continue;
                }

                if let Some(ty) = dep.get_if_0() {
                    std::print!(
                        "\t\t|\tType {}\n",
                        to_string_type_id_to_string_options(*ty, opts)
                    );
                } else if let Some(tp) = dep.get_if_1() {
                    std::print!(
                        "\t\t|\tPack {}\n",
                        to_string_type_pack_id_to_string_options(*tp, opts)
                    );
                } else if let Some(cons) = dep.get_if_2() {
                    std::print!(
                        "\t\t|\tCons {}\n",
                        to_string_constraint_to_string_options(unsafe { &**cons }, opts)
                    );
                }
            }
        }
    }
}
