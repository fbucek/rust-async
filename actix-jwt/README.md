# Actix JWT with SQLite

Source: [Build an API in Rust with JWT Authentication](https://auth0.com/blog/build-an-api-in-rust-with-jwt-authentication-using-actix-web/)

Sqlite choosed because of easier setup.

## How to run

### Devel 

`cargo watch -x "run --bin actix-jwt"` start server
`cargo watch -s "./check.sh"` -> will perform checking waiting for server
