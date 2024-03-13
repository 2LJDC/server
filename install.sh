#!/bin/bash

cp target/release/server /usr/bin/server

if [ ! -d /var/server_conf ]; then
	mkdir /var/server_conf
fi

cp configuration.yaml /var/server_conf/configuration.yaml

