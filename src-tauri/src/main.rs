// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

static mut global_int: i32 = 5;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet, test_action])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}



#[tauri::command]
fn greet(name: &str) -> String {
    let mut result: String = "".to_string();
    unsafe {
        global_int += 1;
        result = format!("Hello, {name} {global_int}!");
    }
    println!("greet() has executed.");
    return result;
}



#[tauri::command]
fn test_action() {
    unsafe {
        global_int += 1;
    }
    println!("test_action() has executed.");
}



