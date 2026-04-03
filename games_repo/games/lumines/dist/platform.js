/**
 * Shared platform detection utilities for WASM games.
 */

window.GamePlatform = {
    /**
     * Detects if the current device is running iOS or iPadOS.
     * This is crucial for handling AudioContext initialization which requires
     * a user interaction on these platforms.
     */
    isIOS: function() {
        return /iPad|iPhone|iPod/.test(navigator.userAgent) || 
               (navigator.platform === 'MacIntel' && navigator.maxTouchPoints > 1);
    }
};
