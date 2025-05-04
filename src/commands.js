async function open_file() {
    try {
        const response = await window.__TAURI__.core.invoke('open_file');
        console.log(response);
        let path = document.getElementById('path_textbox');
        path.value = response;
    } catch (error) {
        console.log("get_path(): Error: ", error);
    }
}

let debounce_info = false;
async function toggle_info() {
    let base = document.getElementById('info-base');
    let overlay = document.getElementById('info');
    let isOpen = overlay.classList.contains('active');
    
    if (debounce_info) { console.log("debounce_info"); return }
    debounce_info = true;

    if (isOpen) {
        overlay.addEventListener('transitionend', () => {
            overlay.style.display = "none";
            base.style.display = "none";
            debounce_info = false;
        }, { once: true });

        overlay.classList.remove('active');
        base.classList.remove('active');
    } else {
        base.style.display = "flex";
        overlay.style.display = "flex";
        
        void overlay.offsetHeight;

        base.classList.add('active');
        overlay.classList.add('active');
        
        debounce_info = false;
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