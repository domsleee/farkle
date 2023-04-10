
export class ProbBar {
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