#!/bin/bash

export TARGET="./mysql-5.7.36-linux-glibc2.12-x86_64"
export BASEDIR="$TARGET/mysql"
# export DATADIR="$TARGET/mysql/data"
export DATADIR="../mysql/data"
export PORT=9797
export USER="root"

# mkdir $TARGET/mysql 2>/dev/null
# mkdir $TARGET/mysql/data 2>/dev/null
mkdir mysql 
mkdir mysql/data 2>/dev/null

cd ./mysql-5.7.36-linux-glibc2.12-x86_64
# initialize
./bin/mysqld_safe --datadir $DATADIR \
--initialize-insecure --user=root \
--defaults_files=/home/gb/installer/mysql/my57.cnf 

# start
./bin/mysqld_safe --datadir $DATADIR \
--defaults_files=/home/gb/mysql/my57.cnf 


chmod -R 750 ./mysql-5.7.36-linux-glibc2.12-x86_64
chmod -R 750 ./mysql


# ./bin/mysqld_safe --user=$USER \
# --basedir=$BASEDIR \
# --datadir=$DATADIR \
# --pid-file=$TARGET/var/run/mysql/mysql.pid \
# --skip-syslog \
# --log-error=$TARGET/var/log/mysql/mysql.err \
# --port=$PORT \
# --socket=$TARGET/var/run/mysqld/mysqld.sock \
# --ledir=$BASEDIR/bin \
# --mysqld=mysqld \
# --bind-address=localhost