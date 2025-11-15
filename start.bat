@echo off
echo Starting MarketHub Platform...
echo.

echo [1/2] Starting Backend Server...
cd backend
start cmd /k "cargo run"
cd ..

timeout /t 3 /nobreak > nul

echo [2/2] Starting Frontend Server...
cd frontend
start cmd /k "python -m http.server 3000"
cd ..

echo.
echo ========================================
echo MarketHub is starting!
echo Backend: http://localhost:8080
echo Frontend: http://localhost:3000
echo ========================================
echo.
echo Press any key to exit...
pause > nul
