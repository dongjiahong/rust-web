/*
 * 参考： https://github.dev/poem-web/poem/blob/master/examples/openapi/todos/src/main.rs
*/
use poem_openapi::{
    payload::Json,
    types::{ParseFromJSON, ToJSON},
    ApiResponse, Object,
};

#[derive(Object)]
#[oai(inline)]
pub struct ResponseObject<T: ParseFromJSON + ToJSON + Send + Sync> {
    code: i32,
    msg: String,
    data: Option<T>,
}

impl<T: ParseFromJSON + ToJSON + Send + Sync> ResponseObject<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            msg: "OK".to_string(),
            data: Some(data),
        }
    }

    pub fn failed(f: (i32, String)) -> Self {
        Self {
            code: f.0,
            msg: f.1,
            data: None,
        }
    }
}

#[derive(ApiResponse)]
pub enum RespV0<T: ParseFromJSON + ToJSON + Send + Sync> {
    #[oai(status = 200)]
    Ok(Json<ResponseObject<T>>),
}
