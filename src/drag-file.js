console.log("Script drag-file.js is running");

async function dragHandler() {
    let path = document.getElementById('path_textbox');

    await window.__TAURI__.webview.getCurrentWebview().onDragDropEvent((event) => {
        switch (event.payload.type) {
            case 'over':
                enable_drag();
                break;
            case 'drop':
                path.value = event.payload.paths;
                disable_drag();
                break
            default:
                disable_drag();
                break
        }
    });
}

function enable_drag() {
    let base = document.getElementById('drag-file');

    base.style.display = "flex";
    base.classList.add('active');
}
function disable_drag() {
    let base = document.getElementById('drag-file');

    base.classList.remove('active');
    base.style.display = "none";
}

dragHandler();
