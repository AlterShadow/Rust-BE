#!/bin/bash -xe
CROSS_TARGET=x86_64-unknown-linux-gnu
cargo zigbuild --target=$CROSS_TARGET --release
ssh mc2fi 'mkdir -p mc2fi/target/release/ mc2fi/log'
(cd target/x86_64-unknown-linux-gnu/release/ && rsync -avizh auth user admin trade-watcher escrow-watcher mc2fi:mc2fi/target/release/ )
scp etc/config.prod.json mc2fi:mc2fi/etc/config.json
rsync -avizh --delete abi/. mc2fi:mc2fi/abi
rsync -avizh etc/systemd/*.service root@mc2fi:/etc/systemd/system/
ssh root@mc2fi 'bash -s' < scripts/restart_services.sh
scripts/remount_functions.sh etc/config.prodref.json
scripts/upload_docs.sh

