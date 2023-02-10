
To watch:
```
cargo watch -i .gitignore -i "pkg/*" -s "wasm-pack build --target no-modules --dev"
```

Also need to serve http from `pkg/`
```
npx http-server --cors
```