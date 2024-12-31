# Diesel

[guide for postgresql](https://diesel.rs/guides/getting-started)

## Interact with postgresql

- Add changes to the table / new table to the `src/database/schema.rs` file
- run `diesel migration generate --diff-schema <migration_name>`
- run `diesel migration run`

## postgresql CLI

- `\dt <table>` shows table info
- `select * from <table>` shows table contents
