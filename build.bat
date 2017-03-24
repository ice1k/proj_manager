@echo off
REM cargo build --release
cargo build
cd ./target/debug
move *.exe ../../
REM copy *.exe ../../
cd ../../
del /s /f /q mng.exe
del /s /f /q command.exe
del /s /f /q drug.exe
del /s /f /q string_apis.exe
rename proj_manager.exe mng.exe
echo ===============================================
echo ===============================================
echo ===============================================
echo ================build success==================
echo ===============================================
echo ===============================================
echo ===============================================
mng