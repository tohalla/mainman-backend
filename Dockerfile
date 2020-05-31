FROM rust:1

WORKDIR /usr/src/mainman
COPY . .

RUN cargo install --path .

EXPOSE 8080
