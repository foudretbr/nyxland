<#
.SYNOPSIS
    Script de configuration automatique de la VM Windows de test pour Nyxland.
.DESCRIPTION
    Ce script configure l'environnement de virtualisation dans D:\Logiciel ou VM  etc\VMs\Windows_Dev
    pour tester Nyxland (Tiling Window Manager) en toute sécurité sans impacter le bureau hôte.
#>

param(
    [ValidateSet("VirtualBox", "VMware")]
    [string]$Hypervisor = "VirtualBox",
    [string]$VmDirectory = "D:\Logiciel ou VM  etc\VMs\Windows_Dev"
)

Write-Host "=================================================" -ForegroundColor Cyan
Write-Host "  Nyxland Windows Manager - Setup VM de Test     " -ForegroundColor Cyan
Write-Host "=================================================" -ForegroundColor Cyan

if (-not (Test-Path $VmDirectory)) {
    New-Item -ItemType Directory -Path $VmDirectory -Force | Out-Null
    Write-Host "[+] Dossier VM créé : $VmDirectory" -ForegroundColor Green
} else {
    Write-Host "[*] Dossier VM existant : $VmDirectory" -ForegroundColor Yellow
}

if ($Hypervisor -eq "VirtualBox") {
    $vbox = Get-Command VBoxManage -ErrorAction SilentlyContinue
    if (-not $vbox) {
        Write-Host "[*] VirtualBox n'est pas détecté dans le PATH." -ForegroundColor Yellow
        Write-Host "[>] Installation de VirtualBox via winget..." -ForegroundColor Cyan
        winget install --id Oracle.VirtualBox --location "D:\Logiciel ou VM  etc\VirtualBox" --accept-source-agreements --accept-package-agreements
    } else {
        Write-Host "[+] VirtualBox est déjà installé : $($vbox.Source)" -ForegroundColor Green
    }
} elseif ($Hypervisor -eq "VMware") {
    $vmware = Get-Command vmware -ErrorAction SilentlyContinue
    if (-not $vmware) {
        Write-Host "[*] VMware Workstation n'est pas détecté." -ForegroundColor Yellow
        Write-Host "[>] Installation de VMware Workstation Pro (gratuit pour usage perso) via winget..." -ForegroundColor Cyan
        winget install --id VMware.WorkstationPro --location "D:\Logiciel ou VM  etc\VMware" --accept-source-agreements --accept-package-agreements
    } else {
        Write-Host "[+] VMware est déjà installé : $($vmware.Source)" -ForegroundColor Green
    }
}

Write-Host "`n[+] Configuration prête dans $VmDirectory !" -ForegroundColor Green
Write-Host "[i] Vous pouvez placer votre ISO Windows dans ce dossier et lancer la VM pour tester Nyxland." -ForegroundColor Cyan
