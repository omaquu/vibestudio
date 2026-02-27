@echo off
call "C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\Common7\Tools\VsDevCmd.bat"
if %errorlevel% neq 0 exit /b %errorlevel%
"C:\Users\kuron\.cargo\bin\dx.exe" build --platform web --release
