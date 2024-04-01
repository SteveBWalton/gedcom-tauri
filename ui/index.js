// access the pre-bundled global API functions
const { invoke } = window.__TAURI__.tauri;

// Now we can call our Command.
// `invoke` returns a Promise
invoke('greet', { name: 'from Rust' })
    .then((response) => {
    document.getElementById('message').innerText = response;
    //window.header.innerHTML = response
    }
);

invoke('get_file_name', { })
    .then((response) => {
    document.getElementById('file_name').innerText = response;
    //window.header.innerHTML = response
    }
);



// Code to execute on button click.
function button2Click()
{
    invoke('pick_file', {});
    location.reload();
    return false;
}



