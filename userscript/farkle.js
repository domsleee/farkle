(() => {
const log = console.log;

class Farkle {
    constructor(wasm_bindgen) {
        const { FarkleSolver } = wasm_bindgen;
        this.farkleSolver = new FarkleSolver();
        this.dicePositions = {};
        for (let id of this._getDiceIds()) this.dicePositions[id] = '';
    }

    async run() {
        console.time('first action')
        this.farkleSolver.decide_action_ext(0, 6, [0, 0]);
        console.timeEnd('first action');

        while (true) {
            await this._waitForYourTurn();
            await this._doAction(this._getText());
            await this._waitForStationaryDice();
        }
    }

    async _doAction(text) {
        let heldScore = this._getHeldScore();
        const score = parseInt($('#bottom-player-total').text());
        const otherScore = parseInt($('#top-player-total').text());
        let [diceInPlayEls, diceInPlay] = this._getDiceInPlay();
        const totalScores = [score, otherScore];
        let numDiceHeldForAction = 0;
        // log(`heldScore: ${heldScore}, dice in play: [${diceInPlay}], scores: [${totalScores}]`);
        if (diceInPlay.length > 0) {
            const roll = diceInPlay.join('').toString();
            log(`decide_held_dice_ext(held_score: ${heldScore}, roll: ${roll}, scores: [${totalScores}])...`);
            const diceToHold = this.farkleSolver.decide_held_dice_ext(heldScore, roll, totalScores);
            log(`diceToHold: ${diceToHold}`);
            numDiceHeldForAction = diceToHold.length;
            if (diceToHold.length > 0) {
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
                await this._waitForStationaryDice();
            }
        }

        heldScore = this._getHeldScore();
        let diceLeft = diceInPlay.length - numDiceHeldForAction;
        if (diceLeft === 0) diceLeft = 6;

        if (diceLeft <= 0 || diceLeft > 6) {
            debugger;
        }
        log(`decide_action_ext(held_score: ${heldScore}, dice_left: ${diceLeft}, scores: [${totalScores}])...`);
        const action = this.farkleSolver.decide_action_ext(heldScore, diceLeft, totalScores);
        log(`action: ${action}`)
        if (action === 'Stay') {
            this._getBankButton().click();
        } else {
            this._getRollButton().click();
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

    async _waitForYourTurn() {
        //log('waiting for turn...');
        while (true) {
            if (!this._getRollButton().disabled
                || !this._getBankButton().disabled) break;
            await sleep(100);
        }
        //log('your turn');
    }

    async waitForClick() {
        let clicked = false;
        const clickHander = () =>  {
            clicked = true;
            document.removeEventListener('click', clickHander);
        }
        document.addEventListener('click', clickHander);
        while (!clicked) {
            await sleep(100);
        }
        console.log('clicked.');
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

window.Farkle = Farkle;
})();