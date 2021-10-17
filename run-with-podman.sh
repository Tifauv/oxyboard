#!/bin/bash

# Container name & tag
NAME=oxyboard
TAG=0.1.0-2.1
CONTAINER="${NAME}:${TAG}"

# Host directory for the /app/data Volume
DATA_DIR=./data-run


# Build the container
#sudo podman build -t "${CONTAINER}" .
podman build -t "${CONTAINER}" .

# Run the container
mkdir -p "${DATA_DIR}"
#sudo podman run -d --name ${NAME} -v "${DATA_DIR}:/app/data" -p 8080:8080 "${CONTAINER}"
podman run -d --name ${NAME} -v "${DATA_DIR}:/app/data" -p 8080:8080 "${CONTAINER}"
