// Rust标准库：获取命令行参数值
// use std::env::args;
use std::{env, process};
use lxmon_minigrep::Config;


fn main() {
    // 获取命令行参数
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    // 处理文件
    if let Err(e) = lxmon_minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}