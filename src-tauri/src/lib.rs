use tauri::command;
use serde::Serialize;
use std::fs;
use std::path::Path;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use serde::Deserialize;
use std::process::Command;
use regex;
use std::path::PathBuf;
use std::io::{ Read, Write};
use dirs;

#[derive(Serialize)]
struct FileEntry {
    id: u64,                      // 唯一标识符
    name: String,
    is_dir: bool,
    selected: bool,
    #[serde(skip_serializing_if = "Option::is_none")]  // 如果为 None 则跳过序列化
    children: Option<Vec<FileEntry>>, // 递归结构
}

impl FileEntry {
    fn new(name: String, is_dir: bool, path: &Path, children: Option<Vec<FileEntry>>) -> Self {
        let id = generate_id(&name, path);
        FileEntry {
            id,
            name,
            is_dir,
            selected: false,
            children,
        }
    }
}

// 生成唯一的 ID
fn generate_id(name: &str, path: &Path) -> u64 {
    let mut hasher = DefaultHasher::new();
    (name, path).hash(&mut hasher);
    hasher.finish()
}

#[command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn read_directory(path: &Path, depth: usize) -> Result<Vec<FileEntry>, String> {
    let mut entries = vec![];

    for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let metadata = entry.metadata().map_err(|e| e.to_string())?;

        if metadata.is_dir() {
            let dir_name = entry.file_name().into_string().unwrap_or_default();

            let mut is_role = true;

            if dir_name == "userpreferences" {
                continue;
            }

            let subdir = read_directory(&entry.path(), depth + 1)?;

            if (depth == 3 || depth == 2 || depth == 1) && subdir.is_empty() {
                continue;
            }

            if depth == 4 {
                is_role = false;
            }

            let children = if subdir.is_empty() {
                None
            } else {
                Some(subdir)
            };

            let dir_entry = FileEntry::new(dir_name, is_role, &entry.path(), children);
            entries.push(dir_entry);
        }
    }

    Ok(entries)
}

#[command]
fn list_directory_contents(path: &str) -> Result<Vec<FileEntry>, String> {
    let path = std::path::Path::new(path);
    if !path.is_dir() {
        return Err("提供的路径不是一个目录".to_string());
    }

    read_directory(path, 1) // 从深度 1 开始
}


#[derive(Deserialize)]
struct CopyParams {
    source_path: String,
    target_path: String,
}

#[command]
fn cp_source_to_target(params: CopyParams) -> bool {
    let source_path = Path::new(&params.source_path);
    let target_path = Path::new(&params.target_path);

    // 处理过程中的错误将返回 false
    if target_path.exists() {
        if let Err(_) = fs::remove_dir_all(&target_path) {
            return false;
        }
        if let Err(_) = fs::create_dir_all(&target_path) {
            return false;
        }
    } else {
        if let Err(_) = fs::create_dir_all(&target_path) {
            return false;
        }
    }

    // 尝试复制目录
    if copy_dir_all(&source_path, &target_path).is_err() {
        return false;
    }

    // 如果一切顺利，返回 true
    true
}

// 递归复制目录
fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), String> {
    if src.is_dir() {
        fs::create_dir_all(dst).map_err(|e| e.to_string())?;
        for entry in fs::read_dir(src).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let file_type = entry.file_type().map_err(|e| e.to_string())?;
            let new_dst = dst.join(entry.file_name());
            if file_type.is_dir() {
                copy_dir_all(&entry.path(), &new_dst)?;
            } else {
                fs::copy(entry.path(), new_dst).map_err(|e| e.to_string())?;
            }
        }
    } else {
        return Err(format!("Source path {} is not a directory", src.display()));
    }
    Ok(())
}

#[command]
fn get_mac_address() -> Result<String, String> {
    // 在macOS上使用ifconfig命令获取MAC地址
    #[cfg(target_os = "macos")]
    {
        let output = Command::new("ifconfig")
            .arg("en0") // 通常en0是主要网络接口
            .output()
            .map_err(|e| e.to_string())?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        // 使用正则表达式提取MAC地址
        let re = regex::Regex::new(r"ether\s+([0-9A-Fa-f]{2}:[0-9A-Fa-f]{2}:[0-9A-Fa-f]{2}:[0-9A-Fa-f]{2}:[0-9A-Fa-f]{2}:[0-9A-Fa-f]{2})")
            .map_err(|e| e.to_string())?;

        if let Some(captures) = re.captures(&stdout) {
            if let Some(mac) = captures.get(1) {
                return Ok(mac.as_str().to_string());
            }
        }

        // 如果en0没有找到，尝试其他接口
        let output = Command::new("ifconfig")
            .output()
            .map_err(|e| e.to_string())?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        if let Some(captures) = re.captures(&stdout) {
            if let Some(mac) = captures.get(1) {
                return Ok(mac.as_str().to_string());
            }
        }

        Err("无法找到MAC地址".to_string())
    }

    // 在Windows上使用ipconfig /all命令获取MAC地址
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("ipconfig")
            .arg("/all")
            .output()
            .map_err(|e| e.to_string())?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        // 使用正则表达式提取MAC地址
        let re = regex::Regex::new(r"Physical Address[\.\s]+: ([0-9A-Fa-f]{2}-[0-9A-Fa-f]{2}-[0-9A-Fa-f]{2}-[0-9A-Fa-f]{2}-[0-9A-Fa-f]{2}-[0-9A-Fa-f]{2})")
            .map_err(|e| e.to_string())?;

        if let Some(captures) = re.captures(&stdout) {
            if let Some(mac) = captures.get(1) {
                // 将Windows格式的MAC地址（用连字符分隔）转换为标准格式（用冒号分隔）
                return Ok(mac.as_str().replace("-", ":"));
            }
        }

        Err("无法找到MAC地址".to_string())
    }

    // 在Linux上使用ip命令获取MAC地址
    #[cfg(target_os = "linux")]
    {
        let output = Command::new("ip")
            .args(&["link", "show"])
            .output()
            .map_err(|e| e.to_string())?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        // 使用正则表达式提取MAC地址
        let re = regex::Regex::new(r"link/ether\s+([0-9a-f]{2}:[0-9a-f]{2}:[0-9a-f]{2}:[0-9a-f]{2}:[0-9a-f]{2}:[0-9a-f]{2})")
            .map_err(|e| e.to_string())?;

        if let Some(captures) = re.captures(&stdout) {
            if let Some(mac) = captures.get(1) {
                return Ok(mac.as_str().to_string());
            }
        }

        Err("无法找到MAC地址".to_string())
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        Err("不支持的操作系统".to_string())
    }
}

// 添加新的restore_mac_address函数
pub fn restore_mac_address() -> Result<(), String> {
    // 在实际应用中，您可能需要存储原始MAC地址
    // 这里我们简单地重启网络接口，让系统恢复原始MAC地址

    #[cfg(target_os = "macos")]
    {
        // 重启网络接口
        let _ = Command::new("sudo")
            .args(&["ifconfig", "en0", "down"])
            .output()
            .map_err(|e| e.to_string())?;

        let _ = Command::new("sudo")
            .args(&["ifconfig", "en0", "up"])
            .output()
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    #[cfg(target_os = "windows")]
    {
        // 在Windows上，通常需要禁用然后重新启用网络适配器
        // 首先获取网络适配器名称
        let output = Command::new("ipconfig")
            .arg("/all")
            .output()
            .map_err(|e| e.to_string())?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        // 使用正则表达式提取适配器名称
        let re = regex::Regex::new(r"Ethernet adapter ([\w\s]+):")
            .map_err(|e| e.to_string())?;

        let adapter_name = if let Some(captures) = re.captures(&stdout) {
            if let Some(name) = captures.get(1) {
                name.as_str().to_string()
            } else {
                return Err("无法找到网络适配器名称".to_string());
            }
        } else {
            return Err("无法找到网络适配器名称".to_string());
        };

        // 禁用网络适配器
        let _ = Command::new("netsh")
            .args(&["interface", "set", "interface", &adapter_name, "admin=disable"])
            .output()
            .map_err(|e| e.to_string())?;

        // 启用网络适配器
        let output = Command::new("netsh")
            .args(&["interface", "set", "interface", &adapter_name, "admin=enable"])
            .output()
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(format!("还原MAC地址失败: {}", error));
        }

        Ok(())
    }

    #[cfg(target_os = "linux")]
    {
        // 首先获取网络接口名称
        let output = Command::new("ip")
            .args(&["link", "show"])
            .output()
            .map_err(|e| e.to_string())?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        // 使用正则表达式提取接口名称
        let re = regex::Regex::new(r"^\d+:\s+(\w+):")
            .map_err(|e| e.to_string())?;

        let interface_name = if let Some(captures) = re.captures(&stdout) {
            if let Some(name) = captures.get(1) {
                name.as_str().to_string()
            } else {
                return Err("无法找到网络接口名称".to_string());
            }
        } else {
            return Err("无法找到网络接口名称".to_string());
        };

        // 重启网络接口
        let _ = Command::new("sudo")
            .args(&["ip", "link", "set", &interface_name, "down"])
            .output()
            .map_err(|e| e.to_string())?;

        let output = Command::new("sudo")
            .args(&["ip", "link", "set", &interface_name, "up"])
            .output()
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(format!("还原MAC地址失败: {}", error));
        }

        Ok(())
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        Err("不支持的操作系统".to_string())
    }
}

// 存储自动还原设置的文件路径
fn get_auto_restore_file_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("jx3-tools");
    fs::create_dir_all(&path).unwrap_or_default();
    path.push("mac_auto_restore.txt");
    path
}

#[command]
fn get_auto_restore_setting() -> Result<bool, String> {
    let path = get_auto_restore_file_path();

    if !path.exists() {
        return Ok(false); // 默认不自动还原
    }

    let mut file = fs::File::open(path).map_err(|e| e.to_string())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| e.to_string())?;

    Ok(contents.trim() == "true")
}

#[command]
fn set_auto_restore_setting(auto_restore: bool) -> Result<(), String> {
    let path = get_auto_restore_file_path();

    let mut file = fs::File::create(path).map_err(|e| e.to_string())?;
    file.write_all(if auto_restore { b"true" } else { b"false" }).map_err(|e| e.to_string())?;

    // 根据操作系统设置开机自启动
    if auto_restore {
        setup_auto_restore_on_boot()?;
    } else {
        remove_auto_restore_on_boot()?;
    }

    Ok(())
}

// 设置开机自动还原MAC地址
fn setup_auto_restore_on_boot() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        // 在macOS上使用launchd
        let launch_agents_dir = dirs::home_dir()
            .ok_or_else(|| "无法获取用户主目录".to_string())?
            .join("Library/LaunchAgents");

        fs::create_dir_all(&launch_agents_dir).map_err(|e| e.to_string())?;

        let plist_path = launch_agents_dir.join("com.jx3tools.mac-restore.plist");
        let app_path = std::env::current_exe().map_err(|e| e.to_string())?;

        let plist_content = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.jx3tools.mac-restore</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
        <string>--restore-mac</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
</dict>
</plist>"#,
            app_path.to_string_lossy()
        );

        let mut file = fs::File::create(&plist_path).map_err(|e| e.to_string())?;
        file.write_all(plist_content.as_bytes()).map_err(|e| e.to_string())?;

        // 加载启动项
        let plist_path_str = plist_path.to_string_lossy().to_string();
        let _ = Command::new("launchctl")
            .args(&["load", "-w", &plist_path_str])
            .output()
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    #[cfg(target_os = "windows")]
    {
        // 在Windows上使用任务计划程序
        let app_path = std::env::current_exe().map_err(|e| e.to_string())?;

        let task_name = "JX3ToolsMacRestore";

        // 创建任务计划
        let _ = Command::new("schtasks")
            .args(&[
                "/create",
                "/tn", task_name,
                "/tr", &format!("\"{}\" --restore-mac", app_path.to_string_lossy()),
                "/sc", "onlogon",
                "/f"  // 强制创建，如果已存在则覆盖
            ])
            .output()
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    #[cfg(target_os = "linux")]
    {
        // 在Linux上使用systemd用户服务
        let config_dir = dirs::config_dir()
            .ok_or_else(|| "无法获取配置目录".to_string())?
            .join("systemd/user");

        fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;

        let service_path = config_dir.join("jx3tools-mac-restore.service");
        let app_path = std::env::current_exe().map_err(|e| e.to_string())?;

        let service_content = format!(
            r#"[Unit]
Description=JX3 Tools MAC Address Restore
After=network.target

[Service]
Type=oneshot
ExecStart={} --restore-mac

[Install]
WantedBy=default.target"#,
            app_path.to_string_lossy()
        );

        let mut file = fs::File::create(service_path).map_err(|e| e.to_string())?;
        file.write_all(service_content.as_bytes()).map_err(|e| e.to_string())?;

        // 启用服务
        let _ = Command::new("systemctl")
            .args(&["--user", "enable", "jx3tools-mac-restore.service"])
            .output()
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        Err("不支持的操作系统".to_string())
    }
}

// 移除开机自动还原MAC地址
fn remove_auto_restore_on_boot() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        // 在macOS上使用launchd
        let plist_path = dirs::home_dir()
            .ok_or_else(|| "无法获取用户主目录".to_string())?
            .join("Library/LaunchAgents/com.jx3tools.mac-restore.plist");

        if plist_path.exists() {
            // 卸载启动项
            let _ = Command::new("launchctl")
                .args(&["unload", "-w", &plist_path.to_string_lossy()])
                .output()
                .map_err(|e| e.to_string())?;

            // 删除plist文件
            fs::remove_file(plist_path).map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    #[cfg(target_os = "windows")]
    {
        // 在Windows上使用任务计划程序
        let task_name = "JX3ToolsMacRestore";

        // 删除任务计划
        let _ = Command::new("schtasks")
            .args(&["/delete", "/tn", task_name, "/f"])
            .output()
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    #[cfg(target_os = "linux")]
    {
        // 在Linux上使用systemd用户服务
        let service_path = dirs::config_dir()
            .ok_or_else(|| "无法获取配置目录".to_string())?
            .join("systemd/user/jx3tools-mac-restore.service");

        if service_path.exists() {
            // 禁用服务
            let _ = Command::new("systemctl")
                .args(&["--user", "disable", "jx3tools-mac-restore.service"])
                .output()
                .map_err(|e| e.to_string())?;

            // 删除服务文件
            fs::remove_file(service_path).map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        Err("不支持的操作系统".to_string())
    }
}

#[command]
fn change_mac_address(mac_address: &str) -> Result<(), String> {
    // 注意：修改MAC地址通常需要管理员/root权限

    // 在macOS上使用ifconfig命令修改MAC地址
    #[cfg(target_os = "macos")]
    {
        // 首先关闭网络接口
        let _ = Command::new("sudo")
            .args(&["ifconfig", "en0", "down"])
            .output()
            .map_err(|e| e.to_string())?;

        // 修改MAC地址
        let output = Command::new("sudo")
            .args(&["ifconfig", "en0", "ether", mac_address])
            .output()
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(format!("修改MAC地址失败: {}", error));
        }

        // 重新启用网络接口
        let _ = Command::new("sudo")
            .args(&["ifconfig", "en0", "up"])
            .output()
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    // 在Windows上使用netsh命令修改MAC地址
    #[cfg(target_os = "windows")]
    {
        // 首先获取网络适配器名称
        let output = Command::new("ipconfig")
            .arg("/all")
            .output()
            .map_err(|e| e.to_string())?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        // 使用正则表达式提取适配器名称
        let re = regex::Regex::new(r"Ethernet adapter ([\w\s]+):")
            .map_err(|e| e.to_string())?;

        let adapter_name = if let Some(captures) = re.captures(&stdout) {
            if let Some(name) = captures.get(1) {
                name.as_str().to_string()
            } else {
                return Err("无法找到网络适配器名称".to_string());
            }
        } else {
            return Err("无法找到网络适配器名称".to_string());
        };

        // 修改MAC地址
        let output = Command::new("netsh")
            .args(&["interface", "set", "interface", &adapter_name, "newmac", &mac_address.replace(":", "-")])
            .output()
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(format!("修改MAC地址失败: {}", error));
        }

        Ok(())
    }

    // 在Linux上使用ip命令修改MAC地址
    #[cfg(target_os = "linux")]
    {
        // 首先获取网络接口名称
        let output = Command::new("ip")
            .args(&["link", "show"])
            .output()
            .map_err(|e| e.to_string())?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        // 使用正则表达式提取接口名称
        let re = regex::Regex::new(r"^\d+:\s+(\w+):")
            .map_err(|e| e.to_string())?;

        let interface_name = if let Some(captures) = re.captures(&stdout) {
            if let Some(name) = captures.get(1) {
                name.as_str().to_string()
            } else {
                return Err("无法找到网络接口名称".to_string());
            }
        } else {
            return Err("无法找到网络接口名称".to_string());
        };

        // 首先关闭网络接口
        let _ = Command::new("sudo")
            .args(&["ip", "link", "set", &interface_name, "down"])
            .output()
            .map_err(|e| e.to_string())?;

        // 修改MAC地址
        let output = Command::new("sudo")
            .args(&["ip", "link", "set", &interface_name, "address", mac_address])
            .output()
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(format!("修改MAC地址失败: {}", error));
        }

        // 重新启用网络接口
        let _ = Command::new("sudo")
            .args(&["ip", "link", "set", &interface_name, "up"])
            .output()
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        Err("不支持的操作系统".to_string())
    }
}

#[command]
fn restore_mac_cmd() -> Result<(), String> {
    restore_mac_address()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, list_directory_contents, cp_source_to_target, get_mac_address, get_auto_restore_setting, set_auto_restore_setting, change_mac_address, restore_mac_cmd])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
