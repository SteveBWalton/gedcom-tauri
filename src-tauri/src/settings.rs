// Modules to hold the settings for the Tauri front end.

use std::path::PathBuf;
use std::sync::Mutex;
use std::fs;
use std::fs::File;
use std::io::Write;     // write_all
use homedir::get_my_home;




// Member variables for the 'GedComDoc' class.
pub struct Settings {
    // This is a count variable just for testing.
    pub count: Mutex<i32>,

    // Testing.
    pub delete_me: Mutex<String>,

    // The file name of the gedcom file.
    pub file_name: Mutex<PathBuf>,
}


// Destructor for Settings.
impl Drop for Settings {
    fn drop(&mut self) {
        println!("Drop for Settings object.");
    }
}



impl Settings {
    // Iniitalise the settings.
    pub fn new() -> Settings {
        return Settings{count: Mutex::new(0), delete_me: Mutex::new(String::new()), file_name: Mutex::new(PathBuf::new())};
    }



    // Load the settings.  Not Implemented.
    pub fn load(&mut self) -> bool {
        // Get the settings file.
        let mut settings_file_name = get_my_home().unwrap().unwrap();
        settings_file_name.push(".walton");
        settings_file_name.push("gedcom-tauri");
        settings_file_name.push("gedcom-tauri.txt");

        // Read from the settings file.
        let data = fs::read_to_string(settings_file_name).expect("Unable to read settings file.");
        println!("Loaded file name as {}", data);

        // Set the file name path setting.
        let mut file_name = self.file_name.lock().unwrap();
        *file_name = PathBuf::from(data);
        // *file_name = PathBuf::from("/testing/test");

        // Return success.
        return true;
    }



    // Save the settings to a file.
    pub fn save(&self) -> bool {
        // Get the settings file.
        let mut settings_file_name = get_my_home().unwrap().unwrap();
        settings_file_name.push(".walton");
        settings_file_name.push("gedcom-tauri");
        settings_file_name.push("gedcom-tauri.txt");
        // println!("Writing gedcom file name to {}", settings_file_name.display());
        let mut output = File::create(settings_file_name).expect("Unable to create file.");
        let file_name = self.file_name.lock().unwrap();
        let file_name_string = (*file_name).to_string_lossy();
        // println!("Writing gedcom file as {}", file_name_string);
        // output.write_all(file_name_string.as_bytes()).expect("Unable to write data.");
        writeln!(output, "{}", file_name_string).expect("Unable to write file name to settings file.");
        output.write_all("\n".as_bytes()).expect("Unable to write data.");

        return true;
    }
}
