use crate::config::app_config::AppConfig;
use rbatis::rbatis::Rbatis;

pub mod user;

pub async fn init_rbatis(config: &AppConfig) -> Rbatis {
    let rb = Rbatis::new();

    if config.server.debug.eq(&false) && rb.is_debug_mode() {
        panic!("已使用release模式，但是rbatis任使用debug模式，请修改配置");
    }

    println!(
        "[WEB] rbatis link database({})...",
        &config.database.database_url[0..config.database.database_url.find(":").unwrap_or(0)]
    );
    rb.link(&config.database.database_url)
        .await
        .expect("[WEB] rbatis link database fail!");
    println!("[WEB] rbatis link database success!");
    rb
}
