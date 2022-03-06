FROM rust:1.60

WORKDIR /usr/src/user_service

COPY . .
RUN cargo install --path .
EXPOSE 5001

CMD ["user_service"]