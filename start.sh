#!/bin/bash
export CURRENT_DIR=`pwd`
export JAVA_HOME=$CURRENT_DIR/jdk-11.0.13+8
export PATH=$PATH:$JAVA_HOME/bin
export CATALINA_HOME="`pwd`/apache-tomcat-9.0.48"
echo $PATH
echo $CATALINA_HOME

cd ./apache-tomcat-9.0.48

./bin/catalina.sh start; tail -fn 100 ./logs/catalina.out