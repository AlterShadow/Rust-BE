#!/bin/bash

export PGHOST=$(jq '.app_db.host' -r etc/config.json)
export PGPORT=$(jq '.app_db.port' -r etc/config.json)
export PGUSER=$(jq '.app_db.user' -r etc/config.json)
export PGPASSWORD=$(jq '.app_db.password' -r etc/config.json)
export DATABASE=$(jq '.app_db.dbname' -r etc/config.json)
pg_exec() {
    echo "executing psql $@"
    psql $@
}
pg_exec2() {
    echo "executing psql $@"
    PGDATABASE=$DATABASE psql $@
}
# this is special, it will use mc2fi as user with -c option
echo "CREATE DATABASE mc2fi;" | pg_exec

pg_exec2 -f db/model.sql
# run twice because of wrong dependencies
pg_exec2 -f db/tbl.sql
pg_exec2 -f db/tbl.sql
pg_exec2 -f db/api.sql
