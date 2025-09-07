# MAIL
<img width="860" height="265" alt="image" src="https://github.com/user-attachments/assets/8cbd2844-e803-45aa-8424-317a5aee4360" />


mail is a game where you post some mail! its just a simple adventure-platformer but theres a bit to explore and several NPCs.

its not always clear where youre supposed to go, which is intentional! explore everything you see!

## play

you can play right here on the web!

URL: https://ingobeans.github.io/mail/

# building from source

## standalone

just do good old `cargo run`

## build for web

with `basic-http-server`, do:
```bash
cargo build --release --target wasm32-unknown-unknown && cp target/wasm32-unknown-unknown/release/mail.wasm web/ && basic-http-server web/
```