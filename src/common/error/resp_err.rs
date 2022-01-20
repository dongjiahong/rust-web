pub enum RespErr {
    Success,

    InvalidParams,
    AuthCheckTokenFail,
    AuthCheckTokenTimeOut,
    AuthTokenErr,
    AuthUserDisabled,

    Fail,
    DBError,
    CacheError,
    SmsError,
    EmailError,
}

impl RespErr {
    pub fn get(&self) -> (i32, String) {
        match *self {
            RespErr::Success => (200, "success".to_string()),

            RespErr::InvalidParams => (400, "参数错误".to_string()),
            RespErr::AuthCheckTokenFail => (4001, "Token鉴权失败".to_string()),
            RespErr::AuthCheckTokenTimeOut => (4002, "Token超时".to_string()),
            RespErr::AuthTokenErr => (4003, "Token生成失败".to_string()),
            RespErr::AuthUserDisabled => (403, "用户被禁用".to_string()),

            RespErr::Fail => (500, "服务异常".to_string()),
            RespErr::DBError => (5001, "服务繁忙请稍后".to_string()),
            RespErr::CacheError => (5002, "数据缓存异常".to_string()),
            RespErr::SmsError => (5003, "短信服务异常".to_string()),
            RespErr::EmailError => (5004, "邮件服务异常".to_string()),
        }
    }
}
