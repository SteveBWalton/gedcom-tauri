// Modules to support individuals for the gedcom-tauri application.

// System modules.
use tauri::State;

// Application modules.
// mod settings;
use crate::settings;
use settings::Settings;

// Return the number of individuals in this gedcom.
#[tauri::command]
pub fn get_number_individuals(state: State<Settings> ) -> usize {
    let gedcom = state.family_tree.lock().unwrap();
    return gedcom.individuals.len()
}


// Return the top individuals in this gedcom.
#[tauri::command]
pub fn get_top_individuals(state: State<Settings> ) -> String {
    return "<tr><td><a href=\"individual.html?idx=0\">I0001</a></td><td>Person One</td></tr><tr><td><a href=\"individual.html?idx=1\">I0002</a></td><td>Person Two</td></tr>".to_string();
}


