#define AppName "OpenSCQ30"
#define AppExeName "openscq30_gui.exe"
#define AppVersion "0.0.1"

[Setup]
AppName={#AppName}
AppVersion={#AppVersion}
AppCopyright=GPL=3.0
AppPublisher=Oppzippy
AppPublisherURL=https://github.com/Oppzippy/OpenSCQ30
OutputBaseFilename=OpenSCQ30_Setup
WizardStyle=modern
DefaultDirName={autopf}\{#AppName}
DefaultGroupName={#AppName}
DisableProgramGroupPage=auto
SetupIconFile=..\..\gui\resources\icon.ico
UninstallDisplayIcon={app}\{#AppExeName}
ArchitecturesAllowed=x64
ArchitecturesInstallIn64BitMode=x64
PrivilegesRequiredOverridesAllowed=dialog commandline
AllowNoIcons=yes

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "..\..\target\release\{#AppExeName}"; DestDir: "{app}"
Source: "..\..\LICENSE.txt"; DestDir: "{app}"
Source: "dlls\*.dll"; DestDir: "{app}"
; TODO gtk dlls

[Icons]
Name: "{group}\{#AppName}"; Filename: "{app}\{#AppExeName}"
Name: "{autodesktop}\{#AppName}"; Filename: "{app}\{#AppExeName}"; Tasks: desktopicon

[Run]
Filename: "{app}\{#AppExeName}"; Description: "Launch program"; Flags: nowait postinstall skipifsilent
