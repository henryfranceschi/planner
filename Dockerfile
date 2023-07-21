FROM rust:1.70.0

WORKDIR /app
COPY . .

RUN apt update \
    && apt install -y libpq-dev \
    && cargo install diesel_cli --no-default-features --features postgres

ENTRYPOINT ["./docker-entrypoint.sh"]
