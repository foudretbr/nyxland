<#
.SYNOPSIS
    Téléchargement automatique de l'ISO officiel Windows pour la VM Nyxland.
.DESCRIPTION
    Utilise le script officiel Fido (utilisé par Rufus) pour générer le lien
    de téléchargement direct depuis les serveurs officiels de Microsoft.
#>

param(
    [string]$DestinationPath = "D:\Logiciel ou VM  etc\VMs\Windows_Dev",
    [ValidateSet("11", "10")]
    [string]$WindowsVersion = "11",
    [string]$Language = "French"
)

$targetDir = $DestinationPath
if (-not (Test-Path $targetDir)) {
    New-Item -ItemType Directory -Path $targetDir -Force | Out-Null
}

Write-Host "=================================================" -ForegroundColor Cyan
Write-Host "  Téléchargement ISO Windows $WindowsVersion ($Language)       " -ForegroundColor Cyan
Write-Host "=================================================" -ForegroundColor Cyan

$fidoScriptPath = Join-Path $targetDir "Fido.ps1"
if (-not (Test-Path $fidoScriptPath)) {
    Write-Host "[>] Téléchargement du moteur Fido officiel..." -ForegroundColor Cyan
    curl.exe -s -L "https://raw.githubusercontent.com/pbatard/Fido/master/Fido.ps1" -o $fidoScriptPath
}

Write-Host "[+] Fido prêt dans : $fidoScriptPath" -ForegroundColor Green
Write-Host "[>] Lancement de la requête de lien ISO Microsoft..." -ForegroundColor Cyan

# Exécuter Fido avec paramètres pour obtenir le lien direct
& powershell.exe -ExecutionPolicy Bypass -File $fidoScriptPath -Win $WindowsVersion -Lang $Language -Arch x64 -Ed Pro -GetUrl
