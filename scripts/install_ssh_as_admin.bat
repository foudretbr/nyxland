@echo off
:: Auto-elevation : demande les droits Administrateur automatiquement
net session >nul 2>&1
if %errorlevel% neq 0 (
    echo Demande des droits Administrateur (clique sur OUI)...
    powershell -Command "Start-Process cmd -ArgumentList '/c \"\"%~f0\"\"' -Verb RunAs"
    exit /b
)

:: Execution du script OpenSSH
echo [*] Installation OpenSSH en cours...
powershell -NoProfile -ExecutionPolicy Bypass -File "D:\nyxland\scripts\setup_ssh.ps1"
echo [*] Termine ! Appuie sur une touche pour fermer.
pause
