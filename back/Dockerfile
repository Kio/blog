FROM rust:1.74

WORKDIR /app

RUN cargo install cargo-watch

COPY . .
RUN cargo build --release

EXPOSE 8000
CMD ["cargo", "watch", "-x", "run"]
