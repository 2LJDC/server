#!/bin/bash

git clone https://github.com/CasinoMLU/Website

rm -Rf /var/www/*

mv Website/* /var/www/

rm -Rf Website/

echo "Done"
