// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn custom_greet(name: &str) -> String {
    format!("Hello, {}! You have been greeted by Thorben", name)
}

#[tauri::command]
fn init_sodoku() -> Vec<Vec<i32>> {
    let mut sodoku = Vec::with_capacity(9);
    (0..9).into_iter().for_each(|_| {
        sodoku.push(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    });
    sodoku
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, custom_greet, init_sodoku])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
