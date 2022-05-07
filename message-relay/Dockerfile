FROM rust:1.60

WORKDIR /usr/src/message-relay

COPY . .
RUN cargo install --path .
EXPOSE 5000

CMD ["message_relay"]