// System modules.
use tauri::State;

// Application modules.
// mod settings;
use crate::settings;
use settings::Settings;



// Return the top sources in this gedcom in html for the index page.
#[tauri::command]
pub fn get_top_sources(state: State<Settings> ) -> String {
    let mut html: String = "<table>".to_string();
    // Get the sources collection.
    let gedcom = state.family_tree.lock().unwrap();
    let sources = &gedcom.sources;

    for i in 0..10 {
        html = format!("{}<tr><td><a href=\"source?idx={}\">S{:04}</a></td><td>Source {}</td></tr>", html, i, i+1, i+1);
    }
    html = format!("{}</table>", html);
    html = format!("{}<p>There are {} sources in this gedcom.</p>", html, sources.len());
    return html;
}

