// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// System modules.
use tauri::api::dialog::blocking::FileDialogBuilder;
use std::sync::Mutex;
use tauri::State;

// Application modules.
mod gedcom_doc;



struct Counter {
    count: Mutex<i32>,
}



fn main() {
    // let mut gedcom_doc = gedcom_doc::GedcomDoc::new();

    tauri::Builder::default()
        .manage(Counter { count: Mutex::new(0) })
        .invoke_handler(tauri::generate_handler![greet, test_action])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



#[tauri::command]
fn greet(name: &str, state: State<Counter>) -> String {
    let mut result: String = "".to_string();

    println!("greet() has executed.");
    let mut counter = state.count.lock().unwrap();
    *counter = *counter + 1;
    result = format!("Hello, {name} {}.", *counter);
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



