#ifndef AppVersion
  #error AppVersion must be supplied by installer/build.ps1
#endif

[Setup]
; Never change AppId: upgrades and uninstall records depend on this value.
AppId={{0B426FE2-75FF-436B-BD46-08FD49A28D87}
AppName=Amategeko y'Umuhanda
AppVersion={#AppVersion}
AppPublisher=Rwanda Bruno
AppPublisherURL=https://amategeko-yumuhanda-alpha.vercel.app/
AppSupportURL=https://amategeko-yumuhanda-alpha.vercel.app/docs
AppUpdatesURL=https://github.com/brunorwanda4/amategeko-yumuhanda/releases
VersionInfoVersion={#AppVersion}
VersionInfoCompany=Rwanda Bruno
VersionInfoDescription=Amategeko y'Umuhanda Setup
VersionInfoProductName=Amategeko y'Umuhanda
DefaultDirName={localappdata}\Programs\Amategeko y'Umuhanda
DefaultGroupName=Amategeko y'Umuhanda
PrivilegesRequired=lowest
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
MinVersion=10.0
WizardStyle=modern dynamic
DisableWelcomePage=no
DisableDirPage=yes
DisableProgramGroupPage=yes
DisableReadyPage=no
DisableFinishedPage=no
UsePreviousTasks=yes
CloseApplications=yes
RestartApplications=no
SetupIconFile=..\assets\icon.ico
UninstallDisplayIcon={app}\amategeko.exe
OutputDir=output
OutputBaseFilename=amategeko-yumuhanda-{#AppVersion}-windows-setup
Compression=lzma2
SolidCompression=yes

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[CustomMessages]
DesktopShortcut=Create a desktop shortcut / Shyiraho ahanyuzwa kuri desktop
LaunchApp=Launch Amategeko y'Umuhanda / Fungura Amategeko y'Umuhanda
DeleteUserData=Also delete your saved progress and settings? / Usibe n'imibare n'igenamiterere byawe?

[Tasks]
Name: "desktopicon"; Description: "{cm:DesktopShortcut}"

[Files]
Source: "..\target\release\amategeko.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\Amategeko y'Umuhanda"; Filename: "{app}\amategeko.exe"
Name: "{userdesktop}\Amategeko y'Umuhanda"; Filename: "{app}\amategeko.exe"; Tasks: desktopicon; Check: ShouldCreateDesktopShortcut

[Run]
Filename: "{app}\amategeko.exe"; Description: "{cm:LaunchApp}"; Flags: nowait postinstall skipifsilent
Filename: "{app}\amategeko.exe"; Flags: nowait runasoriginaluser; Check: WizardSilent

[Code]
function ShouldCreateDesktopShortcut: Boolean;
begin
  Result := (not WizardSilent) or
    (not FileExists(ExpandConstant('{app}\unins000.exe'))) or
    FileExists(ExpandConstant('{userdesktop}\Amategeko y''Umuhanda.lnk'));
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var
  DataDir: String;
begin
  if (CurUninstallStep = usPostUninstall) and (not UninstallSilent) then
  begin
    if MsgBox(CustomMessage('DeleteUserData'), mbConfirmation,
      MB_YESNO or MB_DEFBUTTON2) = IDYES then
    begin
      DataDir := ExpandConstant('{userappdata}\amategeko\AmategekoYumuhanda\data');
      DelTree(DataDir, True, True, True);
    end;
  end;
end;
