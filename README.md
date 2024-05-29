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
