#!/bin/bash

if [ ! -d /tmp/website ]; then
	mkdir /tmp/website
else
	rm -Rf /tmp/website/*
fi

cd /tmp/website

git clone https://github.com/CasinoMLU/Website

rm -Rf /var/www/*

mv Website/* /var/www/

rm -Rf Website/

echo "Done"
