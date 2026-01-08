#!/usr/bin/env bash
set -euo pipefail

docker run --rm \
    --user $(id -u):$(id -g) \
    -v $PWD:/local openapitools/openapi-generator-cli generate \
    -i /local/firefly-iii-6.4.14-v1.yaml \
    -c /local/config.yaml \
    -g rust \
    -o /local
