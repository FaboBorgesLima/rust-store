FROM mariadb:11.4-rc-jammy

ENV MARIADB_DATABASE="data_base"
ENV MARIADB_USER="user"
ENV MARIADB_PASSWORD_FILE="/run/secrets/db_password"
ENV MARIADB_ROOT_PASSWORD="12345678"
ENV MARIADB_SOCKET_ADDRESS="db"

COPY init.sql /docker-entrypoint-initdb.d/init.sql