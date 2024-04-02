// System modules.
use tauri::State;

// Application modules.
// mod settings;
use crate::settings;
use settings::Settings;



// Return the number of individuals in this gedcom.
#[tauri::command]
pub fn get_number_families(state: State<Settings> ) -> usize {
    let gedcom = state.family_tree.lock().unwrap();
    return gedcom.families.len()
}
