async function get_path() {
    try {
        const response = await window.__TAURI__.core.invoke('get_path');
        document.getElementById('path_textbox').value = response;
    } catch (error) {
        console.log("get_path(): Error: ", error);
    }
}

async function begin() {
    var path = document.getElementById('path_textbox');
    var cfg = document.getElementById('slider');
    var preset = document.getElementById('preset');

    console.log("Path: ", path.value);
    console.log("Cfg: ", cfg.value);
    console.log("Preset: ", preset.value);

    try {
        await window.__TAURI__.core.invoke('begin', {path: path.value, cfg: cfg.value, preset: preset.value});
        console.log('begin(): Succesfully called the backend');
    } catch (error) {
        console.error('begin(): Error calling backend:', error);
    }
}

const appWindow = window.__TAURI__.window.getCurrentWindow();
async function app_close() {
    appWindow.close();
}

async function app_minimize() {
    appWindow.minimize();
}

const { listen } = window.__TAURI__.event;
listen('progress', (event) => {
    console.log("Event 'progress' triggered");
    const progressbar = document.getElementById('progress');
    progressbar.style.display = "block";
    progressbar.innerHTML = event.payload;
});