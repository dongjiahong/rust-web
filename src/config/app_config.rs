use config;
// use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Server {
    /// 服务地址
    pub server_url: String,
    pub token_secret: String,
    /// debug 模式
    pub debug: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Database {
    /// 数据库地址
    pub database_url: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Logger {
    /// 日志目录
    pub log_dir: String,
    /// 日志条数上限
    pub log_cup: u64,
    /// 日志分割尺寸, "100MB", 单位：KB, MB, GB
    pub log_temp_size: String,
    /// 日志打包可选格式: ""(空-不压缩), "gzip", "zip", "lz4"(lz4压缩包非常快)
    pub log_pack_compress: String,
    /// 日志滚动配置: All【全部保留】, KeepTime(Duration)【按时间保留】，KeepNum(i64)【按版本保留】
    pub log_rolling_type: String,
    /// 日志等级, info、debug、error
    pub log_level: String,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppConfig {
    pub server: Server,
    pub logger: Logger,
    pub database: Database,
}

/// 默认配置
impl Default for AppConfig {
    fn default() -> Self {
        // 默认读取环境变量`APP_CONFIG`
        if let Ok(config_path) = env::var("APP_CONFIG") {
            let mut settings = config::Config::default();
            settings
                .merge(config::File::with_name(config_path.as_str()))
                .unwrap();
            match settings.try_into() {
                Ok(result) => result,
                Err(e) => panic!("deserialize err: {}, ", e),
            }
        } else {
            panic!("can't get config APP_CONFIG from env");
        }
    }
}

impl AppConfig {
    fn new(config_path: String) -> Self {
        let mut settings = config::Config::default();
        settings
            .merge(config::File::with_name(config_path.as_str()))
            .unwrap();
        match settings.try_into() {
            Ok(result) => result,
            Err(e) => panic!(
                "deserialize config err: {}, config_path: {}",
                e, config_path
            ),
        }
    }
}
