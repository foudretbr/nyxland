# Tuer le processus DISM qui etait bloque a 50%
Stop-Process -Name "DismHost" -Force -ErrorAction SilentlyContinue

$sshDir = "C:\Program Files\OpenSSH"
Write-Host "[1/4] Installation des fichiers OpenSSH dans $sshDir..." -ForegroundColor Cyan
if (-not (Test-Path $sshDir)) {
    New-Item -ItemType Directory -Path $sshDir -Force | Out-Null
}
Copy-Item -Path "D:\nyxland\scripts\OpenSSH-Win64\*" -Destination $sshDir -Recurse -Force

Write-Host "[2/4] Enregistrement du service sshd..." -ForegroundColor Cyan
& powershell.exe -ExecutionPolicy Bypass -File "$sshDir\install-sshd.ps1"

Write-Host "[3/4] Demarrage du service..." -ForegroundColor Cyan
Set-Service -Name sshd -StartupType Automatic
Start-Service sshd

Write-Host "[4/4] Ouverture du port 22 dans le pare-feu..." -ForegroundColor Cyan
Remove-NetFirewallRule -Name "OpenSSH-Server-In-TCP" -ErrorAction SilentlyContinue
New-NetFirewallRule -Name 'OpenSSH-Server-In-TCP' -DisplayName 'OpenSSH Server (sshd)' -Enabled True -Direction Inbound -Protocol TCP -Action Allow -LocalPort 22

Write-Host "`n[+++] SERVEUR SSH ACTIVE ET OPERATIONNEL SUR LE PORT 22 ! [+++]" -ForegroundColor Green
Write-Host "[i] Tu peux maintenant connecter Zed depuis ton PC portable :" -ForegroundColor Cyan
Write-Host "    ssh tbenn@192.168.1.153/D:/nyxland" -ForegroundColor Yellow
