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
    ".\packaging\windows\root\bin"
    ".\packaging\windows\root\share\locale"
    ".\packaging\windows\root\share\glib-2.0\schemas"
)
$filesToCopyNoOverwrite = @{
    ".\LICENSE.txt"                                         = ".\packaging\windows\root";
    "C:\gtk-build\gtk\x64\release\bin\*.dll"                = ".\packaging\windows\root\bin";
    "C:\gtk-build\gtk\x64\release\bin\gdbus.exe"            = ".\packaging\windows\root\bin";
    "C:\gtk-build\gtk\x64\release\share\glib-2.0\schemas\*" = ".\packaging\windows\root\share\glib-2.0\schemas";
    "C:\gtk-build\gtk\x64\release\share\locale\*"           = ".\packaging\windows\root\share\locale";
}
$filesToCopy = @{
    ".\target\release\openscq30_gui.exe" = ".\packaging\windows\root\bin";
    ".\gui\locale\*"                     = ".\packaging\windows\root\share\locale";
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

& "C:\Program Files (x86)\Inno Setup 6\ISCC.exe" .\packaging\windows\setup.iss
