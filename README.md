# EnglishSpanishTranslation
Download rust if not installed at https://www.rust-lang.org/tools/install

download the code

Make sure rust is up to date. 1.56 or higher (to check do rustc --version)

if not then copy and paste this. curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh


then after the update type this in the terminal
```bash
rustup target add wasm32-unknown-unknown
```

next type cargo install trunk

lastly do trunk serve and go to localhost:8080
