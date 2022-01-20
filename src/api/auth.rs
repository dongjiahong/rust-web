use poem_openapi::{payload::Json, Object, types::Password, OpenApi, ApiResponse};

use crate::api;

/// 认证用户
#[derive(Debug, Clone, Object)]
pub struct Auth {
    /// 账户名
    #[oai(validator(max_length = 64))]
    name: String,
    /// 账户密码
    #[oai(validator(max_length = 32))]
    password: Password,
}

#[derive(ApiResponse)]
enum AuthUserResponse {
    /// Returns when the use auth is successfully
    #[oai(status = 200)]
    Ok(Json<i64>),
}

pub struct AuthApi;

#[OpenApi]
impl AuthApi {
    /// 用户认证
    #[oai(path = "/auth", method = "post", tag = "api::ApiTags::Auth")]
    async fn auth(&self, obj: Json<Auth>) -> Json<Auth> {
        todo!()
    }
}

