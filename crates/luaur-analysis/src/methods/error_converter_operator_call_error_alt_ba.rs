use crate::records::error_converter::ErrorConverter;
use crate::records::unexpected_array_like_table_item::UnexpectedArrayLikeTableItem;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_57(&self, _error: &UnexpectedArrayLikeTableItem) -> String {
        String::from(
            "Unexpected array-like table item: the indexer key type of this table is not `number`.",
        )
    }
}
