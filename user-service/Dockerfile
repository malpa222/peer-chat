FROM rust:1.60

WORKDIR /usr/src/user-service

COPY . .
RUN cargo install --path .
EXPOSE 5000

CMD ["user_service"]