## Prerequisites (macOS Setup)

### 1. Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Follow the on-screen instructions. After installation, restart your terminal or run:
```bash
source ~/.cargo/env
```

### 2. Install wasm-pack
```bash
brew install wasm-pack
```
Or alternatively:
```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

## Building
```bash
wasm-pack build --target web
```

## Testing
```bash
python3 -m http.server 8000
# Then open http://localhost:8000/test.html
# Type a password in the input box to see live strength analysis
```
