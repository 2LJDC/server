#!/bin/bash

if [ -d /var/www/ ]; then
	rm -Rf /app/www/*
fi

if [ -d /var/www/graphics ]; then
	rm -Rf /app/www/graphics
fi

cd /app/www/

git clone https://github.com/2LJDCU/Website

mv -f /app/www/Website/* /app/www/

rm -Rf /app/www/Website

echo "Done"
