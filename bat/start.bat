@echo off
set CURRENT_DIR=%~dp0
set JAVA_HOME=%CURRENT_DIR%/jdk-11.0.13+8
set CATALINA_HOME=./apache-tomcat-9.0.56
set PATH=%PATH%:%JAVA_HOME%:%JAVA_HOME%/bin:%CATALINA_HOME%
echo %PATH%

cd ./apache-tomcat-9.0.56

./bin/catalina.bat start;
