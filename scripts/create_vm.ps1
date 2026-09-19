<#
.SYNOPSIS
    Création automatique de la machine virtuelle VirtualBox pour tester Nyxland.
.DESCRIPTION
    Configure une VM "Nyxland_Dev_VM" optimisée et stockée exclusivement sur le disque D:
    dans D:\Logiciel ou VM  etc\VMs\Windows_Dev.
#>

param(
    [string]$VmName = "Nyxland_Dev_VM",
    [string]$VmFolder = "D:\Logiciel ou VM  etc\VMs\Windows_Dev",
    [string]$IsoPath = "",
    [int]$RamMb = 4096,
    [int]$CpuCount = 4,
    [int]$DiskSizeMb = 51200
)

$vboxDefaultPath = "C:\Program Files\Oracle\VirtualBox\VBoxManage.exe"
$vboxCustomPath = "D:\Logiciel ou VM  etc\VirtualBox\VBoxManage.exe"

$vboxmanage = $null
if (Get-Command VBoxManage -ErrorAction SilentlyContinue) {
    $vboxmanage = "VBoxManage"
} elseif (Test-Path $vboxDefaultPath) {
    $vboxmanage = $vboxDefaultPath
} elseif (Test-Path $vboxCustomPath) {
    $vboxmanage = $vboxCustomPath
}

if (-not $vboxmanage) {
    Write-Host "[-] VBoxManage.exe introuvable. Veuillez exécuter le fichier d'installation téléchargé dans D:\Logiciel ou VM  etc\VirtualBox-7.0.20-Win.exe" -ForegroundColor Red
    exit 1
}

Write-Host "=================================================" -ForegroundColor Cyan
Write-Host "  Configuration VM Nyxland (Disque D: garanti)   " -ForegroundColor Cyan
Write-Host "=================================================" -ForegroundColor Cyan

# 0. Définir le dossier par défaut de VirtualBox sur D:
Write-Host "[0/6] Configuration du dossier global VirtualBox sur D:..." -ForegroundColor Cyan
& $vboxmanage setproperty defaultmachinefolder $VmFolder

# 1. Créer et enregistrer la VM sur D:
Write-Host "[1/6] Création de la machine virtuelle..." -ForegroundColor Cyan
& $vboxmanage createvm --name $VmName --ostype "Windows11_64" --register --basefolder $VmFolder

# 2. Configurer CPU, RAM, VRAM, Chipset ICH9, TPM 2.0 et Réseau
Write-Host "[2/6] Configuration des ressources (RAM: ${RamMb}Mo, CPU: $CpuCount)..." -ForegroundColor Cyan
& $vboxmanage modifyvm $VmName --memory $RamMb --cpus $CpuCount --vram 128 --graphicscontroller vboxsvga --chipset ich9 --firmware efi --tpm-type 2.0 --boot1 dvd --boot2 disk --boot3 none --boot4 none --nic1 nat

# 3. Contrôleur Stockage SATA
Write-Host "[3/6] Ajout du contrôleur de stockage..." -ForegroundColor Cyan
& $vboxmanage storagectl $VmName --name "SATA" --add sata --controller IntelAHCI --bootable on

# 4. Attacher l'ISO sur le Port 0 (Lecteur DVD prioritaire)
if ($IsoPath -and (Test-Path $IsoPath)) {
    Write-Host "[4/6] Montage de l'image ISO sur le lecteur optique (Port 0) : $IsoPath" -ForegroundColor Cyan
    & $vboxmanage storageattach $VmName --storagectl "SATA" --port 0 --device 0 --type dvddrive --medium $IsoPath
} else {
    Write-Host "[4/6] Aucun ISO monté pour l'instant." -ForegroundColor Yellow
}

# 5. Créer et attacher le disque virtuel VDI sur D: (Port 1)
$vdiPath = Join-Path $VmFolder "$VmName\$VmName.vdi"
Write-Host "[5/6] Création du disque virtuel VDI sur D: (${DiskSizeMb}Mo)..." -ForegroundColor Cyan
& $vboxmanage createmedium disk --filename $vdiPath --size $DiskSizeMb --format VDI
& $vboxmanage storageattach $VmName --storagectl "SATA" --port 1 --device 0 --type hdd --medium $vdiPath

Write-Host "`n[+] VM '$VmName' configurée avec succès dans : $VmFolder" -ForegroundColor Green
Write-Host "[+] Emplacement du disque VDI : $vdiPath" -ForegroundColor Green
Write-Host "[i] Pour lancer la VM : & '$vboxmanage' startvm '$VmName'" -ForegroundColor Cyan
Write-Host "[!] NOTE : Au lancement de la VM, cliquer immédiatement dans la fenêtre et appuyer sur ESPACE pour valider le boot CD/DVD." -ForegroundColor Yellow
