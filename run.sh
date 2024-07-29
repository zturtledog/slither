#!/bin/sh

cargo build
cp target/debug/slither out
chmod +x out/slither
./out/slither