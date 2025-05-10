@echo off
REM Check if Git is already installed
git --version >nul 2>&1
IF %ERRORLEVEL% NEQ 0 (
    echo Git not found. Installing Git...
    REM Download and install Git
    set GIT_INSTALLER_URL=https://github.com/git-for-windows/git/releases/download/v2.42.0.windows.1/Git-2.42.0-64-bit.exe
    set GIT_INSTALLER=Git-Installer.exe

    REM Download the installer
    powershell -Command "Invoke-WebRequest -Uri %GIT_INSTALLER_URL% -OutFile %GIT_INSTALLER%"

    REM Run the installer (silent install)
    %GIT_INSTALLER% /VERYSILENT /NORESTART

    REM Clean up the installer
    del %GIT_INSTALLER%

    REM Check if Git was installed successfully
    git --version >nul 2>&1
    IF %ERRORLEVEL% NEQ 0 (
        echo Git installation failed.
        exit /b 1
    ) else (
        echo Git installed successfully.
    )
) else (
    echo Git is already installed.
)

REM Clone the repository
echo Cloning the repository...
git clone https://github.com/FoldFunc/GlobalInputCatcher.git

echo Done!
pause

