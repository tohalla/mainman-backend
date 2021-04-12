#!/bin/bash
LC_ALL=C cat <<EOF >./db/.env
POSTGRES_PASSWORD=$(</dev/urandom tr -dc 'A-Za-z0-9' | head -c 64)
POSTGRES_DB=mainman
EOF

LC_ALL=C cat <<EOF >./.env
DB_PASSWORD_CLIENT=$(</dev/urandom tr -dc 'A-Za-z0-9' | head -c 64)
JWT_KEY=$(</dev/urandom tr -dc 'A-Za-z0-9' | head -c 64)
SESSION_KEY=$(</dev/urandom tr -dc 'A-Za-z0-9' | head -c 64)

STRIPE_SECRET=
SMTP_USERNAME=
SMTP_PASSWORD=
EOF
