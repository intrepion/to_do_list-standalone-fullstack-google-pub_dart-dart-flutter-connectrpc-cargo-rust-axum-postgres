# Protocol Buffer Definitions

This directory contains the `.proto` files for the application, defining the gRPC services and message types used by both the Rust server and Dart/Flutter client.

## Structure

```
proto/
├── buf.yaml              # Buf module configuration
├── auth/
│   └── v1/
│       └── auth.proto   # Authentication service definitions
└── todo/
    └── v1/
        └── todo.proto   # Todo service definitions
```

## Tools

This project uses [Buf](https://buf.build/) as the primary Protocol Buffer toolchain. Buf provides:

- **bufbuild/buf** - Modern Protocol Buffer compiler and linting tool
- **protoc** - Traditional Protocol Buffer compiler (used internally by buf)

## Installation

Install the Buf CLI:

```bash
# macOS (Homebrew)
brew install bufbuild/buf/buf

# Linux
curl -sSL https://github.com/bufbuild/buf/releases/latest/download/buf-$(uname -s)-$(uname -m) -o /usr/local/bin/buf
chmod +x /usr/local/bin/buf

# Verify installation
buf --version
```

## Code Generation

To generate Rust and Dart code from the proto definitions:

```bash
# Install generation plugins
buf plugin install

# Generate all code (Rust server + Dart client)
make gen

# Or directly with buf
buf generate --template buf.gen.yaml proto
```

### Generated Output

- **Rust**: `server/src/gen/` - Contains generated `.rs` files using prost
- **Dart**: `client/lib/src/gen/` - Contains generated Dart files for protobuf and ConnectRPC

## Services

### AuthService (auth/v1/auth.proto)

Handles Google OAuth 2.0 authentication:
- `Authenticate` - Exchange Google token for JWT

### TodoService (todo/v1/todo.proto)

Handles todo CRUD operations:
- `CreateTodo` - Create a new todo
- `GetTodo` - Get a single todo by ID
- `ListTodos` - List all todos for a user
- `UpdateTodo` - Update an existing todo
- `DeleteTodo` - Delete a todo

## Adding New Proto Files

1. Create a new `.proto` file in the appropriate package directory (e.g., `proto/new/v1/new.proto`)
2. Define your messages and services
3. Update the `buf.yaml` if adding new dependencies
4. Run `buf generate` to regenerate code
5. Commit both the `.proto` file and the generated code

## Linting

Buf provides built-in linting. To check your proto files:

```bash
buf lint proto
```

## Breaking Changes

Buf can detect breaking changes. To check for breaking changes:

```bash
buf breaking --against .git#branch=main proto
```
