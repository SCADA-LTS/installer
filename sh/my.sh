#!/bin/bash

## grzesiekb (c) 2021 
## gbylica@softq.pl, grzegorz.bylica@gmail.com
##


export TARGET="./mysql-5.7.36-linux-glibc2.12-x86_64"
export DATADIR="../mysql/data"
export PORT=9797
export USER="root"

chmod -R 750 $TARGET
chmod -R 750 ./mysql

cd $TARGET

# start
./bin/mysqld --datadir $DATADIR \
 --bind-address=localhost \
 --port=$PORT 
