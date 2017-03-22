@echo off
cargo clean
REM cargo build --release
cargo build
cd ./target/debug
move proj_manager.exe ../../
cd ../../