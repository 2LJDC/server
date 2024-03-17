#!/bin/bash

git reset --hard HEAD    

git pull

#ip=$(ip a | grep eth0 | tail -n 1 | awk -F' ' '{ print $2 }' | awk -F'/' '{ print $1 }')

#sed s/127.0.0.1/$ip/g src/main.rs > src/main2.rs 

#mv src/main2.rs src/main.rs 

cargo build --release
