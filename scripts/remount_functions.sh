#!/bin/bash
CONFIG=$1
export PGPASSWORD=$(jq '.app_db.password' -r $CONFIG)
export PGHOST=$(jq '.app_db.host' -r $CONFIG)
export PGPORT=$(jq '.app_db.port' -r $CONFIG)
export PGUSER=$(jq '.app_db.user' -r $CONFIG)
export PGPASSWORD=$(jq '.app_db.password' -r $CONFIG)
export PGDATABASE=$(jq '.app_db.dbname' -r $CONFIG)
echo "DROP SCHEMA api CASCADE;" | cat - db/api.sql | tee | psql
