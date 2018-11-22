FROM rustlang/rust:nightly

RUN cargo install diesel_cli

RUN cargo install cargo-watch

WORKDIR /usr/app

EXPOSE 3000
