async function get_path() {
    try {
        const response = await window.__TAURI__.core.invoke('get_path');
        document.getElementById('path_textbox').value = response;
    } catch (error) {
        console.log("Error: ", error);
    }
}
