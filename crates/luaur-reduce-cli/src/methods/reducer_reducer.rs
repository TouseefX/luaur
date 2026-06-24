use crate::records::reducer::Reducer;

pub fn reducer_reducer(this: &mut Reducer) {
    this.parse_options.capture_comments = true;
    this.parse_options.store_cst_data = true;
}
