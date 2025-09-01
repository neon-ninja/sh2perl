@echo off
echo To run this test: ./fail FAILING_TEST_NUMBER
echo To run a specific test: ./fail TEST_NUMBER
git stash push examples .\f.bat .\fail.bat .\src\main.rs src\testing.rs
cargo run --bin debashc -- fail %*

REM Simulate keypresses for Cursor
powershell -Command "Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.SendKeys]::SendWait('+{BACKSPACE}'); [System.Windows.Forms.SendKeys]::SendWait('{ENTER}')"
