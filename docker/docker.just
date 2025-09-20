
[doc("Verify that UID and GID are set correctly")]
check:
  #!/bin/sh
  if [ -z "${UID}" ] || [ -z "${GID}" ]; then \
    echo "Error: UID or GID is not set."; \
    exit 1;
  fi
  echo "Check succesful"

build-builder: check
  docker compose build web-builder

build: build-builder
  #!/bin/bash
  set -e
  echo Creating directory ${ROOT_DIR}/build/web
  mkdir -p ${ROOT_DIR}/build/web
  docker compose run --rm web-builder
