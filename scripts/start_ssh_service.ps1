Write-Host "=== Demarrage du serveur SSH ===" -ForegroundColor Cyan
if (-not (Test-Path "C:\ProgramData\ssh")) {
    New-Item -ItemType Directory -Path "C:\ProgramData\ssh" -Force | Out-Null
}
Copy-Item "D:\nyxland\scripts\OpenSSH-Win64\sshd_config_default" "C:\ProgramData\ssh\sshd_config" -Force
& "D:\nyxland\scripts\OpenSSH-Win64\install-sshd.ps1"
Set-Service -Name sshd -StartupType Automatic
Start-Service sshd
Get-Service sshd
Write-Host "`n[+] Serveur SSH demarre et pret pour Zed !" -ForegroundColor Green
