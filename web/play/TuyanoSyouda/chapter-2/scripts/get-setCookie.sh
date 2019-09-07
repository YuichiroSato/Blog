#!/bin/bash

curl -v localhost:9000/setCookie
echo "\n"
curl -v localhost:9000/setCookie?name=Taro
echo "\n"
curl -v localhost:9000/setCookie?name=Siro
echo "\n"
