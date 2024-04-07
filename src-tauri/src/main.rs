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
use individuals::get_individual_html;
mod families;
use families::get_number_families;
use families::get_top_families;
use families::get_family_html;
mod sources;
use sources::get_top_sources;



fn main() {
    let mut settings = Settings::new();
    settings.load();

    tauri::Builder::default()
        // .manage(Settings::new())
        .manage(settings)
        .invoke_handler(tauri::generate_handler![greet, get_file_name, pick_file, get_header_tags, get_number_individuals, get_top_individuals, get_individual_html, get_number_families, get_top_families, get_family_html, get_top_sources])
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



// Returns the tags in the header.
#[tauri::command]
fn get_header_tags(state: State<Settings> ) -> String {
    // Get the header tags.
    let gedcom = state.family_tree.lock().unwrap();
    let tags = &gedcom.tags;

    let mut html: String = "<pre style=\"border: 1px solid black;  background-color: white;\">".to_string();
    for line in &tags.to_decorated_html()
    {
        html = format!("{}{}<br/>", html, line);
    }
    html += "</pre>";

    return html;
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




