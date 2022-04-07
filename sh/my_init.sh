#!/bin/bash

## grzesiekb (c) 2021 
## gbylica@softq.pl, grzegorz.bylica@gmail.com
##

export TARGET="./mysql-5.7.36-linux-glibc2.12-x86_64"
export DATADIR="../mysql/data"
export MY_LNG="./share/english"
export USER="root"
export MySQL_INTERNAL_SCADA_LTS_LONG=`pwd`/mysql/log/logs.err


if [ -d "./mysql "]
  then 
    echo "The data for the internal mysql database is probably initialised"
    exit 0
fi

rm -R ./mysql
mkdir mysql 
mkdir mysql/data
mkdir mysql/log

touch $MySQL_INTERNAL_SCADA_LTS_LONG
echo $MySQL_INTERNAL_SCADA_LTS_LONG
echo $DATADIR

cd $TARGET

# initialize
./bin/mysqld --datadir $DATADIR \
--initialize-insecure --user=$USER \
--language=$MY_LNG \
--log-error=$MySQL_INTERNAL_SCADA_LTS_LONG \
--lc-messages=en_US

