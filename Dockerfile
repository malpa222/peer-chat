FROM rust

RUN mkdir -p /home/serv

COPY ./src serv
COPY ./.env serv