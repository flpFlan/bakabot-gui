@echo off
call powershell /C (Invoke-WebRequest -Uri https://install.python-poetry.org -UseBasicParsing).Content | .\Python311\python - --uninstall
del /F /A .\Python311
del /F /A .\bakabot
call .\uninstall
pause