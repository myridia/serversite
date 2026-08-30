rm target -Rf
rm public -Rf
cargo build --release
./target/release/serversite-gen -o salamander -d config
firefox public/index.html
