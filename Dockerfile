FROM rust:latest as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/rust_quotes_api
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/rust_quotes_api /usr/local/bin/rust_quotes_api

CMD ["cargo", "run"]
