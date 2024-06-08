# Rust store

A simple store stock management system made using docker, rust, tailwind and vue.

### Requirements for local development

-   Docker engine 26.1
-   Node 20 and npm

### Recomended configuration for local development

-   backend:
    -   Cargo (Rust package manager) 1.77
    -   Rust 1.77
    -   vscode extensions
-   frontend:
    -   vscode extensions

## Run dev tutorial

-   backend:
    -   create .env using .env.example as example.
    -   create a secrets/db_password.txt with the db password.
    -   start using:
        `docker compose up`
-   frontend:
    -   enter in frontend folder.
    -   run:
        `npm run dev`

# Deployment requirements

-   Docker engine 26.1

## Run deployment tutorial

create a .env file using [.env.example](./.env.example)

creeate a secrets dir with "db_password.txt" and "db_root_password.txt"

run:
`docker compose -f docker-compose.deploy.yaml up --build`

the frontend will run at port 80 and communicates to the backend through the "http://rust-store:8080" url, so you will need a DNS pointing to this address.
