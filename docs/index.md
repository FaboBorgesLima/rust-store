# Rust store

A simple store stock management system made using docker, rust, tailwind and vue.

## Requirements

- Docker engine 26.1

### Recomended configuration for local development

- backend:
  - Cargo (Rust package manager) 1.77
  - Rust 1.77
  - vscode extensions
- frontend:
  - vscode extensions

## Development tutorial

- create .env using .env.example as example.
- create a secrets/db_root_password.txt with the root password.
- start containers with:
  `docker compose up`
- and for rebuild after every modification:
  `docker compose watch`

now, go ahead and start coding!

## Deployment tutorial

create a .env file using [.env.example](../.env.example)

create a secrets dir with "db_root_password.txt"

run:
`docker compose -f docker-compose.deploy.yaml up --build`

the frontend will run at port 80 and communicates to the backend through the "http://rust-store:8080" url, so you will need a DNS pointing to this address.
