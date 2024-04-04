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
    let mut html = "".to_string();
    for i in 0..10 {
        html = format!("{}<tr><td><a href=\"individual.html?idx={}\">I{:04}</a></td><td>Individual {}</td></tr>", html, i, i+1, i+1);
    }
    return html;

}


// Return a description of the specified individual in html.
#[tauri::command]
pub fn get_individual_html(individual_idx: usize, state: State<Settings> ) -> String {
    let mut html: String = format!("<h1>ID{}</h1>", individual_idx);

    // Show the original gedcom.
    html += "<div style=\"display: inline-block; vertical-align:top;\">";
    html += "<pre style=\"border: 1px solid black;  background-color: white;\">";
    let gedcom = state.family_tree.lock().unwrap();
    let individual = &gedcom.individuals[individual_idx];
    for line in &individual.gedcom {
        html += &line;
        html += "<br/>";
    }
    html += "</pre></div>";

    // Show the current tags.
    html += "<div style=\"display: inline-block; vertical-align:top;\">";
    html += "<pre style=\"border: 1px solid black;  background-color: white;\">";
    for tag in &individual.tags.tags {
        html = format!("{}{} '{}' '{}'<br/>", html, tag.level, tag.key, tag.value);
        if tag.tags.tags.len() > 0
        {
            for child_tag in &tag.tags.tags
            {
                html = format!("{}    {} '{}' '{}'<br/>", html, child_tag.level, child_tag.key, child_tag.value);
            }
        }
    }
    html += "</pre></div>";

    return html;
}

