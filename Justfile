[windows]
set shell := ["powershell.exe", "-NoProfile", "-Command"]

wasm_bindgen_version := "0.2.108"

# Lokale WebApp starten
default: start

# WebAssembly-Buildwerkzeuge einrichten
setup:
    rustup target add wasm32-unknown-unknown
    cargo install wasm-bindgen-cli --version {{wasm_bindgen_version}} --locked

# Rust-Spielkern für den Browser bauen
wasm:
    cargo build --locked -p game-wasm --target wasm32-unknown-unknown --target-dir target --release
    wasm-bindgen --target web --no-typescript --out-dir target/browser --out-name game target/wasm32-unknown-unknown/release/game_wasm.wasm

# Axum starten
start: wasm
    cargo run --locked -p cannon-cutlass-web

# Statischen Build für ChatGPT Sites erzeugen
export: wasm
    cargo run --locked -p cannon-cutlass-web -- export

# Rust-Tests ausführen
test:
    cargo test --workspace --locked

# Formatierung, Tests und den kompletten Export prüfen
check:
    cargo fmt --all -- --check
    just test
    just export
