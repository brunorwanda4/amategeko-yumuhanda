#define MyAppVersion "1.0.0"
#ifndef MyAppBinary
#define MyAppBinary "target\release\amategeko.exe"
#endif

[Setup]
AppId={{C0B9A8BF-9D6D-43D5-B4FD-9241E33F9D3B}
AppName=Amategeko y'Umuhanda
AppVersion={#MyAppVersion}
AppPublisher=Rwanda Bruno
DefaultDirName={autopf}\Amategeko
DefaultGroupName=Amategeko y'Umuhanda
PrivilegesRequired=lowest
OutputDir=dist
OutputBaseFilename=Amategeko-Setup
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
SetupIconFile=assets\icon.ico
UninstallDisplayIcon={app}\amategeko.exe

[Tasks]
Name: "desktopicon"; Description: "Create a desktop shortcut"; GroupDescription: "Additional shortcuts:"; Flags: unchecked

[Files]
Source: "{#MyAppBinary}"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\Amategeko y'Umuhanda"; Filename: "{app}\amategeko.exe"
Name: "{autodesktop}\Amategeko y'Umuhanda"; Filename: "{app}\amategeko.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\amategeko.exe"; Description: "Launch Amategeko y'Umuhanda"; Flags: nowait postinstall skipifsilent
