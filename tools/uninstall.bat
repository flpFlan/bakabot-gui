@echo off
call powershell /C (Invoke-WebRequest -Uri https://install.python-poetry.org -UseBasicParsing).Content | .\Python311\python - --uninstall
rd /s /q Python311
rd /s /q bakabot
pause