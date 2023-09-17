@echo off
call powershell /C iwr https://www.python.org/ftp/python/3.11.5/python-3.11.5-embed-amd64.zip -OutFile .\python311.zip
if exist Python311 rd /s /q Python311
call powershell /C Expand-Archive -Path python311.zip -DestinationPath .\Python311
del /F /A .\python311.zip
call powershell /C (Invoke-WebRequest -Uri https://install.python-poetry.org -UseBasicParsing).Content | .\Python311\python -
call .\restore

if %errorlevel%==0 goto poetry
echo Add-Type -AssemblyName PresentationFramework>messagebox.ps1
echo [System.Windows.MessageBox]::Show("本地文件修复失败，bot可能无法正常运行(可以尝试运行restore进行修复)", "错误", "OK", "Error")>>messagebox.ps1
powershell -ExecutionPolicy Bypass -File messagebox.ps1
del messagebox.ps1
goto end

:poetry
cd .\bakabot
call %APPDATA%\Python\Scripts\poetry config virtualenvs.path .\virtualenvs
call %APPDATA%\Python\Scripts\poetry install --without dev --all-extras

:end
pause