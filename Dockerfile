FROM rust

RUN mkdir -p /home/server

COPY .env /home/server
COPY net /home/server
