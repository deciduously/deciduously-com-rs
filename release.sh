#/usr/bin/env sh

OUT="release/"

# build executabe
cargo build --release

# purge release dir
if [ -d "$OUT" ]; then
    rm -rf $OUT
fi
mkdir $OUT

# copy everything in
cp LICENSE $OUT
cp README.md $OUT
cp -r templates $OUT
cp target/release/deciduously-com $OUT