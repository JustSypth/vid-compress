const appWindow = window.__TAURI__.window.getCurrentWindow();
const { listen } = window.__TAURI__.event;

let processing = false;
listen('PROCESSING', (event) => {
    console.log("Began compressing..");
    processing = event.payload === "true";

    if (processing === false) {
        console.log("Stopped compressing..");
        close_confirmation();
    }
});

listen('STATUS', (event) => {
    const progressbar = document.getElementById('progress');
    progressbar.style.display = "block";
    progressbar.innerHTML = event.payload;
});

async function begin() {
    if (processing) {return;}
    var path = document.getElementById('path_textbox');
    var crf = document.getElementById('slider');
    var preset = document.getElementById('preset');

    var hevc = document.getElementById('hevc');

    try {
        await window.__TAURI__.core.invoke('begin', {path: path.value, crf: crf.value, preset: preset.value, hevc: hevc.checked});
        console.log('begin(): Succesfully called the backend');
    } catch (error) {
        console.error('begin(): Error calling backend:', error);
    }
}

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

async function app_close() {
    if (processing) {
        var base = document.getElementById('confirmation-base');
        var overlay = document.getElementById('confirmation');
        var confirmYes = document.getElementById('confirm-yes');
        var confirmNo = document.getElementById('confirm-no');
        let isOpen = overlay.classList.contains('active');

        if (isOpen) {
            overlay.addEventListener('transitionend', () => {
                overlay.style.display = "none";
                base.style.display = "none";
            }, { once: true });
            
            overlay.classList.remove('active');
            base.classList.remove('active');
        } else {
            base.style.display = "flex";
            overlay.style.display = "flex";
            void overlay.offsetHeight;
            base.classList.add('active');
            overlay.classList.add('active');

            confirmYes.addEventListener('click', () => {
                appWindow.close()
            });
            confirmNo.addEventListener('click', () => {
                var overlay = document.getElementById('confirmation');
            
                overlay.addEventListener('transitionend', () => {
                    overlay.style.display = "none";
                    base.style.display = "none";
                }, { once: true });
            
                overlay.classList.remove('active');
                base.classList.remove('active');
            });
        }
        return;
    }
    appWindow.close();
}
async function close_confirmation() {
    var base = document.getElementById('confirmation-base');
    var overlay = document.getElementById('confirmation');
    let isOpen = overlay.classList.contains('active');

    if (isOpen) {
        overlay.addEventListener('transitionend', () => {
            overlay.style.display = "none";
            base.style.display = "none";
        }, { once: true });
        
        overlay.classList.remove('active');
        base.classList.remove('active');
    }
}

async function app_minimize() {
    appWindow.minimize();
}

let debounce = false;
async function toggle_info() {
    let base = document.getElementById('info-base');
    let overlay = document.getElementById('info');
    let isOpen = overlay.classList.contains('active');
    
    if (debounce) { console.log("debounce"); return }
    debounce = true;

    if (isOpen) {
        overlay.addEventListener('transitionend', () => {
            overlay.style.display = "none";
            base.style.display = "none";
            debounce = false;
        }, { once: true });

        overlay.classList.remove('active');
        base.classList.remove('active');
    } else {
        base.style.display = "flex";
        overlay.style.display = "flex";
        
        void overlay.offsetHeight;

        base.classList.add('active');
        overlay.classList.add('active');
        
        debounce = false;
    }
}
document.getElementById('info-base').addEventListener('click', function(e) {
    if (e.target === this) {
        toggle_info();
    }
});

async function open_sypth() {
    console.log("Sypth.xyz opening..");
    let url = "https://sypth.xyz/";
    window.__TAURI__.core.invoke("open_url", {url});
    toggle_info();
}
async function open_github() {
    console.log("Github opening..");
    let url = "https://github.com/JustSypth/vid-compress";
    window.__TAURI__.core.invoke("open_url", {url});
    toggle_info();
}

async function toggle_documentation() {
    let base = document.getElementById('documentation-base');
    let overlay = document.getElementById('documentation');
    let isOpen = overlay.classList.contains('active');

    if (isOpen) {
        overlay.addEventListener('transitionend', () => {
            overlay.style.display = "none";
            base.style.display = "none";
        }, { once: true });

        overlay.classList.remove('active');
        base.classList.remove('active');
    } else {
        base.style.display = "flex";
        overlay.style.display = "flex";

        void overlay.offsetHeight;
        
        overlay.classList.add('active');

        overlay.addEventListener('transitionend', () => {
            console.log("Transition end");
            toggle_info();
            base.classList.add('active');
        }, { once: true });
    }
}
document.getElementById('documentation-base').addEventListener('click', function(e) {
    if (e.target === this) {
        toggle_documentation();
    }
});