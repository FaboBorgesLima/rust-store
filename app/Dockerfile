FROM rust:alpine3.19 as build
WORKDIR /app
COPY src /app/src
COPY Cargo.lock /app/Cargo.lock
COPY Cargo.toml /app/Cargo.toml
COPY build.sh /app/build.sh
ARG IS_DEV
RUN mkdir /build
RUN apk add openssl libressl-dev musl-dev
RUN sh build.sh

FROM scratch
WORKDIR /bin
ENV MARIADB_DATABASE="data_base"
ENV MARIADB_USER="user"
ENV MARIADB_PASSWORD="password"
ENV MARIADB_ADDRESS="db"
ENV MARIADB_PORT="3306"
EXPOSE 8080
COPY --from=build /build/app /bin/app
CMD [ "/bin/app" ]