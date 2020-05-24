#!/bin/bash
LC_ALL=C cat <<EOF >./db/.env
POSTGRES_PASSWORD=$(</dev/urandom tr -dc 'A-Za-z0-9' | head -c 64)
POSTGRES_DB=hallussa
EOF

LC_ALL=C cat <<EOF >./.env
DB_PASSWORD_CLIENT=$(</dev/urandom tr -dc 'A-Za-z0-9' | head -c 64)
EOF
