#!/bin/bash

if [ -d /tmp/Website ]; then
	rm -Rf /tmp/website/*
fi

cd /tmp

git clone https://github.com/CasinoMLU/Website

#rm -Rf /var/www/*

mv -f /tmp/Website/* /var/www/

rm -Rf /tmp/Website

echo "Done"
