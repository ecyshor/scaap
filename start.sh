#!/bin/sh

set -ex

echo "Initializing scaap"
if test -f "/etc/scaap/init-runtime.sh"; then
  echo "Running init script for scaap"
  /etc/scaap/init-runtime.sh
fi
echo "Starting scaap"
exec "$@"
