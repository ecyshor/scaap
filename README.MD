# SCAAP - script as a service
Run any script configured in the container by using a simple HTTP API

Useful for running adhoc scripts for monitoring, maintenance and so on.

## Supported script runtimes
Currently supported:

- bash
- python

To support a new runtime just open a PR with the updated Dockerfile

## Usage

For usage example check the docker-compose.yml file and the sample directory.

## Example usage:

Read raspberry pi temperature by using prometheus.

## The container is build for multi architecture