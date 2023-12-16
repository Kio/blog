FROM rust:1.74

WORKDIR /app

RUN cargo install cargo-watch sea-orm-cli

COPY . .

EXPOSE 8000
CMD ["cargo", "watch", "-x", "run", "-B", "1"]
