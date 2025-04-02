const { listen } = window.__TAURI__.event;

let begin_btn = document.getElementById('begin');

let processing = false;
listen('PROCESSING', (event) => {
    processing = event.payload === "true";

    if (processing) {
        console.log("Began compressing..");
    }
    if (!processing) {
        console.log("Stopped compressing..");
        state = "start";
        begin_btn.innerHTML = "Start";
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
let debounce_begin = false;
async function begin() {
    console.log(state);
    if (state === "start") {
        if (processing) {return;}
        state = "stop";
        begin_btn.innerHTML = "Stop";
        console.log("Set to stop");

        var path = document.getElementById('path_textbox');
        var crf = document.getElementById('slider');
        var preset = document.getElementById('preset');
    
        var hevc = document.getElementById('hevc');
    
        try {
            window.__TAURI__.core.invoke('begin', {path: path.value, crf: crf.value, preset: preset.value, hevc: hevc.checked});
            console.log('Successfully called the backend');
        } catch (error) {
            console.error('Error calling backend: ', error);
        }

        return;
    }
    if (state === "stop") {
        if (debounce_begin) {return;}
        state = "start";
        begin_btn.innerHTML = "Start";
        console.log("Set to start");

        debounce_begin = true;

        await window.__TAURI__.core.invoke('stop');

        debounce_begin = false;
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