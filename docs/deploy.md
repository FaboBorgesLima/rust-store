## Deployment tutorial

create a .env file using [.env.example](./.env.example)

creeate a secrets dir with "db_root_password.txt"

run:
`docker compose -f docker-compose.deploy.yaml up --build`

the frontend will run at port 80 and communicates to the backend through the "http://rust-store:8080" url, so you will need a DNS pointing to this address.
