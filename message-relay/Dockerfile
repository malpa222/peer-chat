FROM rust:1.60

WORKDIR /usr/src/message-relay

COPY . .

RUN chmod +x message-relay

EXPOSE 5001

CMD ["./message-relay"]