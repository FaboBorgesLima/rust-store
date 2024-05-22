FROM rust:alpine3.19 as build
WORKDIR /app
COPY src /app/src
COPY Cargo.toml /app/Cargo.toml
RUN cargo build -r

FROM alpine:3.19 
WORKDIR /bin
COPY --from=build /app/target/release/app /bin/app
CMD [ "/bin/app" ]