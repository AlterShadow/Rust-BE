#!/bin/bash -xe
cargo build --all --release
mkdir -p /home/mc2fi/mc2fi/target/release/ /home/mc2fi/mc2fi/log
cd target/release/ 
rsync -avizh mc2fi_auth mc2fi_user mc2fi_admin mc2fi_watcher mc2fi_asset_price /home/mc2fi/mc2fi/target/release/
rsync -avizh scripts db /home/mc2fi/
scp etc/config.prod.json /home/mc2fi/mc2fi/etc/config.json
rsync -avizh etc/systemd/*.service /etc/systemd/system/
sh scripts/remount_functions.sh etc/config.json
sh scripts/restart_services.sh
scripts/upload_docs.sh

