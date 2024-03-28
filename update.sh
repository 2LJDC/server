#!/bin/bash

git pull
cargo build --release
podman build -t server .
podman kill main-server; podman rm main-server
podman run -dt --name main-server -p 0.0.0.0:443:8000 server
