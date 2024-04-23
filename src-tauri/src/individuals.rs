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
    let gedcom = state.family_tree.lock().unwrap();
    let mut html = "".to_string();
    for i in 0..10 {
        let individual = &gedcom.individuals[i];
        html = format!("{}<tr><td><a href=\"individual.html?idx={}\">{}</a></td><td><a href=\"individual.html?idx={}\">{}</a></td></tr>", html, individual.idx, individual.idx, individual.idx, individual.get_name().unwrap());
    }
    return html;

}

// Returns a title for the specified individual.
#[tauri::command]
pub fn get_individual_title(individual_idx: &str, state: State<Settings> ) -> String {
// Get the individual.
    let gedcom = state.family_tree.lock().unwrap();
    // let individual = &gedcom.individuals[individual_idx];
    let individual = &gedcom.get_individual(individual_idx);

    match individual.tags.find_one("NAME") {
        Some(tag) => {
            String::from(&tag.value)
        }
        None => {
            "NAME tag not found.".to_string()
        }
    }
}



// Return a description of the specified individual in html.
#[tauri::command]
pub fn get_individual_html(individual_idx: &str, state: State<Settings> ) -> String {
    let mut html: String = format!("<h1>{}</h1>", individual_idx);

    // Get the individual.
    let gedcom = state.family_tree.lock().unwrap();
    // let individual = &gedcom.individuals[individual_idx];
    let individual = &gedcom.get_individual(individual_idx);

    // Show the original gedcom.
    html += "<div style=\"display: inline-block; vertical-align:top;\">";
    html += "<pre style=\"border: 1px solid black;  background-color: white;\">";
    for line in &individual.gedcom {
        html += &line;
        html += "<br/>";
    }
    html += "</pre></div>";

    // Show the current tags in gedcom format.
    html += "<div style=\"display: inline-block; vertical-align:top;\">";
    html += "<pre style=\"border: 1px solid black;  background-color: white;\">";
    for line in &individual.tags.to_gedcom_file()
    {
        html = format!("{}{}<br/>", html, line);
    }
    html += "</pre></div>";

    // Show the current tags.
    html += "<div style=\"display: inline-block; vertical-align:top;\">";
    html += "<pre style=\"border: 1px solid black;  background-color: white;\">";
    for line in &individual.tags.to_decorated_html()
    {
        html = format!("{}{}<br/>", html, line);
    }
    html += "</pre></div>";

    return html;
}

