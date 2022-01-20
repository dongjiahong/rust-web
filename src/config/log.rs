use std::time::Duration;

use crate::common::CONTEXT;
use fast_log;
use fast_log::consts::LogSize;
use fast_log::plugin::file_split::{Packer, RollingType};
use fast_log::plugin::packer::{GZipPacker, LZ4Packer, LogPacker, ZipPacker};
pub fn init_log() {
    // create log dir
    let _ = std::fs::create_dir_all(&CONTEXT.app_config.logger.log_dir);

    // fast log这个库有点问题，配置日志目录时，好像是通过字符串末尾是否有'/'来判断的
    let mut log_dir = String::new();
    if !CONTEXT.app_config.logger.log_dir.ends_with("/") {
        log_dir = CONTEXT.app_config.logger.log_dir.clone() + "/";
    }
    // init fast log
    let _ = fast_log::init_split_log(
        &log_dir,
        str_to_temp_size(&CONTEXT.app_config.logger.log_temp_size),
        str_to_rolling(&CONTEXT.app_config.logger.log_rolling_type),
        str_to_log_level(&CONTEXT.app_config.logger.log_level),
        None,
        choose_packer(&CONTEXT.app_config.logger.log_pack_compress),
        CONTEXT.app_config.server.debug,
    )
    .unwrap();
    if CONTEXT.app_config.server.debug == false {
        println!("[WEB] release_mode is up! [file log] open, [console_log] disabled!");
    }
}

fn choose_packer(packer: &str) -> Box<dyn Packer> {
    match packer {
        "lz4" => Box::new(LZ4Packer {}),
        "zip" => Box::new(ZipPacker {}),
        "gzip" => Box::new(GZipPacker {}),
        _ => Box::new(LogPacker {}),
    }
}

fn str_to_temp_size(arg: &str) -> LogSize {
    match arg {
        arg if arg.ends_with("MB") => {
            let end = arg.find("MB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::MB(num.parse::<usize>().unwrap())
        }
        arg if arg.ends_with("KB") => {
            let end = arg.find("KB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::KB(num.parse::<usize>().unwrap())
        }
        arg if arg.ends_with("GB") => {
            let end = arg.find("GB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::GB(num.parse::<usize>().unwrap())
        }
        _ => LogSize::MB(100),
    }
}

fn str_to_rolling(arg: &str) -> RollingType {
    match arg {
        arg if arg.starts_with("KeepNum(") => {
            let end = arg.find(")").unwrap();
            let num = arg["KeepNum(".len()..end].to_string();
            RollingType::KeepNum(num.parse::<i64>().unwrap())
        }
        arg if arg.starts_with("KeepTime(") => {
            let end = arg.find(")").unwrap();
            let num = arg["KeepTime(".len()..end].to_string();
            RollingType::KeepTime(Duration::from_secs(num.parse::<u64>().unwrap()))
        }
        _ => RollingType::All,
    }
}

fn str_to_log_level(arg: &str) -> log::Level {
    return match arg {
        "warn" => log::Level::Warn,
        "error" => log::Level::Error,
        "trace" => log::Level::Trace,
        "info" => log::Level::Info,
        "debug" => log::Level::Debug,
        _ => log::Level::Info,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    use fast_log::{
        consts, init_split_log,
        plugin::{self, packer::ZipPacker},
    };
    use log::{debug, info};
    #[test]
    pub fn test_file_compation() {
        let _ = init_split_log(
            "target/logs/",         // 文件存储的目录
            consts::LogSize::MB(1), // 日志文件的大小
            plugin::file_split::RollingType::All,
            log::Level::Info,
            None,
            Box::new(ZipPacker {}), // 压缩类型，默认zip
            true,
        ); // or Box::new(LZ4Packer{})
        for _ in 0..2000 {
            info!("Commencing yak shaving");
            debug!("----debug----");
        }
        sleep(Duration::from_secs(1));
    }

    #[test]
    fn test_str_to_log_level() {
        assert_eq!(str_to_log_level("warn"), log::Level::Warn);
        assert_eq!(str_to_log_level("error"), log::Level::Error);
        assert_eq!(str_to_log_level("trace"), log::Level::Trace);
        assert_eq!(str_to_log_level("info"), log::Level::Info);
        assert_eq!(str_to_log_level("debug"), log::Level::Debug);
        assert_eq!(str_to_log_level("Warn"), log::Level::Info);
        assert_eq!(str_to_log_level(" "), log::Level::Info);
    }
}
