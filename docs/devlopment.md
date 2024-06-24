# Development

- create .env using .env.example as example.
- create a secrets/db_root_password.txt with the root password.
- start containers with:
  `docker compose up`
- and for rebuild after every modification:
  `docker compose watch`

now, go ahead and start coding!

## Recomended configuration for local development

- backend:
  - Cargo (Rust package manager) 1.77
  - Rust 1.77
  - vscode extensions
- frontend:
  - vscode extensions
