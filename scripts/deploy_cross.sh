#!/bin/bash
CROSS_TARGET=x86_64-unknown-linux-gnu
mv ~/.cargo/config ~/.cargo/config.back
cargo zigbuild --target=$CROSS_TARGET --release
COMPILE=$?
mv  ~/.cargo/config.back ~/.cargo/config
if [ $COMPILE -ne 0 ]
then
    exit $COMPILE
fi
set -e
(cd target/x86_64-unknown-linux-gnu/release/ && rsync -avizh auth user admin mc2fi:mc2fi/target/release/ )
ssh root@mc2fi 'bash -s' < restart_services.sh

./upload_docs.sh

