#!/bin/bash
set -e
# Usage ./deploy.sh
cargo build # prebuild to check for errors, debug to improve compile time
HEAD=$(git stash create)
if [ -z "$HEAD" ]; then
  HEAD=HEAD
fi
git ls-tree -r "$HEAD" --name-only | rsync -avizh --files-from=- ./ cv:mc2fi/
ssh cv 'cd mc2fi && cargo build --release'
ssh root@cv 'bash -s' < restart_services.sh

./upload_docs.sh
