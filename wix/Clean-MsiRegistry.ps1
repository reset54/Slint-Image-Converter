<#
.SYNOPSIS
    MSI registry cleanup with auto-backup.
    Target: SlintImageConverter UpgradeCode.
#>

Param(
    [string]$UpgradeCode = "{03e94582-7412-4e12-8d5a-9c43821764c2}"
)

# Admin privileges validation
if (-not ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    Write-Error "Elevated privileges required."
    exit 1
}


function Get-PackedGuid([string]$Guid) {
    $clean = $Guid.Replace("{", "").Replace("}", "").Replace("-", "")
    $p1 = $clean.Substring(0, 8).ToCharArray(); [Array]::Reverse($p1)
    $p2 = $clean.Substring(8, 4).ToCharArray(); [Array]::Reverse($p2)
    $p3 = $clean.Substring(12, 4).ToCharArray(); [Array]::Reverse($p3)
    $res = -join $p1 + (-join $p2) + (-join $p3)
    for ($i = 16; $i -lt 32; $i += 2) { $res += $clean[$i+1] + $clean[$i] }
    return $res.ToUpper()
}


$packed = Get-PackedGuid $UpgradeCode
$backupDir = Join-Path $PSScriptRoot "reg_backups"
if (-not (Test-Path $backupDir)) { New-Item -ItemType Directory -Path $backupDir | Out-Null }

$registryPaths = @(
    "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Installer\UpgradeCodes\$packed",
    "HKCR:\Installer\UpgradeCodes\$packed",
    "HKCU\Software\Microsoft\Installer\UpgradeCodes\$packed",
    "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Installer\UserData\S-1-5-18\Products\$packed"
)

foreach ($path in $registryPaths) {
    if (Test-Path $path) {
        # Prepare path for reg.exe export
        $regExportPath = $path -replace "HKLM:", "HKEY_LOCAL_MACHINE" `
                              -replace "HKCU:", "HKEY_CURRENT_USER" `
                              -replace "HKCR:", "HKEY_CLASSES_ROOT"

        $backupFile = Join-Path $backupDir "backup_$($packed)_$(Get-Date -Format 'HHmmss').reg"

        # Silent export
        & reg export $regExportPath "$backupFile" /y | Out-Null

        # Remove key
        Remove-Item -Path $path -Recurse -Force -ErrorAction SilentlyContinue
        Write-Host "Processed and backed up: $path" -ForegroundColor Gray
    }
}

Write-Host "MSI State Reset Complete." -ForegroundColor Green
