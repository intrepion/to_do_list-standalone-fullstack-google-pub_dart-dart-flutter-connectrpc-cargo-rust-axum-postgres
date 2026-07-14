# to_do_list-standalone-fullstack-google-pub_dart-dart-flutter-connectrpc-cargo-rust-axum-postgres

To Do List written using Google OAuth 2.0 for the login, Dart + Flutter for the client, ConnectRPC API architecture, Rust + axum + connectrpc crate (https://crates.io/crates/connectrpc) as the server, and using Postgres as the storage.

## Database Setup

The application requires a PostgreSQL database. You have two options to set it up:

### Option 1: Local PostgreSQL (Recommended)

PostgreSQL is already configured locally with:
- **User**: `user`
- **Password**: `password`
- **Database**: `to_do_list`
- **Port**: `5432`

The database is ready to use with the connection string: `postgres://user:password@localhost:5432/to_do_list`

### Option 2: Docker (Alternative)

If you prefer using Docker, run:

```bash
# Start PostgreSQL container
docker-compose up -d postgres

# Stop PostgreSQL container
docker-compose down
```

The Docker container uses the same credentials:
- **User**: `user`
- **Password**: `password`
- **Database**: `to_do_list`
- **Port**: `5432` (mapped from container to host)
