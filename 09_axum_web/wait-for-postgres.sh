#!/bin/bash

# Wait for PostgreSQL to be available
# Usage: wait-for-postgres.sh host port user password database command

set -e

host=$1
port=$2
user=$3
password=$4
database=$5
shift 5
cmd="$@"

until PGPASSWORD=$password psql -h "$host" -p "$port" -U "$user" -d "$database" -c '\q'; do
  >&2 echo "PostgreSQL is unavailable - sleeping"
  sleep 1
done

>&2 echo "PostgreSQL is up - executing command"
exec $cmd