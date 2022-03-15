@echo off

set TARGET="mysql-5.7.35-win64"
set CURRENT_DIR=%~dp0
set DATADIR="..\mysql\data"
set MY_LNG="share\english"
set USER="root"
set MySQL_INTERNAL_SCADA_LTS_LOG=%CURRENT_DIR%\mysql\log\logs.err
set PORT=9797

echo %MySQL_INTERNAL_SCADA_LTS_LOG%
echo %DATADIR%

REM start

/bin/mysqld --datadir %DATADIR% \
--bind-address=localhost \
--port=%PORT% \
--language=%MY_LNG% \
--log-error=%MySQL_INTERNAL_SCADA_LTS_LOG% \
--lc-messages=en_US


