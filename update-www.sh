#!/bin/bash

if [ -d /var/www/update ]; then
	rm -Rf /var/www/update
fi

cd /var/www/update

git clone https://github.com/CasinoMLU/Website

#rm -Rf /var/www/*

mv -f var/www/update/Website/* /var/www/

rm -Rf var/www/update/Website

echo "Done"
