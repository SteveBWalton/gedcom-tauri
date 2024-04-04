// Access the pre-bundled global API functions
const { invoke } = window.__TAURI__.tauri;

// Now we can call our Command.
// `invoke` returns a Promise
invoke('greet', { name: 'from Rust' })
    .then((response) => {
        document.getElementById('message').innerText = response;
    }
);

invoke('get_file_name', { })
    .then((response) => {
        document.getElementById('file_name').innerText = response;
    }
);

invoke('get_number_individuals', { })
    .then((response) => {
        document.getElementById('individuals_count').innerText = "There are " + response.toString() + " individuals in this gedcom.";
    }
);

invoke('get_top_individuals', { })
    .then((response) => {
        document.getElementById('individuals_table').innerHTML = response;
    }
);

invoke('get_number_families', { })
    .then((response) => {
        document.getElementById('families_count').innerText = "There are " + response.toString() + " families in this gedcom.";
    }
);

invoke('get_top_families', { })
    .then((response) => {
        document.getElementById('families_table').innerHTML = response;
    }
);



const queryString = window.location.search;
console.log(queryString);
const urlParams = new URLSearchParams(queryString);
if (urlParams.has('file'))
{
    const file = urlParams.get('file');
    document.getElementById('parameter').innerText = "The file parameter is '" + file + "'.";
}
else
{
    document.getElementById('parameter').innerText = "The file parameter is missing.";
}





// Code to execute on button click.
function button2Click()
{
    invoke('pick_file', {});
    // This does not work because the function returns long before the file is selected.
    location.reload();
    return false;
}



