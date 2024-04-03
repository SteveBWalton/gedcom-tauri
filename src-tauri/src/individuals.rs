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


// Return a description of the specified individual in html.
#[tauri::command]
// pub fn get_individual_html(state: State<Settings> ) -> String {
//    let individual_idx = 0;
pub fn get_individual_html(individual_idx: usize, state: State<Settings> ) -> String {
    // let mut html: String = "".to_string();
    // html = format!("<h1>ID{}</h1>", individual_idx);
    let mut html: String = format!("<h1>ID{}</h1>", individual_idx);

    html += "<div style=\"display: inline-block; vertical-align:top;\">";
    html += "<pre style=\"border: 1px solid black;  background-color: white;\">";

    let gedcom = state.family_tree.lock().unwrap();
    let individual = &gedcom.individuals[individual_idx];
    for line in &individual.gedcom {
        // html += format!("{}<br/>", line);
        html += &line;
        html += "<br/>";
    }
    html += "</pre></div>";

    return html;
}

