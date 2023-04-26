#!/bin/sh

echo "starting docker service..."
sudo systemctl start docker || exit 1

echo "starting docker containers..."
sudo docker compose up --build
