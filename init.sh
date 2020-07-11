#!/bin/bash
docker-compose build --no-cache
docker-compose up -d
docker exec -it database1_1 sh -c "psql -U database1_role database1 -c \"DROP SCHEMA public CASCADE;\""
docker exec -it database1_1 sh -c "psql -U database1_role database1 -c \"CREATE SCHEMA public;\""
docker exec -it database1_1 sh -c "psql -U database1_role database1 -c \"GRANT ALL ON SCHEMA public TO database1_role;\""
docker exec -it database1_1 sh -c "psql -U database1_role database1 -c \"GRANT ALL ON SCHEMA public TO public;\""
docker-compose down
