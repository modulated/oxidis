#!/usr/bin/env bash
set -x
set -eo pipefail

export DATABASE_URL="sqlite:data.db"
sqlx database create
sqlx migrate run