# Running the frontend
1. Make sure you have the wasm32-unknown-unknown rust target installed. You can do this with `rustup target add wasm32-unknown-unknown`.
2. Install wasm-pack (`cargo install wasm-pack`).
3. Navigate to web/ and run `npm i && npm run build-wasm`.
4. `npm run dev` and it should be working.

# Supported instructions
1. mov
2. add
3. sub
4. xor
5. cmp
6. jmp
7. j(e/ne/a/ae/b/be/g/ge/l/le)

# Features
1. Labels defined with `label:`
2. Register view
3. Memory view
