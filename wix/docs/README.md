This README.md is designed to be a comprehensive guide for both experienced developers and beginners. It covers everything from basic commands to deep-level troubleshooting.
Slint Image Converter: Installer Guide

This directory contains the source code and automation scripts for the Slint Image Converter Windows Installer (MSI). The installer is built using the WiX Toolset v5 and is configured for a native 64-bit (x64) environment.

### 1. Prerequisites (Before You Start)

Before building the installer, ensure you have the following:

Rust Compiler: Installed via rustup.

WiX Toolset v5: Installed via dotnet tool install --global wix.

Compiled Application: You must build the release version of the app first.

```
cargo build --release
```

### 2. How to Build the Installer

The build process is fully automated via a PowerShell script. It handles dependencies, downloads necessary WiX extensions, and compiles the final MSI package.

From the project root, run:
PowerShell

```
powershell .\wix\build_msi.ps1
```

What this does:

Locates the compiled .exe in target/release/.

Fetches the WixToolset.UI.wixext (User Interface) extension.

Compiles the .wxs source files into a single SlintImageConverter.msi in the project root.

### 3. How to Install and View Logs

Since Windows Installer logs are written in a specific encoding (UTF-16 LE), they can look like "gibberish" in standard text editors. Use the following command to install the app and immediately read the log in a human-readable format:
PowerShell

```
msiexec /i SlintImageConverter.msi /l*v install.log; Get-Content .\install.log -Encoding Unicode
```

### 4. How to Uninstall and View Logs

If you need to remove the application and verify that all files and registry keys were deleted, use the uninstall command:
PowerShell

```
msiexec /x SlintImageConverter.msi /l*v uninstall.log; Get-Content .\uninstall.log -Encoding Unicode
```

### 5. The Solution (Registry Reset)

If your installer is not picking up the C:\Program Files\ path, you need to clear the Windows Installer cache. We provide a specialized script for this:

Run the cleanup script:

```
powershell .\wix\Clean-MsiState.ps1
```

What this script does:

Converts your project's UpgradeCode into a "Packed GUID" (the format Windows uses).

Creates a backup of the registry keys in wix\reg_backups\.

Deletes the specific cached entries that force the old installation path.

### 6. Technical Specifications

    Architecture: x64 (64-bit Native).

    Installation Scope: Per-Machine (requires Admin privileges).

    Default Path: C:\Program Files\SlintImageConverter\.

    Registry Keys: HKCU\Software\reset54\SlintImageConverter.
