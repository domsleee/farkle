import { ProbBar } from './probBar';
import { sleep } from './runner';
const log = console.log;
const localStorageKey = 'farkleSolver.results';

export class FarkleScript {
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
        const filesToTry = [
            `${this.SERVER}/exact.bincode`,
            `${this.SERVER}/approx.bincode`
        ]
        for (let i = 0; i < filesToTry.length; ++i) {
            const file = filesToTry[i];
            try {
                await this.populate_solver(file, this.farkleSolver);
                break;
            } catch (e) {
                console.log(e);
                if (i == filesToTry.length-1) {
                    console.log("WARNING!!! Could not load any cache file. Suggest running `cargo run --release -- approx`.\n"
                    + "Otherwise be prepared to wait for several minutes while the entire tree is calculated in wasm");
                }
            }
        }
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