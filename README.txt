# Run the schema file

psql -U netflix -d netflix -a -f db/schema.sql

# Run a migrations

psql -U netflix -d netflix -a -f db/migrations/<my_file>.sql

# For Global db connection pool

https://doc.rust-lang.org/beta/std/lazy/struct.SyncLazy.html
