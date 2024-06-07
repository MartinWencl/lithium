#!/usr/bin/env bash

CONN_STR="sqlite:lithium.db"

export DATABASE_URL=sqlite:/home/martinw/projects/lithium/lithium.db

sqlx database drop -D "$CONN_STR"
sqlx database create -D "$CONN_STR"
sqlx migrate run -D "$CONN_STR"
