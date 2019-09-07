#!/bin/bash

curl -v localhost:9000/setSession
echo "\n"
curl -v localhost:9000/setSession?name=Taro
echo "\n"
