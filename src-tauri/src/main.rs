// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use tauri::api::dialog::blocking::FileDialogBuilder;





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
// The async allows the function to run not on the main thread and allow blocking dialogs.
async fn test_action() {
    let dialog_result = tauri::api::dialog::blocking::FileDialogBuilder::new()
        .add_filter("Markdown", &["md", "jpg"])
        .pick_file();
    match dialog_result {
        None => {
            println!("Cancel.");
        }
        Some(path) => {
            println!("OK.");
            let path_name = path.to_string_lossy();
            println!("{path_name}");
        }
    }
}



