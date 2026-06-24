//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:999:type_stringifier_operator_call`
//! Source: `Analysis/src/ToString.cpp:999-1079` (hand-ported)

use crate::functions::follow_type::follow;
use crate::functions::get_type_alt_j::get;
use crate::functions::is_overloaded_function::is_overloaded_function;
use crate::records::element_result::ElementResult;
use crate::records::function_type::FunctionType;
use crate::records::intersection_type::IntersectionType;
use crate::records::to_string_span::ToStringSpan;
use crate::records::type_stringifier::TypeStringifier;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;
use core::ffi::c_void;

impl TypeStringifier {
    /// C++ `void operator()(TypeId ty, const IntersectionType& uv)`.
    /// NOTE: unlike the union arm, C++ iterates `uv.parts` directly here
    /// (no flattening iterator).
    pub fn operator_call_4(&mut self, ty: TypeId, uv: &IntersectionType) {
        unsafe {
            if (*self.state).has_seen(uv as *const IntersectionType as *const c_void) {
                (*(*self.state).result).cycle = true;
                (*self.state).emit("*CYCLE*");
                return;
            }

            let mut results: Vec<ElementResult> = Vec::new();
            let mut results_length: usize = 0;
            let mut length_limit_hit = false;

            for &part in uv.parts.iter() {
                let el = follow(part);

                let saved = core::mem::take(&mut (*(*self.state).result).name);
                let saved_spans_size = (&(*(*self.state).result).type_spans).len();

                let need_parens = !(*self.state).cycle_names.contains(&el)
                    && (!get::<UnionType>(el).is_null() || !get::<FunctionType>(el).is_null());

                if need_parens {
                    (*self.state).emit("(");
                }

                self.stringify_type_id(el);

                if need_parens {
                    (*self.state).emit(")");
                }

                let mut elem = ElementResult::default();
                elem.str = core::mem::take(&mut (*(*self.state).result).name);

                for i in saved_spans_size..(&(*(*self.state).result).type_spans).len() {
                    elem.spans.push((&(*(*self.state).result).type_spans)[i]);
                }
                (*(*self.state).result)
                    .type_spans
                    .truncate(saved_spans_size);

                results_length += elem.str.len();
                results.push(elem);

                (*(*self.state).result).name = saved;

                let max_type_length = (*(*self.state).opts).max_type_length;
                length_limit_hit = max_type_length > 0 && results_length > max_type_length;

                if length_limit_hit {
                    break;
                }
            }

            (*self.state).unsee(uv as *const IntersectionType as *const c_void);

            if !length_limit_hit && !luaur_common::FFlag::DebugLuauToStringNoLexicalSort.get() {
                results.sort_unstable_by(|a, b| a.str.cmp(&b.str));
            }

            let mut first = true;
            let should_place_on_newlines = results.len()
                > (*(*self.state).opts).composite_types_single_line_limit
                || is_overloaded_function(ty);
            for elem in results.iter() {
                if !first {
                    if should_place_on_newlines {
                        (*self.state).newline();
                    } else {
                        (*self.state).emit(" ");
                    }
                    (*self.state).emit("& ");
                }

                let base_pos = (&(*(*self.state).result).name).len();
                (*self.state).emit(elem.str.as_str());
                for span in elem.spans.iter() {
                    (*(*self.state).result).type_spans.push(ToStringSpan {
                        start_pos: base_pos + span.start_pos,
                        end_pos: base_pos + span.end_pos,
                        r#type: span.r#type,
                    });
                }

                first = false;
            }
        }
    }
}
