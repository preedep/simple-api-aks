#!/bin/bash
set -e

echo "Building linux binary..."
./build4linux.sh
status=$?

if [ $status -ne 0 ]; then
    echo "Failed to build linux binary"
    exit 1
fi
echo "Building docker image..."
docker rm -f simple-api-aks:latest
docker build --platform=linux/amd64 -t simple-api-aks:latest .
#docker build -t simple-api-aks:latest .

echo "Pushing to docker hub..."
docker tag simple-api-aks:latest nickmsft/simple-api-aks:latest
docker push nickmsft/simple-api-aks:latest
