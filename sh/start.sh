#!/bin/bash
chmod +x ./my_init.sh my.sh cdb.sh
./my_init.sh &
sleep 5
./my.sh &
sleep 5
./cdb.sh &

export CURRENT_DIR=`pwd`
export JAVA_HOME=$CURRENT_DIR/jdk-11.0.13+8
export PATH=$PATH:$JAVA_HOME/bin
export CATALINA_HOME="`pwd`/apache-tomcat-9.0.56"
echo $PATH
echo $CATALINA_HOME

cd ./apache-tomcat-9.0.56

./bin/catalina.sh start; tail -fn 100 ./logs/catalina.out
