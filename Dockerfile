FROM rust:latest AS rust-build

WORKDIR /usr/src

RUN USER=root cargo new postman-backup
WORKDIR /usr/src/postman-backup
COPY ./Cargo.toml ./Cargo.lock ./
RUN cargo build --release
RUN rm -rf src
COPY ./src ./src
RUN cargo build --release




FROM debian:bookworm-slim

WORKDIR /app

COPY --from=rust-build /usr/src/postman-backup/target/release/postman-backup /usr/local/bin/postman-backup

CMD postman-backup