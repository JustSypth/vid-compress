const slider = document.getElementById("slider");
const output = document.getElementById("slider-value");

slider.addEventListener("input", () => {
    output.textContent = slider.value;
});

const advanced = document.getElementById("advanced");
const advanced_box = document.getElementById("advanced_box");
let x = 0;
advanced.addEventListener("click", () => {
    if (x == 0) {
        //open
        advanced.innerHTML = "Advanced <small>▲</small>";
        x = 1;
        advanced_box.style.display = "flex";
    } else {
        //close
        advanced.innerHTML = "Advanced <small>▼</small>";
        advanced_box.style.display = "none";
        x = 0;
    }
});