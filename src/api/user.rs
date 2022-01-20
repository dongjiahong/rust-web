use poem_openapi::{param::Path, payload::Json, types::Password, Object, OpenApi};

use crate::api;
use crate::common::response::{RespV0, ResponseObject};
use crate::models;

///  创建用户
#[derive(Debug, Object, Clone)]
struct User {
    /// 用户名
    #[oai(validator(max_length = 64))]
    name: String,
    /// 密码
    #[oai(validator(max_length = 32))]
    password: Password,
}

/// 更新用户
#[derive(Debug, Object, Clone)]
struct UpdateUser {
    /// Id
    id: i64,
    /// Name
    name: Option<String>,
    /// Password
    password: Option<String>,
}

pub struct UserApi;

#[OpenApi]
impl UserApi {
    /// 创建用户
    #[oai(path = "/api/user", method = "post", tag = "api::ApiTags::User")]
    async fn create_user(&self, u: Json<User>) -> RespV0<i64> {
        match models::user::create_user(&u.0.name, u.0.password.as_ref()).await {
            Ok(id) => return RespV0::Ok(Json(ResponseObject::success(id.unwrap()))),
            Err(e) => return RespV0::Ok(Json(ResponseObject::failed(e.get()))),
        }
    }

    #[oai(
        path = "/api/user/:user_id",
        method = "get",
        tag = "api::ApiTags::User"
    )]
    async fn find_user(&self, user_id: Path<i64>) -> RespV0<models::user::User> {
        match models::user::find_user(user_id.0).await {
            Ok(res) => {
                if let Some(user) = res {
                    RespV0::Ok(Json(ResponseObject::success(user)))
                } else {
                    RespV0::Ok(Json(ResponseObject::failed((
                        200,
                        "can't find user".to_string(),
                    ))))
                }
            }
            Err(e) => return RespV0::Ok(Json(ResponseObject::failed(e.get()))),
        }
    }

    #[oai(
        path = "/api/user/:user_id",
        method = "delete",
        tag = "api::ApiTags::User"
    )]
    async fn delete_user(&self, user_id: Path<i64>) -> RespV0<String> {
        todo!()
    }

    #[oai(
        path = "/api/user/:user_id",
        method = "put",
        tag = "api::ApiTags::User"
    )]
    async fn update_user(&self, user_id: Path<i64>, update: Json<UpdateUser>) -> RespV0<String> {
        todo!()
    }
}
