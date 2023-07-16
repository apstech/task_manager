FROM rust:latest
MAINTAINER ale <apstech.info@gmail.com>

WORKDIR /usr/src/myapp

COPY . .

RUN cargo install --path .

CMD ["myapp"]