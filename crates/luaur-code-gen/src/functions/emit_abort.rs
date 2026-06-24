use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::label::Label;

pub fn emit_abort(build: &mut AssemblyBuilderA64, abort: &mut Label) {
    let mut skip = Label { id: 0, location: 0 };
    build.b_label(&mut skip);
    build.place_b(core::ptr::null(), abort, 0); // setLabel(abort)
    build.udf();
    build.place_b(core::ptr::null(), &mut skip, 0); // setLabel(skip)
}
