#!/usr/bin/env bash
cargo build --release
sudo rm -f /usr/local/bin/tfexe
sudo install target/release/tfexe /usr/local/bin
