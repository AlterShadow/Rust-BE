#!/bin/bash -xe
CROSS_TARGET=x86_64-unknown-linux-gnu
cargo zigbuild --target=$CROSS_TARGET --all --release
ssh mc2fi 'mkdir -p mc2fi/target/release/ mc2fi/log'
(cd target/x86_64-unknown-linux-gnu/release/ && rsync -avizh mc2fi_auth mc2fi_user mc2fi_admin mc2fi_watcher mc2fi:mc2fi/target/release/ )
rsync -avizh scripts db mc2fi:mc2fi/
scp etc/config.prod.json mc2fi:mc2fi/etc/config.json
rsync -avizh etc/systemd/*.service root@mc2fi:/etc/systemd/system/
ssh mc2fi '(cd mc2fi && scripts/remount_functions.sh etc/config.json)'
ssh root@mc2fi 'bash -s' < scripts/restart_services.sh
scripts/upload_docs.sh

