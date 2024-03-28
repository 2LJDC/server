#!/bin/bash

git pull
cargo build --release
rm -Rf www
git clone https://github.com/2LJDC/Website_2.0
mv Website_2.0 www
podman build -t server .
podman kill main-server; podman rm main-server
podman run -dt --name main-server -p 0.0.0.0:443:8000 server
