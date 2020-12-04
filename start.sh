#!/bin/bash
set -ex
if test -f "/etc/scaap/init-runtime.sh"; then
  /etc/scaap/init-runtime.sh
fi
exec "$@"