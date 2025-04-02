const appWindow = window.__TAURI__.window.getCurrentWindow();

function get_confirm() {
    return new Promise((resolve) => {
        let base = document.getElementById('confirmation-base');
        let overlay = document.getElementById('confirmation');
        let confirmYes = document.getElementById('confirm-yes');
        let confirmNo = document.getElementById('confirm-no');
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
                resolve(true);
            });
            confirmNo.addEventListener('click', () => {
                var overlay = document.getElementById('confirmation');
            
                overlay.addEventListener('transitionend', () => {
                    overlay.style.display = "none";
                    base.style.display = "none";
                }, { once: true });
            
                overlay.classList.remove('active');
                base.classList.remove('active');
    
                resolve(false);
            });
        }
    });
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