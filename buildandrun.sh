./deletecontainer.sh
./deleteimage.sh
npx wasm-pack build "./" --target web --out-name web --out-dir ./web/pkg
yes | docker compose rm && docker compose build && docker compose up
