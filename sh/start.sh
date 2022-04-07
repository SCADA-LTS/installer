#!/bin/bash
chmod +x ./my_init.sh my.sh cdb.sh
#init data for mysql
./my_init.sh &
#start mysql
sleep 5
./my.sh &


export CURRENT_DIR=`pwd`
export JAVA_HOME=$CURRENT_DIR/jdk-11.0.13+8
export PATH=$PATH:$JAVA_HOME/bin
export CATALINA_HOME="`pwd`/apache-tomcat-9.0.56"
echo $PATH
echo $CATALINA_HOME

export to_check=`netstat -an | grep ':8080' | wc -l`
# netstat -an | grep ':8080' | grep -q -v '127.0.0.1\|::1'

echo "number of open tomcat ports: $to_check"

if ((to_check == 0)); 
  then 
    #create db
    sleep 5
    ./cdb.sh &

    cd ./apache-tomcat-9.0.56
    ./bin/catalina.sh start &
    tail -fn 100 ./logs/catalina.out
    
  else
    echo "the internal scadalts is running"
    exit 0
    
fi
