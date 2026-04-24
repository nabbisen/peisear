#!/bin/sh

cd crates
crates="peisear-core peisear-auth peisear-storage peisear-web"
for crate in $crates; do
    cd $crate
    cargo package
    cargo publish
    cd ..
done
cd ..

cargo package
cargo publish
