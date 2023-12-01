FROM rust:1.74

WORKDIR /usr/src/blog
COPY . .

RUN cargo install --path .
EXPOSE 8000

CMD ["back"]
