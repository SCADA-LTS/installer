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

if netstat -an | grep ':8080' | grep -q -v '127.0.0.1\|::1'
  then 
    echo "the internal scadalts is running"
    exit 0
  else
    cd ./apache-tomcat-9.0.56
    ./bin/catalina.sh start; tail -fn 100 ./logs/catalina.out
fi
