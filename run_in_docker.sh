#!/bin/bash

docker-compose up -d
if [[ "$CLIENT_DEBUG" = "1" ]]; then
	export RUST_LOG=debug
fi
docker-compose run client
docker-compose down
