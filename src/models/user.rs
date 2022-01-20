use log::error;
use poem_openapi::Object;
use rbatis::crud::CRUD;

use crate::common::error::{resp_err::RespErr, Result};
use crate::common::CONTEXT;

#[crud_table(table_name: "t_user")]
#[derive(Clone, Debug, Object)]
pub struct User {
    pub id: Option<i64>,
    pub account: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
}

impl_field_name_method!(User {
    id,
    account,
    name,
    password
});

//查询使用name查询user
pub async fn get_user_by_name(name: &str) -> Result<Option<User>> {
    let wrapper = CONTEXT.rb.new_wrapper().eq(User::name(), name);
    match CONTEXT.rb.fetch_by_wrapper(wrapper).await {
        Ok(user) => Ok(user),
        Err(err) => {
            error!("get user by name, err: {}", err);
            return Err(RespErr::Fail);
        }
    }
}

pub async fn find_user(id: i64) -> Result<Option<User>> {
    let wrapper = CONTEXT.rb.new_wrapper().eq(User::id(), id);
    match CONTEXT.rb.fetch_by_wrapper(wrapper).await {
        Ok(user) => Ok(user),
        Err(err) => {
            error!("find user by id, err: {}", err);
            return Err(RespErr::Fail);
        }
    }
}

// 创建user
pub async fn create_user(name: &str, password: &str) -> Result<Option<i64>> {
    let u = User {
        id: None,
        account: None,
        name: Some(name.to_string()),
        password: Some(password.to_string()),
    };
    match CONTEXT.rb.save(&u, &[]).await {
        Ok(id) => Ok(id.last_insert_id),
        Err(err) => {
            error!("create user err: {}", err);
            return Err(RespErr::Fail);
        }
    }
}
