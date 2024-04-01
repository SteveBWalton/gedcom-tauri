// Modules to hold the settings for the Tauri front end.

use std::path::PathBuf;
use std::sync::Mutex;


// Member variables for the 'GedComDoc' class.
pub struct Settings {
    // This is a count variable just for testing.
    pub count: Mutex<i32>,

    // Testing.
    pub delete_me: Mutex<String>,

    // The file name of the gedcom file.
    pub file_name: Mutex<PathBuf>,
}



impl Drop for Settings {
    fn drop(&mut self) {
        println!("Drop for Settings object.");
    }
}



impl Settings {
    pub fn new() -> Settings {
        return Settings{count: Mutex::new(0), delete_me: Mutex::new(String::new()), file_name: Mutex::new(PathBuf::new())};
    }

    pub fn load(&mut self) -> bool {
        let mut fred = self.file_name.lock().unwrap();
        *fred = PathBuf::from("/testing/test");
        return true;
    }
}
