#!/bin/bash

diesel migration run --database-url postgres://postgres:$POSTGRES_PASSWORD@db:5432/$POSTGRES_DB
