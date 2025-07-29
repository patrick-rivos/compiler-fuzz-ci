#!/bin/bash

# Clean up existing ghcr.io/patrick-rivos/compiler-fuzz-ci images
docker rm $(docker ps -a -f status=exited -f ancestor=ghcr.io/patrick-rivos/compiler-fuzz-ci -q) || true
docker image ls --filter reference=ghcr.io/patrick-rivos/compiler-fuzz-ci --filter dangling=true --quiet | xargs docker rmi -f || true

docker pull ghcr.io/patrick-rivos/compiler-fuzz-ci:latest
