#!/bin/bash

## grzesiekb (c) 2021 
## gbylica@softq.pl, grzegorz.bylica@gmail.com
##


export TARGET="./mysql-5.7.36-linux-glibc2.12-x86_64"
export DATADIR="../mysql/data"
export PORT=9797
export USER="root"
export MySQL_INTERNAL_SCADA_LTS_LONG=`pwd`/mysql/log/logs.err
export MY_LNG="./share/english"

if netstat -an | grep ':9797' | grep -q -v '127.0.0.1\|::1'
  then 
    echo "the internal mysql is running"
    exit 0
fi

chmod -R 766 $TARGET
chmod -R 766 ./mysql

echo $MySQL_INTERNAL_SCADA_LTS_LONG
echo $DATADIR

cd $TARGET

# start
./bin/mysqld --datadir $DATADIR \
 --bind-address=localhost \
 --port=$PORT \
 --language=$MY_LNG \
 --log-error=$MySQL_INTERNAL_SCADA_LTS_LONG \
 --lc-messages=en_US