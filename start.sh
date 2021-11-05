#!/bin/bash

export JAVA_HOME = ./jdk-11.0.13+8
export PATH = $PATH:$JAVA_HOME/bin

cd ./apache-tomcat-9.0.48

./bin/catalina.sh start; tail -fn 100 ./logs/catalina.out