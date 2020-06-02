FROM rust:1

WORKDIR /migrate
COPY ./scripts ./scripts
RUN chmod +x ./scripts/*

RUN cargo install diesel_cli --no-default-features --features postgres

CMD ./scripts/up.sh
