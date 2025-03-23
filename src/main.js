const main = document.getElementById("main");
const slider = document.getElementById("slider");
const output = document.getElementById("slider-value");
const advanced = document.getElementById("advanced");
const advancedBox = document.getElementById("advanced_box");
const versionText = document.getElementById('version');

// Update window borders based on OS
let osPromise = window.__TAURI__.core.invoke('get_os');
osPromise.then((os) => {
    if (os == "windows") {
        main.style.borderRadius = 0;
    }
});

// Update version value in info overlay
let versionPromise = window.__TAURI__.core.invoke('get_version');
versionPromise.then((version) => {
    console.log("Version: " + version);
    versionText.innerHTML = "v"+version;
});

// Update slider value display
slider.addEventListener("input", () => {
    output.textContent = slider.value;
});

// Toggle advanced box visibility
advanced.addEventListener("click", () => {
    const isOpen = advancedBox.style.display === "flex";
    
    advancedBox.style.display = isOpen ? "none" : "flex";
    advanced.innerHTML = `Advanced <small>${isOpen ? "▼" : "▲"}</small>`;
});
