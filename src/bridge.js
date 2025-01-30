function close_window() {
    const invoke = window.__TAURI__.core.invoke;
    invoke('close');
}
