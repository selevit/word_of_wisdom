#!/bin/bash
set -e
cargo build --release
killall server || true &>/dev/null
echo "Running TCP server"
./target/release/server &
server_pid=$!
set +e
sleep 0.1
echo "Running TCP client"
trap "kill $server_pid" SIGINT
./target/release/client
kill $server_pid
