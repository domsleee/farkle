(() => {
const log = console.log;

class Farkle {
    constructor(wasm_bindgen) {
        const { greet, FarkleSolver } = wasm_bindgen;
        this.greet = greet;
        this.farkleSolver = new FarkleSolver();
        this.dicePositions = {};
        for (let id of this._getDiceIds()) this.dicePositions[id] = '';
    }

    async run() {
        log(this.greet());
        while (true) {
            await this._waitForYourTurn();
            await this._doAction(this._getText());
            await this._waitForStationaryDice();
        }
    }

    async _doAction(text) {
        const heldScore = parseInt($('#bottom-player-round-score b').text());
        const score = parseInt($('#bottom-player-total').text());
        const otherScore = parseInt($('#top-player-total').text());
        const diceInPlayEls = Array.from($('.dice'))
            .map(el => el)
            .filter(el => !el.alt.includes('saved'))
            .sort((a, b) => parseInt(a.alt) - parseInt(b.alt));
        const diceInPlay = diceInPlayEls.map(t => parseInt(t.alt));
        const totalScores = [score, otherScore];
        log(`heldScore: ${heldScore}, dice in play: [${diceInPlay}], scores: [${totalScores}]`);
        if (diceInPlay.length > 0) {
            const newHeldDice = this.farkleSolver.decide_held_dice_ext(heldScore, diceInPlay.join('').toString(), totalScores);
            log(`newHeldDice: ${newHeldDice}`);
            if (newHeldDice.length > 0) {
                let elsToClick = [];
                let j = 0;
                for (let el of diceInPlayEls) {
                    if (j < newHeldDice.length && el.alt === newHeldDice[j]) {
                        elsToClick.push(el);
                        j += 1;
                    }
                }
                for (let el of elsToClick) {
                    el.click();
                    await sleep(350);
                }
                await this._waitForStationaryDice();
            } else {
                return;
            }
        }

        const action = this.farkleSolver.decide_action_ext(heldScore, diceInPlay.length, totalScores);
        log(`action: ${action}`)
        if (action === 'Stay') {
            this._getBankButton().click();
        } else {
            this._getRollButton().click();
        }
    }

    _getRollButton = () => $("#throw-button button")[0];
    _getBankButton = () => $("#bank-button button")[0];

    async _waitForYourTurn() {
        log('waiting for turn...');
        while (true) {
            if (!this._getRollButton().disabled
                || !this._getBankButton().disabled) break;
            await sleep(100);
        }
        log('your turn');
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