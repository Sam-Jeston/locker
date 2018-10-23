docker run \
  -p 5432:5432 \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD="postgres" \
  -e POSTGRES_DB=locker \
  --name "postgres" \
  -d postgres:9.5
  --rm
