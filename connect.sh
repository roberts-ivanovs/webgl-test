#!/bin/bash

if [ $1 = '--django' ];
then
    echo "Connecting to djangoapp"
    docker-compose exec djangoapp bash
elif [ $1 = '--db' ];
then
    echo "Connecting to postgres"
    docker-compose exec database1 sh -c "psql -U database1_role database1"
elif [ $1 = '--node' ];
then
    echo "Connecting to node"
    docker-compose exec node sh
elif [ $1 = '--nginx' ];
then
    echo "Connecting to nginx"
    docker-compose exec nginx bash
else
    echo "starting development mode"
    docker-compose -f docker-compose.yml up
fi;

