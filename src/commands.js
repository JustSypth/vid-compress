const appWindow = window.__TAURI__.window.getCurrentWindow();
const { listen } = window.__TAURI__.event;

let processing = false;
listen('PROCESSING', (event) => {
    console.log("Event ", event);
    processing = event.payload === "true";
});

listen('STATUS', (event) => {
    const progressbar = document.getElementById('progress');
    progressbar.style.display = "block";
    progressbar.innerHTML = event.payload;
});

async function get_path() {
    try {
        const response = await window.__TAURI__.core.invoke('get_path');
        console.log(response);
        let path = document.getElementById('path_textbox');
        path.value = response;
    } catch (error) {
        console.log("get_path(): Error: ", error);
    }
}

async function begin() {
    if (processing) {return;}
    var path = document.getElementById('path_textbox');
    var cfg = document.getElementById('slider');
    var preset = document.getElementById('preset');

    // console.log("Path: ", path.value);
    // console.log("Cfg: ", cfg.value);
    // console.log("Preset: ", preset.value);

    try {
        await window.__TAURI__.core.invoke('begin', {path: path.value, cfg: cfg.value, preset: preset.value});
        console.log('begin(): Succesfully called the backend');
    } catch (error) {
        console.error('begin(): Error calling backend:', error);
    }
}

async function app_close() {
    if (processing) {
        console.log("Pressed X");
        var overlay = document.getElementById('confirmation');
        let isOpen = overlay.classList.contains('active');

        if (isOpen) {
            overlay.addEventListener('transitionend', () => {
                overlay.style.display = "none";
            }, { once: true });
    
            overlay.classList.remove('active');
        } else {
            overlay.style.display = "flex";
            void overlay.offsetHeight;
            overlay.classList.add('active');
            
            // Close about overlay if its on
            let about = document.getElementById('info');
            if (about.classList.contains('active')) {
                about.addEventListener('transitionend', () => {
                    about.style.display = "none";
                }, { once: true });
        
                about.classList.remove('active');
            }
        }
        return;
    }
    appWindow.close();
}

async function app_minimize() {
    appWindow.minimize();
}

let debounce = false;
async function toggle_info() {
    let overlay = document.getElementById('info');
    let isOpen = overlay.classList.contains('active');
    
    if (debounce) { console.log("debounce"); return }
    debounce = true;

    if (isOpen) {
        overlay.addEventListener('transitionend', () => {
            overlay.style.display = "none";
            debounce = false;
        }, { once: true });

        overlay.classList.remove('active');
    } else {
        overlay.style.display = "flex";
        void overlay.offsetHeight;
        overlay.classList.add('active');
        debounce = false;
    }
}
async function open_sypth() {
    console.log("Sypth.xyz opening..");
    let url = "https://sypth.xyz/";
    window.__TAURI__.core.invoke("open_url", {url});
}
async function open_github() {
    console.log("Github opening..");
    let url = "https://github.com/JustSypth/vid-compress";
    window.__TAURI__.core.invoke("open_url", {url});
}

async function toggle_documentation() {
    let parent = document.getElementById('info');
    let overlay = document.getElementById('documentation');
    let isOpen = overlay.classList.contains('active');

    if (isOpen) {
        overlay.addEventListener('transitionend', () => {
            overlay.style.display = "none";
        }, { once: true });

        overlay.classList.remove('active');
    } else {
        overlay.style.display = "flex";
        void overlay.offsetHeight;
        overlay.classList.add('active');

        overlay.addEventListener('transitionend', () => {
            console.log("Transition end");
            toggle_info();
        }, { once: true });
    }
}