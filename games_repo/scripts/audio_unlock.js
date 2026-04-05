// Capture AudioContext to resume it on iOS Safari
let captured_ctx = null;
(function() {
    const OriginalAudioContext = window.AudioContext || window.webkitAudioContext;
    if (OriginalAudioContext) {
        const NewAudioContext = function() {
            const ctx = new OriginalAudioContext();
            captured_ctx = ctx;
            return ctx;
        };
        NewAudioContext.prototype = OriginalAudioContext.prototype;
        if (window.AudioContext) window.AudioContext = NewAudioContext;
        if (window.webkitAudioContext) window.webkitAudioContext = NewAudioContext;
    }
})();

function resumeAudio() {
    if (captured_ctx) {
        if (captured_ctx.state === 'suspended') {
            captured_ctx.resume().then(() => {
                console.log("AudioContext resumed successfully");
            });
        }
        // Force unlock with a silent oscillator
        try {
            const osc = captured_ctx.createOscillator();
            const silent = captured_ctx.createGain();
            silent.gain.value = 0;
            osc.connect(silent);
            silent.connect(captured_ctx.destination);
            osc.start(0);
            osc.stop(0.1);
        } catch (e) {
            console.error("Error playing silent sound:", e);
        }
    }
}

// Add listeners for any subsequent interaction to ensure audio is unlocked
window.addEventListener('click', resumeAudio, true);
window.addEventListener('touchstart', resumeAudio, true);
window.addEventListener('touchend', resumeAudio, true);
