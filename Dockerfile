FROM rust:1

WORKDIR /usr/mainman
COPY . .

RUN cargo install --path .

EXPOSE 8080
