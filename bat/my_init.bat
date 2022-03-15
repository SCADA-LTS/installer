@echo off

set TARGET="mysql-5.7.35-win64"
set CURRENT_DIR=%~dp0
set DATADIR="..\mysql\data"
set MY_LNG="share\english"
set USER="root"
set MySQL_INTERNAL_SCADA_LTS_LOG=%CURRENT_DIR%\mysql\log\logs.err

rmdir mysql /s /q
mkdir mysql
mkdir mysql/data

echo %MySQL_INTERNAL_SCADA_LTS_LOG%
echo %DATADIR%

cd %TARGET%

REM initialize

\bin\mysqld.exe --datadir %DATADIR% \
--initialize-insecure --user=%USER% \
--language=%MY_LNG% \
--log-error=%MySQL_INTERNAL_SCADA_LTS_LOG% \
--lc-messages=en_US

