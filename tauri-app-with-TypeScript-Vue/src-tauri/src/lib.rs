// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
// 返回字符串
fn get_greeting() -> String {
    "Hello from Rust!".into()
}

// 返回结构化数据（需实现 Serialize）
#[tauri::command]
fn get_user() -> serde_json::Value {
    serde_json::json!({
        "name": "Alice",
        "age": 30
    })
}

// 返回自定义结构体
#[derive(serde::Serialize)]
struct Settings {
    dark_mode: bool,
    language: String,
}

#[tauri::command]
fn get_settings() -> Settings {
    Settings {
        dark_mode: true,
        language: "en".into(),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_greeting,
            get_user,
            get_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
