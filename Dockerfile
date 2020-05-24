FROM rust:1

WORKDIR /usr/src/hallussa-backend
COPY . .

RUN cargo install --path .

EXPOSE 8080
