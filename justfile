export UID := shell("id -u")
export GID := shell("id -g")
export JUST_LIST_SUBMODULES := "true"
export ROOT_DIR := justfile_directory()

mod docker "docker/docker.just"

default:
  just --list

print:
  @echo "Dioxus CLI justfile"
  @echo "UID: ${UID}, GID: ${GID}"

build:
  docker compose run --rm web-builder 


