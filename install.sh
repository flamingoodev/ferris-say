workdir .
cargo build --release
sudo cp target/release/ferris-say /usr/local/bin/
echo '🍺 ferris-say installed successfully!'
