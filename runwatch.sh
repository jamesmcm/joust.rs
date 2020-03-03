#!/bin/sh 

cargo watch -i .gitignore -i "pkg/*" -s "wasm-pack build" &
 cd www && npm run start
