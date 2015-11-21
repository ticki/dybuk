cargo build --release

sudo cp target/release/dybuk /usr/local/bin/dybuk

sudo cp -rf target/release/deps /usr/local/lib/rustlib/dybuk
