// ==UserScript==
// @name          Farkle
// @namespace     http://www.example.com/
// @description   WASM test
// @include       *
// @require       file:C:\Users\user\git\farkle\userscript\farkle.js
// @require       file:C:\Users\user\git\farkle\userscript\userscript.js
// ==/UserScript==

(() => {
const log = console.log;
const SERVER = 'http://127.0.0.1:8080';

class Runner {
    async run() {
        await loadWasm();
        await wasm_bindgen(`${SERVER}/farkle_bg.wasm`);
    
        new window.Farkle(wasm_bindgen).run();
    }
}


// WASM files
async function loadWasm() {
    let attempts = 0;
    includeJs(`${SERVER}/farkle.js`);
    while (typeof wasm_bindgen == 'undefined') {
        attempts += 1;
        if (attempts === 5) {
            log(`warning: failed to load wasm_bindgen after ${attempts} attempts`)
        }
        await sleep(200);
    }
    log(`Loaded after ${attempts} attempt/s`);
}

function includeJs(jsFilePath) {
    const js = document.createElement("script");
    js.type = "text/javascript";
    js.src = jsFilePath;
    document.body.appendChild(js);
}

new Runner().run();
})();

// MISC
function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms))
}

