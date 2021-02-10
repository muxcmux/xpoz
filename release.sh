#!/bin/bash
set -e

tt=$(gcc -dumpmachine)
ver=$(awk -F ' = ' '$1 ~ /version/ { gsub(/[\"]/, "", $2); printf("%s",$2) }' Cargo.toml)
name="xpoz-$ver-$tt"
release="releases/$name"

cargo build --release

mkdir -p "$release"

mv target/release/xpoz "$release/xpoz"

cd frontend
yarn build

mv build/ "../$release/public"

cd "../$release"

rm -rf public/_dist_
rm -rf public/__snowpack__
rm -rf public/web_modules
rm -rf public/js/*.txt

cd ..

tar -czvf "$name.tar.gz" $name

rm -rf "$name"
