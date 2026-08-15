To cross compile for Raspberry Pi Zero W:

1) Install Docker:
    - `curl -fsSL https://get.docker.com | sh`
    - `sudo usermod -aG docker $USER`

2) Install cross:
    - `cargo install cross --git https://github.com/cross-rs/cross`

3) Add rustup target:
    - `rustup target add arm-unknown-linux-gnueabihf`

4) Build outpost-server:
    `sudo -E env "PATH=$PATH" cross build --release --target arm-unknown-linux-gnueabihf -p outpost_server`