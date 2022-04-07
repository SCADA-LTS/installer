@echo off


set TARGET=mysql-5.7.35-winx64
set CURRENT_DIR=%~dp0
set DATADIR=%CURRENT_DIR%\mysql\data
set MY_LNG=%CURRENT_DIR%\%TARGET%\share\english
set USER=root
set MySQL_INTERNAL_SCADA_LTS_LOG=%CURRENT_DIR%\mysql\log\logs.err
set PORT=9797

echo %MySQL_INTERNAL_SCADA_LTS_LOG%
echo %DATADIR%
echo %MY_LNG%

cd %TARGET%

REM start

bin\mysqld.exe --datadir %DATADIR% --bind-address=localhost --port=%PORT% --language=%MY_LNG% --log-error=%MySQL_INTERNAL_SCADA_LTS_LOG% --lc-messages=en_US


