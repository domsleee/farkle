/*
 * ATTENTION: The "eval" devtool has been used (maybe by default in mode: "development").
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
/******/ (() => { // webpackBootstrap
/******/ 	"use strict";
/******/ 	var __webpack_modules__ = ({

/***/ "./userscript/entry.js":
/*!*****************************!*\
  !*** ./userscript/entry.js ***!
  \*****************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _runner__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./runner */ \"./userscript/runner.js\");\n// ==UserScript==\r\n// @name          Farkle\r\n// @namespace     http://www.example.com/\r\n// @description   WASM test\r\n// @include       *\r\n// @require       /Users/dom/git/farkle/pkg/entry.bundle.js\r\n// ==/UserScript==\r\n\r\n\r\nnew _runner__WEBPACK_IMPORTED_MODULE_0__.Runner().run();\n\n//# sourceURL=webpack://farkle/./userscript/entry.js?");

/***/ }),

/***/ "./userscript/farkleScript.js":
/*!************************************!*\
  !*** ./userscript/farkleScript.js ***!
  \************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   \"FarkleScript\": () => (/* binding */ FarkleScript)\n/* harmony export */ });\n/* harmony import */ var _probBar__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./probBar */ \"./userscript/probBar.js\");\n/* harmony import */ var _runner__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./runner */ \"./userscript/runner.js\");\n\r\n\r\nconst log = console.log;\r\nconst localStorageKey = 'farkleSolver.results';\r\n\r\nclass FarkleScript {\r\n    constructor(wasm_bindgen, SERVER) {\r\n        const { set_panic_hook, FarkleSolverWasm, populate_solver } = wasm_bindgen;\r\n        set_panic_hook();\r\n        this.probBar = new _probBar__WEBPACK_IMPORTED_MODULE_0__.ProbBar();\r\n        this.farkleSolver = new FarkleSolverWasm();\r\n        this.populate_solver = populate_solver;\r\n        this.dicePositions = {};\r\n        this.turnNumber = 0;\r\n        this.SERVER = SERVER;\r\n        for (let id of this._getDiceIds()) this.dicePositions[id] = '';\r\n    }\r\n\r\n    async run() {\r\n        console.log('w/ record', this._getLocalStorage());\r\n\r\n        console.time('populate');\r\n        await this.populate_solver(`${this.SERVER}/exact.bincode`, this.farkleSolver);\r\n        console.timeEnd('populate');\r\n\r\n        console.time(`first action`)\r\n        this.farkleSolver.decide_action_ext(0, 6, [0, 0]);\r\n        console.timeEnd('first action');\r\n\r\n        while (true) {\r\n            await this._waitForYourTurn();\r\n            if (this._getStartNewGameButton() != null) {\r\n                this._recordWinnerAndRefresh();\r\n                break;\r\n            }\r\n            await this._doAction();\r\n            await this._waitForStationaryDice();\r\n        }\r\n    }\r\n\r\n    _getScores = () => [\r\n        parseInt($('#bottom-player-total').text()),\r\n        parseInt($('#top-player-total').text())\r\n    ];\r\n\r\n    async _doAction() {\r\n        let heldScore = 0;\r\n        let totalScores = this._getScores();\r\n\r\n        let diceLeft = 6;\r\n        const [initialProb, _] = this.farkleSolver.decide_action_ext(heldScore, diceLeft, totalScores);\r\n        this.probBar.setTurnNumber(++this.turnNumber);\r\n        this.probBar.setWinProbability(initialProb);\r\n\r\n        while (true) {\r\n            log(`decide_action_ext(held_score: ${heldScore}, dice_left: ${diceLeft}, scores: [${totalScores}])...`);\r\n            const [probAction, action] = this.farkleSolver.decide_action_ext(heldScore, diceLeft, totalScores);\r\n            log(probAction, `action: ${action}`);\r\n            this.probBar.setWinProbability(probAction);\r\n            if (action === 'Stay') {\r\n                this._getBankButton().click();\r\n                break;\r\n            }\r\n\r\n            this._getRollButton().click();\r\n            await this._waitForStationaryDice();\r\n            let [diceInPlayEls, diceInPlay] = this._getDiceInPlay();\r\n            const roll = diceInPlay.join('').toString();\r\n            log(`decide_held_dice_ext(held_score: ${heldScore}, roll: ${roll}, scores: [${totalScores}])...`);\r\n            const [probHeldDice, diceToHoldScore, diceToHold] = this.farkleSolver.decide_held_dice_ext(heldScore, roll, totalScores);\r\n            log(probHeldDice, `hold: ${diceToHold}`);\r\n            this.probBar.setWinProbability(probHeldDice);\r\n            if (diceToHoldScore === 0) {\r\n                break;\r\n            }\r\n            \r\n            await this._clickDice(diceToHold);\r\n            await this._waitForStationaryDice();\r\n            heldScore += diceToHoldScore;\r\n            diceLeft -= diceToHold.length;\r\n            if (diceLeft === 0) {\r\n                diceLeft = 6;\r\n            }\r\n        }\r\n    }\r\n\r\n    _recordWinnerAndRefresh() {\r\n        const winLossRecord = this._getLocalStorage();\r\n        const scores = this._getScores();\r\n        if (scores[0] > scores[1]) {\r\n            winLossRecord.wins++;\r\n        } else {\r\n            winLossRecord.loss++;\r\n        }\r\n        localStorage[localStorageKey] = JSON.stringify(winLossRecord);\r\n        console.log(winLossRecord);\r\n        this._getStartNewGameButton().click();\r\n    }\r\n    _getLocalStorage() {\r\n        try {\r\n            return JSON.parse(localStorage[localStorageKey]);\r\n        } catch {\r\n            return {wins: 0, loss: 0};\r\n        }\r\n    }\r\n    \r\n    async _clickDice(diceToHold) {\r\n        let [diceInPlayEls] = this._getDiceInPlay();\r\n        let elsToClick = [];\r\n        let j = 0;\r\n        for (let el of diceInPlayEls) {\r\n            if (j < diceToHold.length && el.alt === diceToHold[j]) {\r\n                elsToClick.push(el);\r\n                j += 1;\r\n            }\r\n        }\r\n        for (let el of elsToClick) {\r\n            el.click();\r\n            await (0,_runner__WEBPACK_IMPORTED_MODULE_1__.sleep)(350);\r\n        }\r\n    }\r\n    _getHeldScore = () => parseInt($('#bottom-player-round-score b').text());\r\n    _getDiceInPlay = () => {\r\n        const diceInPlayEls = Array.from($('.dice'))\r\n            .map(el => el)\r\n            .filter(el => !el.alt.includes('saved'))\r\n            .sort((a, b) => parseInt(a.alt) - parseInt(b.alt));\r\n        const diceInPlay = diceInPlayEls.map(t => parseInt(t.alt));\r\n        return [diceInPlayEls, diceInPlay];\r\n    }\r\n\r\n    _getRollButton = () => $(\"#throw-button button\")[0];\r\n    _getBankButton = () => $(\"#bank-button button\")[0];\r\n    _getStartNewGameButton = () => $(\"#result-box[style='display: block;'] button#start-new-game\").toArray().at(0)\r\n\r\n    async _waitForYourTurn() {\r\n        //log('waiting for turn...');\r\n        while (true) {\r\n            if (!this._getRollButton().disabled\r\n                || !this._getBankButton().disabled\r\n                || this._getStartNewGameButton() != null) break;\r\n            await (0,_runner__WEBPACK_IMPORTED_MODULE_1__.sleep)(100);\r\n        }\r\n        //log('your turn');\r\n    }\r\n\r\n    async _waitForStationaryDice() {\r\n        while (true) {\r\n            await (0,_runner__WEBPACK_IMPORTED_MODULE_1__.sleep)(100);\r\n            let changed = false;\r\n            for (let id of this._getDiceIds()) {\r\n                const dicePosition = $(`#${id}`)[0].getAttribute('style')\r\n                if (dicePosition !== this.dicePositions[id]) {\r\n                    changed = true;\r\n                }\r\n                this.dicePositions[id] = dicePosition;\r\n            }\r\n            if (!changed) break;\r\n        }\r\n    }\r\n\r\n    _getDiceIds() {\r\n        return [1,2,3,4,5,6].map(id => `d${id}`);\r\n    }\r\n\r\n    _getText() {\r\n        return $('#messageBox').text();\r\n    }\r\n}\n\n//# sourceURL=webpack://farkle/./userscript/farkleScript.js?");

/***/ }),

/***/ "./userscript/probBar.js":
/*!*******************************!*\
  !*** ./userscript/probBar.js ***!
  \*******************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   \"ProbBar\": () => (/* binding */ ProbBar)\n/* harmony export */ });\n\r\nclass ProbBar {\r\n    constructor() {\r\n        const div = document.createElement('div');\r\n        div.id = 'scorecard-column'\r\n        div.innerHTML = `\r\n            <table id='scorecard'>\r\n                <tbody>\r\n                    <tr>\r\n                        <td>Turn number</td>\r\n                        <td class='turnNumber'>0</td>\r\n                    </tr>\r\n                    <tr>\r\n                        <td>Win prob.</td>\r\n                        <td style='position: relative'>\r\n                            <span class='winChance' style='position: relative; z-index: 3'>50%</span>\r\n                            <span class='winChanceBar' style='background:lightgreen; position: absolute; left:0; top:0; bottom:0; height:100%; transition: width 400ms ease-in-out; z-index: 2'></span>\r\n                        </td>\r\n                    </tr>\r\n                </tbody>\r\n            </table>`;\r\n        $(\"#player-scores div:first\").after(div);\r\n    }\r\n\r\n    setTurnNumber(turnNumber) {\r\n        $(\"td.turnNumber\").text(turnNumber);\r\n    }\r\n\r\n    setWinProbability(prob) {\r\n        const winChanceText = `${(100 * prob).toFixed(2)}%`;\r\n        $(\"span.winChance\").text(winChanceText);\r\n        $(\"span.winChanceBar\")[0].style.width = `${prob*100}%`;\r\n    }\r\n}\n\n//# sourceURL=webpack://farkle/./userscript/probBar.js?");

/***/ }),

/***/ "./userscript/runner.js":
/*!******************************!*\
  !*** ./userscript/runner.js ***!
  \******************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   \"Runner\": () => (/* binding */ Runner),\n/* harmony export */   \"sleep\": () => (/* binding */ sleep)\n/* harmony export */ });\n/* harmony import */ var _farkleScript__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./farkleScript */ \"./userscript/farkleScript.js\");\n\r\n\r\nconst log = console.log;\r\nconst SERVER = 'http://127.0.0.1:8080';\r\n\r\nclass Runner {\r\n    async run() {\r\n        await loadWasm();\r\n        await wasm_bindgen(`${SERVER}/farkle_wasm_bg.wasm`);\r\n    \r\n        new _farkleScript__WEBPACK_IMPORTED_MODULE_0__.FarkleScript(wasm_bindgen, SERVER).run();\r\n    }\r\n}\r\n\r\n\r\n// WASM files\r\nasync function loadWasm() {\r\n    let attempts = 0;\r\n    includeJs(`${SERVER}/farkle_wasm.js`);\r\n    while (typeof wasm_bindgen == 'undefined') {\r\n        attempts += 1;\r\n        if (attempts === 5) {\r\n            log(`warning: failed to load wasm_bindgen after ${attempts} attempts`)\r\n        }\r\n        await sleep(200);\r\n    }\r\n    log(`Loaded after ${attempts} attempt/s`);\r\n}\r\n\r\nfunction includeJs(jsFilePath) {\r\n    const js = document.createElement(\"script\");\r\n    js.type = \"text/javascript\";\r\n    js.src = jsFilePath;\r\n    document.body.appendChild(js);\r\n}\r\n\r\n// MISC\r\nfunction sleep(ms) {\r\n    return new Promise(resolve => setTimeout(resolve, ms))\r\n}\r\n\r\n\n\n//# sourceURL=webpack://farkle/./userscript/runner.js?");

/***/ })

/******/ 	});
/************************************************************************/
/******/ 	// The module cache
/******/ 	var __webpack_module_cache__ = {};
/******/ 	
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/ 		// Check if module is in cache
/******/ 		var cachedModule = __webpack_module_cache__[moduleId];
/******/ 		if (cachedModule !== undefined) {
/******/ 			return cachedModule.exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = __webpack_module_cache__[moduleId] = {
/******/ 			// no module.id needed
/******/ 			// no module.loaded needed
/******/ 			exports: {}
/******/ 		};
/******/ 	
/******/ 		// Execute the module function
/******/ 		__webpack_modules__[moduleId](module, module.exports, __webpack_require__);
/******/ 	
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/ 	
/************************************************************************/
/******/ 	/* webpack/runtime/define property getters */
/******/ 	(() => {
/******/ 		// define getter functions for harmony exports
/******/ 		__webpack_require__.d = (exports, definition) => {
/******/ 			for(var key in definition) {
/******/ 				if(__webpack_require__.o(definition, key) && !__webpack_require__.o(exports, key)) {
/******/ 					Object.defineProperty(exports, key, { enumerable: true, get: definition[key] });
/******/ 				}
/******/ 			}
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/hasOwnProperty shorthand */
/******/ 	(() => {
/******/ 		__webpack_require__.o = (obj, prop) => (Object.prototype.hasOwnProperty.call(obj, prop))
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/make namespace object */
/******/ 	(() => {
/******/ 		// define __esModule on exports
/******/ 		__webpack_require__.r = (exports) => {
/******/ 			if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 				Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 			}
/******/ 			Object.defineProperty(exports, '__esModule', { value: true });
/******/ 		};
/******/ 	})();
/******/ 	
/************************************************************************/
/******/ 	
/******/ 	// startup
/******/ 	// Load entry module and return exports
/******/ 	// This entry module can't be inlined because the eval devtool is used.
/******/ 	var __webpack_exports__ = __webpack_require__("./userscript/entry.js");
/******/ 	
/******/ })()
;