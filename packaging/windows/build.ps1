$ErrorActionPreference = "Stop"
$root = "$PSScriptRoot\..\.."

cargo make --profile release --env CARGO_MAKE_WORKSPACE_INCLUDE_MEMBERS=gui --cwd $root build

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
    "$root\packaging\windows\root\bin"
    "$root\packaging\windows\root\share\locale"
    "$root\packaging\windows\root\share\glib-2.0\schemas"
)
$filesToCopyNoOverwrite = @{
    "$root\LICENSE.txt"                                     = "$root\packaging\windows\root";
    "C:\gtk-build\gtk\x64\release\bin\*.dll"                = "$root\packaging\windows\root\bin";
    "C:\gtk-build\gtk\x64\release\bin\gdbus.exe"            = "$root\packaging\windows\root\bin";
    "C:\gtk-build\gtk\x64\release\share\glib-2.0\schemas\*" = "$root\packaging\windows\root\share\glib-2.0\schemas";
    "C:\gtk-build\gtk\x64\release\share\locale\*"           = "$root\packaging\windows\root\share\locale";
}
$filesToCopy = @{
    "$root\target\release\openscq30_gui.exe" = "$root\packaging\windows\root\bin";
    "$root\target\release\share\*"           = "$root\packaging\windows\root\share";
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

& "C:\Program Files (x86)\Inno Setup 6\ISCC.exe" "$root\packaging\windows\setup.iss"
