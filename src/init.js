const main = document.getElementById("main");
const slider = document.getElementById("slider");
const output = document.getElementById("slider-value");
const advanced = document.getElementById("advanced");
const advancedBox = document.getElementById("advanced_box");
const hevc = document.getElementById('hevc-div');

const versionText = document.getElementById('version');

let overlays = document.getElementsByClassName("overlay");
let overlays_array = Array.from(overlays)

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

hevc.addEventListener("click", () => {
    let hevc_checkbox = document.getElementById('hevc');
    hevc_checkbox.checked = !hevc_checkbox.checked;
});