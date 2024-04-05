set -x
set -eo pipefail

CONTAINER_NAME="flights-local-db"
DB_USER="flights.user"
DB_PASSWORD="flights.pass"
DB_NAME="flights.db"

docker run \
-e POSTGRES_USER=${DB_USER} \
-e POSTGRES_PASSWORD=${DB_PASSWORD} \
-e POSTGRES_DB=${DB_NAME} \
-p 5432:5432 \
--name ${CONTAINER_NAME} \
-d postgres