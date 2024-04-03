// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// System modules.
use tauri::api::dialog::blocking::FileDialogBuilder;
use tauri::State;

// Application modules.
mod settings;
use settings::Settings;
mod individuals;
use individuals::get_number_individuals;
use individuals::get_top_individuals;
mod families;
use families::get_number_families;



fn main() {
    let mut settings = Settings::new();
    settings.load();

    tauri::Builder::default()
        // .manage(Settings::new())
        .manage(settings)
        .invoke_handler(tauri::generate_handler![greet, get_file_name, pick_file, get_number_individuals, get_top_individuals, get_number_families])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    println!("main() finished.");
}



#[tauri::command]
fn greet(name: &str, state: State<Settings> ) -> String {
    println!("greet() has executed.");
    let mut count = state.count.lock().unwrap();
    *count += 1;

    let result = format!("Hello, {name} {}.", *count);
    return result;
}



// Return the current file name in the settings.
#[tauri::command]
fn get_file_name(state: State<Settings> ) -> String {
    let test = state.file_name.lock().unwrap();
    let file_name = test.to_string_lossy();

    let result = format!("{}", file_name);
    return result;
}


// The async allows the function to run not on the main thread and allow blocking dialogs.
// The '_, is required because this is an async function.
// In Tauri, async function have to return something.  Without the state you get away without a return, but it is actually required.
// async fn pick_file(state: State<'_, Settings>) -> Result<String, ()> {    // Return Ok("Hello".to_string());
// async fn pick_file(state: State<'_, Settings>) -> Result<bool, ()> {      // return Ok(true);
#[tauri::command]
async fn pick_file(state: State<'_, Settings>) -> Result<(), ()> {           // return Ok(());
    println!("pick_file() start.");
    let dialog_result = FileDialogBuilder::new()
        .add_filter("Gedcom", &["ged", "gedcom"])
        .add_filter("Markdown", &["md", "jpg"])
        .add_filter("All Files", &["*"])
        .pick_file();
    match dialog_result {
        None => {
            println!("Cancel.");
        }
        Some(path) => {
            println!("OK.");
            let path_name :String = path.to_string_lossy().to_string();
            println!("{path_name}");
            let mut file_name = state.file_name.lock().unwrap();
            *file_name = path;

            // Release the mutex lock, otherwise the save will not work.
            drop(file_name);

            // Save the current settings.
            state.save();
        }
    }
    println!("pick_file() finish.");
    // return Ok("Hello".to_string());
    // return Ok(true);
    return Ok(());
}




