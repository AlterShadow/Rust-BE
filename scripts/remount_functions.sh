#!/bin/bash
CONFIG=$1
export PGPASSWORD=$(jq '.app_db.password' -r $CONFIG)
echo "DROP SCHEMA api CASCADE;" | cat - db/api.sql | tee | psql -h localhost -p 5433 -U meh mc2fi
