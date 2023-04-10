// ==UserScript==
// @name          Farkle
// @match         https://cardgames.io/farkle*
// ==/UserScript==
/******/ (() => { // webpackBootstrap
/******/ 	"use strict";
var __webpack_exports__ = {};

;// CONCATENATED MODULE: ./userscript/probBar.js

class ProbBar {
    constructor() {
        const div = document.createElement('div');
        div.id = 'scorecard-column'
        div.innerHTML = `
            <table id='scorecard'>
                <tbody>
                    <tr>
                        <td>Turn number</td>
                        <td class='turnNumber'>0</td>
                    </tr>
                    <tr>
                        <td>Win prob.</td>
                        <td style='position: relative'>
                            <span class='winChance' style='position: relative; z-index: 3'>50%</span>
                            <span class='winChanceBar' style='background:lightgreen; position: absolute; left:0; top:0; bottom:0; height:100%; transition: width 400ms ease-in-out; z-index: 2'></span>
                        </td>
                    </tr>
                </tbody>
            </table>`;
        $("#player-scores div:first").after(div);
    }

    setTurnNumber(turnNumber) {
        $("td.turnNumber").text(turnNumber);
    }

    setWinProbability(prob) {
        const winChanceText = `${(100 * prob).toFixed(2)}%`;
        $("span.winChance").text(winChanceText);
        $("span.winChanceBar")[0].style.width = `${prob*100}%`;
    }
}
;// CONCATENATED MODULE: ./userscript/farkleScript.js


const log = console.log;
const localStorageKey = 'farkleSolver.results';

class FarkleScript {
    constructor(wasm_bindgen, SERVER) {
        const { set_panic_hook, FarkleSolverWasm, populate_solver } = wasm_bindgen;
        set_panic_hook();
        this.probBar = new ProbBar();
        this.farkleSolver = new FarkleSolverWasm();
        this.populate_solver = populate_solver;
        this.dicePositions = {};
        this.turnNumber = 0;
        this.SERVER = SERVER;
        for (let id of this._getDiceIds()) this.dicePositions[id] = '';
    }

    async run() {
        console.log('w/ record', this._getLocalStorage());

        console.time('populate');
        await this.populate_solver(`${this.SERVER}/exact.bincode`, this.farkleSolver);
        console.timeEnd('populate');

        console.time(`first action`)
        this.farkleSolver.decide_action_ext(0, 6, [0, 0]);
        console.timeEnd('first action');

        while (true) {
            await this._waitForYourTurn();
            if (this._getStartNewGameButton() != null) {
                this._recordWinnerAndRefresh();
                break;
            }
            await this._doAction();
            await this._waitForStationaryDice();
        }
    }

    _getScores = () => [
        parseInt($('#bottom-player-total').text()),
        parseInt($('#top-player-total').text())
    ];

    async _doAction() {
        let heldScore = 0;
        let totalScores = this._getScores();

        let diceLeft = 6;
        const [initialProb, _] = this.farkleSolver.decide_action_ext(heldScore, diceLeft, totalScores);
        this.probBar.setTurnNumber(++this.turnNumber);
        this.probBar.setWinProbability(initialProb);

        while (true) {
            log(`decide_action_ext(held_score: ${heldScore}, dice_left: ${diceLeft}, scores: [${totalScores}])...`);
            const [probAction, action] = this.farkleSolver.decide_action_ext(heldScore, diceLeft, totalScores);
            log(probAction, `action: ${action}`);
            this.probBar.setWinProbability(probAction);
            if (action === 'Stay') {
                this._getBankButton().click();
                break;
            }

            this._getRollButton().click();
            await this._waitForStationaryDice();
            let [diceInPlayEls, diceInPlay] = this._getDiceInPlay();
            const roll = diceInPlay.join('').toString();
            log(`decide_held_dice_ext(held_score: ${heldScore}, roll: ${roll}, scores: [${totalScores}])...`);
            const [probHeldDice, diceToHoldScore, diceToHold] = this.farkleSolver.decide_held_dice_ext(heldScore, roll, totalScores);
            log(probHeldDice, `hold: ${diceToHold}`);
            this.probBar.setWinProbability(probHeldDice);
            if (diceToHoldScore === 0) {
                break;
            }
            
            await this._clickDice(diceToHold);
            await this._waitForStationaryDice();
            heldScore += diceToHoldScore;
            diceLeft -= diceToHold.length;
            if (diceLeft === 0) {
                diceLeft = 6;
            }
        }
    }

    _recordWinnerAndRefresh() {
        const winLossRecord = this._getLocalStorage();
        const scores = this._getScores();
        if (scores[0] > scores[1]) {
            winLossRecord.wins++;
        } else {
            winLossRecord.loss++;
        }
        localStorage[localStorageKey] = JSON.stringify(winLossRecord);
        console.log(winLossRecord);
        this._getStartNewGameButton().click();
    }
    _getLocalStorage() {
        try {
            return JSON.parse(localStorage[localStorageKey]);
        } catch {
            return {wins: 0, loss: 0};
        }
    }
    
    async _clickDice(diceToHold) {
        let [diceInPlayEls] = this._getDiceInPlay();
        let elsToClick = [];
        let j = 0;
        for (let el of diceInPlayEls) {
            if (j < diceToHold.length && el.alt === diceToHold[j]) {
                elsToClick.push(el);
                j += 1;
            }
        }
        for (let el of elsToClick) {
            el.click();
            await sleep(350);
        }
    }
    _getHeldScore = () => parseInt($('#bottom-player-round-score b').text());
    _getDiceInPlay = () => {
        const diceInPlayEls = Array.from($('.dice'))
            .map(el => el)
            .filter(el => !el.alt.includes('saved'))
            .sort((a, b) => parseInt(a.alt) - parseInt(b.alt));
        const diceInPlay = diceInPlayEls.map(t => parseInt(t.alt));
        return [diceInPlayEls, diceInPlay];
    }

    _getRollButton = () => $("#throw-button button")[0];
    _getBankButton = () => $("#bank-button button")[0];
    _getStartNewGameButton = () => $("#result-box[style='display: block;'] button#start-new-game").toArray().at(0)

    async _waitForYourTurn() {
        //log('waiting for turn...');
        while (true) {
            if (!this._getRollButton().disabled
                || !this._getBankButton().disabled
                || this._getStartNewGameButton() != null) break;
            await sleep(100);
        }
        //log('your turn');
    }

    async _waitForStationaryDice() {
        while (true) {
            await sleep(100);
            let changed = false;
            for (let id of this._getDiceIds()) {
                const dicePosition = $(`#${id}`)[0].getAttribute('style')
                if (dicePosition !== this.dicePositions[id]) {
                    changed = true;
                }
                this.dicePositions[id] = dicePosition;
            }
            if (!changed) break;
        }
    }

    _getDiceIds() {
        return [1,2,3,4,5,6].map(id => `d${id}`);
    }

    _getText() {
        return $('#messageBox').text();
    }
}
;// CONCATENATED MODULE: ./userscript/runner.js


const runner_log = console.log;

class Runner {
    async run() {
        await loadWasm();
        await wasm_bindgen(`${"https://github.com/domsleee/farkle/raw/gh-pages"}/farkle_wasm_bg.wasm`);
    
        new FarkleScript(wasm_bindgen, "https://github.com/domsleee/farkle/raw/gh-pages").run();
    }
}


// WASM files
async function loadWasm() {
    let attempts = 0;
    includeJs(`${"https://github.com/domsleee/farkle/raw/gh-pages"}/farkle_wasm.js`);
    while (typeof wasm_bindgen == 'undefined') {
        attempts += 1;
        if (attempts === 5) {
            runner_log(`warning: failed to load wasm_bindgen after ${attempts} attempts`)
        }
        await sleep(200);
    }
    runner_log(`Loaded after ${attempts} attempt/s`);
}

function includeJs(jsFilePath) {
    const js = document.createElement("script");
    js.type = "text/javascript";
    js.src = jsFilePath;
    document.body.appendChild(js);
}

// MISC
function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms))
}


;// CONCATENATED MODULE: ./userscript/entry.js

new Runner().run();
/******/ })()
;