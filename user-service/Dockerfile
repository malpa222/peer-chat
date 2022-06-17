FROM rust:1.60

WORKDIR /usr/src/user-service

COPY . .

# run migrations
RUN cargo install diesel_cli --no-default-features --features postgres
RUN chmod +x user-service

# build project
# RUN cargo build --release &&\ 
# cp target/release/user-service . &&\
# chmod +x docker_start.sh

EXPOSE 5000

CMD ["./user-service"]