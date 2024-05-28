cd ./web/
npx wasm-pack build "../" --target web --out-name web --out-dir ./pkg
cd ..
yes | docker compose rm && docker compose build && docker compose up
