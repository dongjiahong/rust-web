pub mod resp_err;

pub type Result<T> = std::result::Result<T, resp_err::RespErr>;
