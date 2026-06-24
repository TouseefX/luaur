pub type FunctionType = extern "C" fn(i64, Option<extern "C" fn(i64)>) -> i64;
