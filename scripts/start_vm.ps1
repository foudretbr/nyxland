<#
.SYNOPSIS
    Lance la machine virtuelle Nyxland_Dev_VM et valide automatiquement le boot EFI.
#>

param(
    [string]$VmName = "Nyxland_Dev_VM"
)

$vboxDefaultPath = "C:\Program Files\Oracle\VirtualBox\VBoxManage.exe"
$vboxmanage = if (Test-Path $vboxDefaultPath) { $vboxDefaultPath } else { "VBoxManage" }

$state = & $vboxmanage showvminfo $VmName --machinereadable | Select-String -Pattern '^VMState='
if ($state -match '"running"') {
    Write-Host "[+] La VM '$VmName' est deja en cours d'execution !" -ForegroundColor Green
    exit 0
}

Write-Host "[*] Demarrage de la VM '$VmName'..." -ForegroundColor Cyan
& $vboxmanage startvm $VmName

Write-Host "[*] Envoi automatique de la touche de validation de boot..." -ForegroundColor Cyan
Start-Sleep -Seconds 4
& $vboxmanage controlvm $VmName keyboardputscancode 39 b9

Write-Host "[+] VM demarree avec succes !" -ForegroundColor Green
