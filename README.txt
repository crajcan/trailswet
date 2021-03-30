# View Docs

cargo doc --open

# Run migrations

## Install sqlx cli

```
cargo install sqlx-cli --no-default-features --features postgres
```

```
sqlx migrate run
```

# For Global db connection pool

https://doc.rust-lang.org/beta/std/lazy/struct.SyncLazy.html
