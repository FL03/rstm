#!/bin/bash

# flags for the script:
# --no-link: don't create a symlink to the result in the current directory
# --no-build: don't build the package, just print the path to the result
# -c or --command: run the given command in the shell with the package available (default build)

[ "$1" = "--no-link" ] && no_link=true && shift
[ "$1" = "--no-build" ] && no_build=true && shift
if [ "$1" = "-c" ] || [ "$1" = "--command" ]; then
  shift
  nix --extra-experimental-features 'nix-command flakes' shell  .#rstm --command "$@" --allow-dirty
  exit $?
fi

nix --extra-experimental-features 'nix-command flakes' "$@" --allow-dirty
