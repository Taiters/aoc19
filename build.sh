#!/bin/sh

for day in day*/; do
    cd $day
    echo "---- $day ----"
    cargo build
    cargo test
    cd ..
done