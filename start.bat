@echo off
set CURRENT_DIR=%~dp0
set JAVA_HOME=%CURRENT_DIR%/jdk-11.0.13+8
set PATH=%PATH%:%JAVA_HOME%/bin
echo %PATH%

cd ./apache-tomcat-9.0.48

./bin/catalina.bat start;