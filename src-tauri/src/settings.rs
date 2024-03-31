// Modules to hold the settings for the Tauri front end.

use std::path::PathBuf;
use std::sync::Mutex;


// Member variables for the 'GedComDoc' class.
pub struct Settings {
    // This is a count variable just for testing.
    pub count: Mutex<i32>,

    // The file name of the gedcom file.
    pub file_name: Mutex<String>,

    // Testing.
    pub test: Mutex<PathBuf>,
}


impl Settings {
    pub fn new() -> Settings {
        return Settings{count: Mutex::new(0), file_name: Mutex::new(String::new()), test: Mutex::new(PathBuf::new())};
    }
}
