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
