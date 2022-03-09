#!/bin/bash
# after init and start include mysql
cd mysql-shell-8.0.27-linux-glibc2.12-x86-64bit
./bin/mysqlsh --uri root:@localhost:9797 --sql -e "create database scadalts"