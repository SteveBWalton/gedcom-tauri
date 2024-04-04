// Access the pre-bundled global API functions
const { invoke } = window.__TAURI__.tauri;

// Get the page parameters.
const queryString = window.location.search;
console.log(queryString);
const urlParams = new URLSearchParams(queryString);
if (urlParams.has('idx'))
{
    const idx = urlParams.get('idx');
    document.getElementById('parameter').innerText = "The index is '" + idx + "'.";

    document.getElementById('description').innerHTML = "<p>Hello World</p>";
    invoke('get_family_html', { familyIdx: parseInt(idx) })
    // invoke('get_individual_html', { })
        .then((response) => {
            document.getElementById('description').innerHTML = response;
        }
    );
}
else
{
    document.getElementById('parameter').innerText = "The index is missing.";
}



