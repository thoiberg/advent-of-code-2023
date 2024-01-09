FROM rust:1.75

RUN mkdir /app
WORKDIR /app

RUN apt-get update
RUN apt-get install -y less vim software-properties-common man gnupg2 zsh sudo

RUN curl -L --proto '=https' --tlsv1.2 \
    -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh \
    | bash

RUN rustup component add clippy rustfmt
RUN cargo binstall -y cargo-watch cargo-nextest