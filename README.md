# Diesel Utilities

This crate provides utilities to work with [Diesel ORM](<(https://diesel.rs/)>).

## Example

```rust
use bs_diesel_utils::Executor;
use tokio::main;

[tokio::main]
async main() {
  let db = Executor::new("postgres://postgres:postgres@localhost/bs").into_ref();
  let res = db.exec(|conn| {
    // blocking diesel calls here
    Ok(())
  }).await.unwrap();
}
```
