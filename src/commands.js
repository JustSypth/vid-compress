const appWindow = window.__TAURI__.window.getCurrentWindow();

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

const { listen } = window.__TAURI__.event;
listen('progress', (event) => {
    const progressbar = document.getElementById('progress');
    progressbar.style.display = "block";
    progressbar.innerHTML = event.payload;
});

async function app_minimize() {
    appWindow.minimize();
}

let processing = false;
listen('confirmation', (event) => {
    console.log("Event ", event);
    processing = event.payload === "true";
});

var overlay = document.getElementById('confirmation');
const confirmYes = document.getElementById('confirm-yes');
const confirmNo = document.getElementById('confirm-no');

confirmYes.addEventListener('click', () => {
    console.log("Button yes clicked")
    appWindow.close();
  });
  
confirmNo.addEventListener('click', () => {
    overlay.style.display = "none";
});

async function app_close() {
    if (confirmation) {
        overlay.style.display = "flex";
    } else {
        appWindow.close();
    }
}