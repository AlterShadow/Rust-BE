#!/bin/bash
CONFIG=$1
export PGHOST=$(jq '.app_db.host' -r $CONFIG)
export PGPORT=$(jq '.app_db.port' -r $CONFIG)
export PGUSER=$(jq '.app_db.user' -r $CONFIG)
export PGPASSWORD=$(jq '.app_db.password' -r $CONFIG)
export DATABASE=$(jq '.app_db.dbname' -r $CONFIG)
pg_exec() {
    echo "executing psql $@"
    psql "$@"
}
pg_exec2() {
    echo "executing psql $@"
    PGDATABASE=$DATABASE psql "$@"
}
# this is special, it will use mc2fi as user with -c option
#echo "DROP DATABASE IF EXISTS mc2fi WITH (FORCE); CREATE DATABASE mc2fi;" | pg_exec

pg_exec2 -c "DROP SCHEMA tbl CASCADE; DROP SCHEMA api CASCADE; DROP SCHEMA public CASCADE; CREATE SCHEMA public;"

pg_exec2 -f db/model.sql
# run twice because of wrong dependencies
pg_exec2 -f db/tbl.sql
pg_exec2 -f db/tbl.sql
pg_exec2 -f db/api.sql