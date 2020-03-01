# joustmp

Multiplayer Joust clone using wasm-bindgen and canvas in Rust

## Building

To run with live changes:

```bash
cargo watch -i .gitignore -i "pkg/*" -s "wasm-pack build"
cd www
npm run start
firefox localhost:8080
```

To build static page (for export):

```bash
wasm-pack build --target=web
firefox www/index_static.html
```

You can then copy the files in pkg/ and www/index\_static.html to deploy the game on a static HTML page.
