// Update window borders based on OS
let osPromise = window.__TAURI__.core.invoke('get_os');
osPromise.then((os) => {
    if (os == "windows") {
        main.style.borderRadius = 0;

        for (let i = 0; i < overlays.length; i++) {
            overlays[i].style.borderRadius = 0;
        }
    }
});

// Update version value in info overlay
let versionPromise = window.__TAURI__.core.invoke('get_version');
versionPromise.then((version) => {
    console.log("Version: " + version);
    versionText.innerHTML = "v"+version;
});