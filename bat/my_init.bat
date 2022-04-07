@echo off

set TARGET=mysql-5.7.35-winx64
set CURRENT_DIR=%~dp0
set DATADIR=%CURRENT_DIR%\mysql\data
set MY_LNG=%CURRENT_DIR%\%TARGET%\share\english
set USER=root
set MySQL_INTERNAL_SCADA_LTS_LOG=%CURRENT_DIR%\mysql\log\logs.err

REM rmdir mysql /s /q
mkdir mysql
mkdir mysql\data
mkdir mysql\log

fsutil file createNew mysql\log\logs.err 0

echo %MySQL_INTERNAL_SCADA_LTS_LOG%
echo %DATADIR%
echo %MY_LNG%

cd %TARGET%

REM initialize

bin\mysqld.exe --datadir %DATADIR% --initialize-insecure --user=%USER% --language=%MY_LNG% --log-error=%MySQL_INTERNAL_SCADA_LTS_LOG% --lc-messages=en_US

