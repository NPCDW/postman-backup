FROM rust:latest AS rust-build

RUN mkdir /usr/src/postman-backup
WORKDIR /usr/src/postman-backup
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
RUN cargo build --release




FROM debian:bookworm-slim

WORKDIR /app
RUN apt-get update && apt-get install -y openssl ca-certificates
COPY --from=rust-build /usr/src/postman-backup/target/release/postman-backup /usr/local/bin/postman-backup
CMD postman-backup