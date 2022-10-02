#!/bin/bash

# Source URL from .env file

export $(grep -v '^#' .env | xargs)

#  Run dbmaate with args

docker run --rm -it --network=host -v "$(pwd)/src/db:/db" --user "$(id -u):$(id -g)" \
ghcr.io/amacneil/dbmate:1 --url ${DATABASE_URL} \
"$@"
