@echo off

cd mysql-shell-1.0.11-windows-x86-64bit
\bin\mysqlsh --uri root:@localhost:9797 --sql -e "create database scadalts"
