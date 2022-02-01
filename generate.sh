#!/usr/bin/env bash
set -euo pipefail

sudo docker run --rm \
    --user $(id -u):$(id -g) \
    -v $PWD:/local openapitools/openapi-generator-cli generate \
    -i /local/firefly-iii-1.5.5.yaml \
    -c /local/config.yaml \
    -g rust \
    -o /local
