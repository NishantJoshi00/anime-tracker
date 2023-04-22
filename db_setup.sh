sudo -u postgres psql -c "CREATE USER sam WITH PASSWORD 'damn' SUPERUSER CREATEDB CREATEROLE INHERIT LOGIN;" 

psql -e -U postgres -c "CREATE DATABASE weeb"

echo DATABASE_URL=postgres://sam:damn@localhost/weeb > .env
