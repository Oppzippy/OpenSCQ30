$ErrorActionPreference = "Stop"

cargo build --package openscq30_gui --release
.\scripts\build-locales.sh

function Write-Filtered-Errors {
    param (
        $errors
    )
    $errors | ForEach-Object {
        if ($_ -notmatch "already exists") {
            Write-Error $_
        }
    }
}

$directoriesToCreate = @(
    "ci\windows\root\bin"
    "ci\windows\root\share\locale"
    "ci\windows\root\share\glib-2.0\schemas"
)
$filesToCopyNoOverwrite = @{
    ".\LICENSE.txt"                                         = ".\ci\windows\root";
    "C:\gtk-build\gtk\x64\release\bin\*.dll"                = ".\ci\windows\root\bin";
    "C:\gtk-build\gtk\x64\release\bin\gdbus.exe"            = ".\ci\windows\root\bin";
    "C:\gtk-build\gtk\x64\release\share\glib-2.0\schemas\*" = ".\ci\windows\root\share\glib-2.0\schemas";
    "C:\gtk-build\gtk\x64\release\share\locale\*"           = ".\ci\windows\root\share\locale";
}
$filesToCopy = @{
    ".\target\release\openscq30_gui.exe" = ".\ci\windows\root\bin";
    ".\gui\locale\*"                     = ".\ci\windows\root\share\locale";
}

foreach ($directory in $directoriesToCreate) {
    New-Item -Type Directory -Path $directory -ErrorVariable errors -ErrorAction SilentlyContinue
    Write-Filtered-Errors $errors
}
foreach ($source in $filesToCopy.Keys) {
    Copy-Item -Recurse -Force -Path $source -Destination $filesToCopy[$source]
}
foreach ($source in $filesToCopyNoOverwrite.Keys) {
    Copy-Item -Recurse -Path $source -Destination $filesToCopyNoOverwrite[$source] -ErrorVariable errors -ErrorAction SilentlyContinue
    Write-Filtered-Errors $errors
}

& "C:\Program Files (x86)\Inno Setup 6\ISCC.exe" ci\windows\setup.iss
