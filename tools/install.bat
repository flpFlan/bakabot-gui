@echo off
call powershell /C iwr https://www.python.org/ftp/python/3.11.5/python-3.11.5-embed-amd64.zip -OutFile .\python311.zip
call powershell /C Expand-Archive -Path python311.zip -DestinationPath .\Python311
del /F /A .\python311.zip
call powershell /C (Invoke-WebRequest -Uri https://install.python-poetry.org -UseBasicParsing).Content | .\Python311\python -
set PATH=%PATH%;%APPDATA%\Python\Scripts
call .\restore

if %errorlevel%==0 goto poetry
echo Add-Type -AssemblyName PresentationFramework>messagebox.ps1
echo [System.Windows.MessageBox]::Show("�����ļ��޸�ʧ�ܣ�bot�����޷���������(���Գ�������restore�����޸�)", "����", "OK", "Error")>>messagebox.ps1
powershell -ExecutionPolicy Bypass -File messagebox.ps1
del messagebox.ps1
goto end

poetry:
cd .\bakabot
poetry config virtualenvs.path .\virtualenvs
poetry install --without dev --all-extras

end:
pause