#!/bin/bash
set -e
cargo build --release
echo "Running TCP server"
./target/release/server &
server_pid=$!
set +e
echo "Running TCP client"
trap "kill $server_pid" SIGINT
./target/release/client
kill $server_pid