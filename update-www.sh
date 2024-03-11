#!/bin/bash

if [ -d /var/www/ ]; then
	rm -Rf /var/www/*
fi

cd /var/www/

git clone https://github.com/CasinoMLU/Website

mv -f /var/www/Website/* /var/www/

rm -Rf /var/www/Website

echo "Done"
