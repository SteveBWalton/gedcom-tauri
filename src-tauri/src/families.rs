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



// Return the top families in this gedcom.
#[tauri::command]
pub fn get_top_families(_state: State<Settings> ) -> String {
    let mut families = "".to_string();
    for i in 0..10 {
        families = format!("{}<tr><td><a href=\"family?idx={}\">F{:04}</a></td><td>Family {}</td></tr>", families, i, i+1, i+1);
    }
    return families;
}



// Return a description of the specified family in html.
#[tauri::command]
pub fn get_family_html(family_idx: usize, state: State<Settings> ) -> String {
    let mut html: String = format!("<h1>ID{}</h1>", family_idx);

    // Get the family object.
    let gedcom = state.family_tree.lock().unwrap();
    let family = &gedcom.families[family_idx];

    html = format!("{}<p>idx = '{}'</p>", html, family.idx);

    // Show the original gedcom.
    html += "<div style=\"display: inline-block; vertical-align:top;\">";
    html += "<pre style=\"border: 1px solid black;  background-color: white;\">";
    for line in &family.gedcom {
        html += &line;
        html += "<br/>";
    }
    html += "</pre></div>";

    // Show the current tags.
    html += "<div style=\"display: inline-block; vertical-align:top;\">";
    html += "<pre style=\"border: 1px solid black;  background-color: white;\">";
    for line in &family.tags.to_decorated_html()
    {
        html = format!("{}{}<br/>", html, line);
    }
    html += "</pre></div>";

    return html;
}
