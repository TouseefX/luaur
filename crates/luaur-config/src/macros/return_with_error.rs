macro_rules! RETURN_WITH_ERROR {
    ($msg:expr) => {
        return {
            if let Some(error_ptr) = error {
                *error_ptr = $msg.to_string();
            }
            None
        };
    };
}

pub(crate) use RETURN_WITH_ERROR;
