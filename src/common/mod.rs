use crate::config;
use crate::models;
use rbatis::rbatis::Rbatis;

pub mod error;
pub mod response;
pub struct AppContext {
    pub app_config: config::app_config::AppConfig,
    pub rb: Rbatis,
}

impl Default for AppContext {
    fn default() -> Self {
        let app_config = config::app_config::AppConfig::default();
        AppContext {
            app_config: app_config.clone(),
            rb: async_std::task::block_on(async {
                models::init_rbatis(&app_config).await
            })
            //rb: tokio::runtime::Runtime::new()
            //.unwrap()
            //.block_on(models::init_rbatis(&app_config)),
        }
    }
}

lazy_static! {
    pub static ref CONTEXT: AppContext = AppContext::default();
}
