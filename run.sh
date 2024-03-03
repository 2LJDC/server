#!/bin/bash

ip=$(ip a | grep eth0 | tail -n 1 | awk -F' ' '{ print $2 }' | awk -F'/' '{ print $1 }')

echo $ip

