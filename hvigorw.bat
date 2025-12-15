@echo off
REM HarmonyOS hvigor wrapper script for Windows
REM This script will execute the hvigor build tool

set DEFAULT_HVIGOR_VERSION=4.0.0
set HVIGOR_APP_HOME=hvigor
set HVIGOR_WRAPPER_DIR=%HVIGOR_APP_HOME%\wrapper

if not exist "%HVIGOR_WRAPPER_DIR%" (
    echo ERROR: hvigor wrapper not found in %HVIGOR_WRAPPER_DIR%
    echo ERROR: Please install hvigor first
    exit /b 1
)

REM Execute hvigor
echo INFO: Executing hvigor...
node "%HVIGOR_WRAPPER_DIR%\hvigor.js" %*
