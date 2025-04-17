const { listen } = window.__TAURI__.event;
const { emit } = window.__TAURI__.event;

let go_btn = document.getElementById('go');

let processing = false;

listen('VID_EXISTS', async () => {
    let confirmed = await get_confirm("Already compressed", "Do you want to recompress the video?");
    
    console.log(confirmed);

    emit('RESPONSE_IGNORE_EXISTING', confirmed);
});


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
        ignore_existing = false;
        close_confirmation();
    }
});

listen('STATUS', (event) => {
    const progressbar = document.getElementById('progress');
    progressbar.style.display = "block";
    progressbar.innerHTML = event.payload;
});

let state = "start";
let debounce_stop = false;
async function go() {
    console.log(state);
    if (state === "start") {
        if (processing) {return;}
        state = "stop";
        go_btn.innerHTML = "Stop";
        console.log("Set to stop");

        let path = document.getElementById('path_textbox');
        let crf = document.getElementById('slider');
        let preset = document.getElementById('preset');
        let audio = document.getElementById('audio-bitrate'); 
        let hevc = document.getElementById('hevc');
    

        console.log(audio.value);
        window.__TAURI__.core.invoke('begin', {path: path.value, crf: crf.value, preset: preset.value, audio: audio.value, hevc: hevc.checked});

        return;
    }
    if (state === "stop") {
        if (debounce_stop) {return;}
        debounce_stop = true;

        const confirmed = await get_confirm("Are you sure to cancel the compression?", "The video is still being compressed."); 
        if (confirmed) {
            console.log("STOP CONFIRMATION: Pressed yes!")
            await window.__TAURI__.core.invoke('stop');
        } else if (!confirmed) {
            console.log("STOP CONFIRMATION: Pressed no!")
            debounce_stop = false;
            return;
        }

        state = "start";
        go_btn.innerHTML = "Start";
        console.log("Set to start");
        debounce_stop = false;
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