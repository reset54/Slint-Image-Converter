# Build script for SlintImageConverter.msi
# Location: project_root/wix/build_msi.ps1
# Optimized for WiX v5.0.2 with x64 architecture support

$ErrorActionPreference = "Stop"


# 1. Configuration
$projectName = "slint-image-converter"
# Points to the project root regardless of caller's current directory
$outputMsi = "$PSScriptRoot/../SlintImageConverter.msi"
$releaseExe = "$PSScriptRoot/../target/release/$projectName.exe"
$wixDir = "$PSScriptRoot"
$extVersion = "5.0.2"

Write-Host "`n--- MSI Packaging Process (WiX v5.0.2) ---" -ForegroundColor Cyan


# 2. Cleanup
if (Test-Path $outputMsi) {
    Write-Host "[DEBUG] Removing old installer..." -ForegroundColor Gray
    Remove-Item $outputMsi -Force
}


# 3. Validation
if (-not (Test-Path $releaseExe)) {
    Write-Host "[ERROR] Binary not found at: $releaseExe" -ForegroundColor Red
    Write-Host "[ERROR] Run 'cargo build --release' from the project root first." -ForegroundColor Red
    exit 1
}


# 4. Dependency Management
$nugetCache = "$env:USERPROFILE\.nuget\packages\wixtoolset.ui.wixext\$extVersion"
$extDll = Get-ChildItem -Path $nugetCache -Filter "WixToolset.UI.wixext.dll" -Recurse -ErrorAction SilentlyContinue | Select-Object -First 1

if ($null -eq $extDll) {
    Write-Host "[DEBUG] Extension v$extVersion not found. Downloading via dotnet restore..." -ForegroundColor Yellow

    $tempProject = "$PSScriptRoot/temp_wix_dep.csproj"
    $projectContent = @"
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <TargetFramework>net8.0</TargetFramework>
  </PropertyGroup>
  <ItemGroup>
    <PackageReference Include="WixToolset.UI.wixext" Version="$extVersion" />
  </ItemGroup>
</Project>
"@
    Set-Content -Path $tempProject -Value $projectContent

    try {
        & dotnet restore $tempProject
    }
    finally {
        if (Test-Path $tempProject) { Remove-Item $tempProject }
        $objPath = Join-Path $PSScriptRoot "obj"
        if (Test-Path $objPath) { Remove-Item $objPath -Recurse -Force }
    }

    $extDll = Get-ChildItem -Path $nugetCache -Filter "WixToolset.UI.wixext.dll" -Recurse | Select-Object -First 1
}

if ($null -eq $extDll) {
    Write-Host "[ERROR] Failed to resolve WixToolset.UI.wixext.dll v$extVersion" -ForegroundColor Red
    exit 1
}

Write-Host "[DEBUG] Using DLL: $($extDll.FullName)" -ForegroundColor Green


# 5. Build MSI (with explicit x64 architecture)
Write-Host "`n[DEBUG] --- Step: WiX Build (Target: x64) ---" -ForegroundColor Cyan
$wxsFiles = Get-ChildItem -Path $wixDir -Filter *.wxs | Select-Object -ExpandProperty FullName

try {
    # Direct DLL link and explicit x64 architecture
    & wix build @wxsFiles -ext "$($extDll.FullName)" -arch x64 -o $outputMsi
} catch {
    Write-Host "`n[FATAL] WiX build failed." -ForegroundColor Red
    exit 1
}


# 6. Summary
if (Test-Path $outputMsi) {
    $sizeMB = [math]::Round((Get-Item $outputMsi).Length / 1MB, 2)
    Write-Host "`n--- Success! ---" -ForegroundColor Green
    Write-Host "Installer created: $outputMsi ($sizeMB MB)" -ForegroundColor White
}
