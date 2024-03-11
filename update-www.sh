#!/bin/bash

if [ -d /var/www/update ]; then
	rm -Rf /var/www/update
fi

rm -Rf /var/www/*

mkdir /var/www/update

cd /var/www/update

git clone https://github.com/CasinoMLU/Website

mv -f /var/www/update/Website/* /var/www/

rm -Rf /var/www/update/Website

echo "Done"
