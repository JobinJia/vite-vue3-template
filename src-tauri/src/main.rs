// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app_lib;
use std::env;

fn main() {
    // 检查命令行参数
    let args: Vec<String> = env::args().collect();

    // 如果有--restore-mac参数，则执行还原MAC地址操作
    if args.len() > 1 && args[1] == "--restore-mac" {
        match app_lib::restore_mac_address() {
            Ok(_) => {
                println!("MAC地址已成功还原");
                return;
            }
            Err(e) => {
                eprintln!("还原MAC地址失败: {}", e);
                return;
            }
        }
    }

    // 正常启动应用
    app_lib::run();
}
