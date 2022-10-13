// Rust标准库：获取命令行参数值
// use std::env::args;
use std::{env, process};

use minigrep::Config;


fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // println!("{:?}",args);
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    
    // 处理文件
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}