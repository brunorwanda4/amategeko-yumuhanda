#ifndef AppVersion
  #error AppVersion must be supplied by tools/package-windows.ps1
#endif

[Setup]
AppId={{4FD924B7-632B-4C52-9303-9B9562407F4F}
AppName=Amategeko y'Umuhanda
AppVersion={#AppVersion}
AppPublisher=Rwanda Bruno
DefaultDirName={localappdata}\Programs\AmategekoYumuhanda
DefaultGroupName=Amategeko y'Umuhanda
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
OutputDir=..\dist
OutputBaseFilename=amategeko-yumuhanda-{#AppVersion}-windows-setup
SetupIconFile=..\assets\icon.ico
UninstallDisplayIcon={app}\amategeko.exe
Compression=lzma2
SolidCompression=yes
CloseApplications=force
RestartApplications=no
WizardStyle=modern

[Files]
Source: "..\target\release\amategeko.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "installer-marker"; DestDir: "{app}"; DestName: "amategeko-installed.marker"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\Amategeko y'Umuhanda"; Filename: "{app}\amategeko.exe"

[Run]
Filename: "{app}\amategeko.exe"; Description: "Amategeko y'Umuhanda"; Flags: nowait runasoriginaluser
