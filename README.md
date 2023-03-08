
To develop, use `npm start`

It runs three scripts
* wasm-pack to compile rust to wasm, exporting to `pkg/`
* webpack for the `userscript/` directory, exporting to `pkg/`
* http-server of the `pkg/` directory

### Results ?
JSON.parse(localStorage['farkleSolver.results'])
`{wins: 1260, loss: 974}`