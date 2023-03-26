#define AppName "OpenSCQ30"
#define AppExeName "openscq30_gui.exe"
#define AppVersion GetVersionNumbersString("..\..\target\release\openscq30_gui.exe")

[Setup]
AppName={#AppName}
AppVersion={#AppVersion}
VersionInfoVersion={#AppVersion}
AppCopyright=(C) Kyle Scheuing
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

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "..\..\target\release\{#AppExeName}"; DestDir: "{app}"
Source: "..\..\LICENSE.txt"; DestDir: "{app}"
Source: "..\..\gui\locales"; DestDir: "{app}"
Source: "dlls\*"; DestDir: "{app}"

[Icons]
Name: "{group}\{#AppName}"; Filename: "{app}\{#AppExeName}"
Name: "{autodesktop}\{#AppName}"; Filename: "{app}\{#AppExeName}"; Tasks: desktopicon

[Run]
Filename: "{app}\{#AppExeName}"; Description: "Launch program"; Flags: nowait postinstall skipifsilent
