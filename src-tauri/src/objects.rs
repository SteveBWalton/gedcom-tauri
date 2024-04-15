// System modules.
use tauri::State;

// Application modules.
// mod settings;
use crate::settings;
use settings::Settings;



// Return the top media objects in this gedcom in html for the index page.
#[tauri::command]
pub fn get_top_objects(state: State<Settings> ) -> String {
    let mut html: String = "<table>".to_string();
    // Get the sources collection.
    let gedcom = state.family_tree.lock().unwrap();
    let objects = &gedcom.objects;

    for i in 0..10 {
        html = format!("{}<tr><td><a href=\"object?idx=M{:04}\">M{:04}</a></td><td>Object {}</td></tr>", html, i+1, i+1, i+1);
    }
    html = format!("{}</table>", html);
    html = format!("{}<p>There are {} objects in this gedcom.</p>", html, objects.len());
    return html;
}



// Return a description of the specified media object in html.
#[tauri::command]
pub fn get_object_html(object_idx: &str, state: State<Settings> ) -> String {
    let mut html: String = format!("<h1>{}</h1>", object_idx);

    // Get the media object.
    let gedcom = state.family_tree.lock().unwrap();
    let object = &gedcom.get_object(object_idx);

    // Show the current tags in decorated style.
    html += "<div style=\"display: inline-block; vertical-align:top;\">";
    html += "<pre style=\"border: 1px solid black;  background-color: white;\">";
    for line in &object.tags.to_decorated_html()
    {
        html = format!("{}{}<br/>", html, line);
    }
    html += "</pre></div>";

    return html;
}


