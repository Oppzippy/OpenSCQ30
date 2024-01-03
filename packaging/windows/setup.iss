#define AppName "OpenSCQ30"
#define AppExeName "openscq30_gui.exe"
#define AppVersion "1.10.1"

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
SetupIconFile=..\..\gui\resources\com.oppzippy.OpenSCQ30.ico
UninstallDisplayIcon={app}\bin\{#AppExeName}
ArchitecturesAllowed=x64
ArchitecturesInstallIn64BitMode=x64
PrivilegesRequiredOverridesAllowed=dialog commandline

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "root\*"; DestDir: "{app}"; Flags: recursesubdirs

[Icons]
Name: "{group}\{#AppName}"; Filename: "{app}\bin\{#AppExeName}"
Name: "{autodesktop}\{#AppName}"; Filename: "{app}\bin\{#AppExeName}"; Tasks: desktopicon

[Run]
Filename: "{app}\bin\{#AppExeName}"; Description: "Launch program"; Flags: nowait postinstall skipifsilent
