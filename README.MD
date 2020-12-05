# SCAAP - script as a service
Run any script configured in the container by using a simple HTTP API

Useful for running adhoc scripts for monitoring, maintenance and so on.

## Usage

The service expects to find a toml file at `/etc/scaap/scaap.toml`.
During startup the file `/etc/scaap/init-runtime.sh` is executed and this can be used to prepare any extra runtime dependencies required by the scripts. (eg: install python, install required libraries for the script and so on)

For usage example check the docker-compose.yml file and the sample directory.

## Example usage:

https://nicu.dev/posts/raspberry-pi-temperature 

Read raspberry pi temperature by using prometheus.

Github repo: (https://github.com/ecyshor/pi-temperature-monitor)[https://github.com/ecyshor/pi-temperature-monitor]

## The container is build for multi architecture

We currently support amd64 and arm64 but more platforms can be easily added in the github actions workflow file.
