import { FarkleScript } from './farkleScript';

const log = console.log;
const SERVER = 'http://127.0.0.1:8080';

export class Runner {
    async run() {
        await loadWasm();
        await wasm_bindgen(`${SERVER}/farkle_wasm_bg.wasm`);
    
        new FarkleScript(wasm_bindgen, SERVER).run();
    }
}


// WASM files
async function loadWasm() {
    let attempts = 0;
    includeJs(`${SERVER}/farkle_wasm.js`);
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

// MISC
export function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms))
}

