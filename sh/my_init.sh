#!/bin/bash

## grzesiekb (c) 2021 
## gbylica@softq.pl, grzegorz.bylica@gmail.com
##

export TARGET="./mysql-5.7.36-linux-glibc2.12-x86_64"
export DATADIR="../mysql/data"
export MY_LNG="./share/english"
export USER="root"


rm -R ./mysql
mkdir mysql 
mkdir mysql/data


cd $TARGET


# initialize
./bin/mysqld --datadir $DATADIR \
--initialize-insecure --user=$USER \
--language=$MY_LNG

