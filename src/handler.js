const { listen } = window.__TAURI__.event;

let go_btn = document.getElementById('go');

let processing = false;
listen('PROCESSING', (event) => {
    processing = event.payload === "true";

    if (processing) {
        console.log("Began compressing..");
    }
    if (!processing) {
        console.log("Stopped compressing..");
        state = "start";
        go_btn.innerHTML = "Start";
        console.log("Set to start");
        close_confirmation();
    }
});

listen('STATUS', (event) => {
    const progressbar = document.getElementById('progress');
    progressbar.style.display = "block";
    progressbar.innerHTML = event.payload;
});

let state = "start";
let debounce_go = false;
async function go() {
    console.log(state);
    if (state === "start") {
        if (processing) {return;}
        state = "stop";
        go_btn.innerHTML = "Stop";
        console.log("Set to stop");

        var path = document.getElementById('path_textbox');
        var crf = document.getElementById('slider');
        var preset = document.getElementById('preset');
    
        var hevc = document.getElementById('hevc');
    
        window.__TAURI__.core.invoke('begin', {path: path.value, crf: crf.value, preset: preset.value, hevc: hevc.checked});

        return;
    }
    if (state === "stop") {
        if (debounce_go) {return;}
        state = "start";
        go_btn.innerHTML = "Start";
        console.log("Set to start");

        debounce_go = true;

        await window.__TAURI__.core.invoke('stop');

        debounce_go = false;
        return;
    }
}

async function app_close() {
    if (processing) {
        const confirmed = await get_confirm(); 
        if (confirmed) {
            appWindow.close()
        }
        return;
    }
    appWindow.close();
}

async function app_minimize() {
    appWindow.minimize();
}