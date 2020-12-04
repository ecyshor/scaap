#!/bin/sh
set -ex
if test -f "/etc/scaap/init-runtime.sh"; then
  echo "Running init script for scaap"
  /etc/scaap/init-runtime.sh
fi
exec "$@"
