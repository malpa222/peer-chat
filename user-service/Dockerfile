FROM rust:1.60

WORKDIR /usr/src/user-service

COPY . .

# run migrations
RUN cargo install diesel_cli --no-default-features --features postgres && \
    diesel setup && \
    diesel migration run

RUN cargo install --path .
EXPOSE 5000

CMD ["user_service"]