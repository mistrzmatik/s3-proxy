FROM rust:1.82.0-alpine3.20 AS build
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN apk add --no-cache musl-dev openssl-dev
WORKDIR /app
COPY ./ /app
RUN cargo build --release
RUN strip target/release/s3_proxy

FROM alpine:3.20 AS runtime
LABEL org.opencontainers.image.source=https://github.com/mistrzmatik/s3-proxy
RUN adduser -S user
RUN apk add --no-cache libgcc
COPY --from=build --chown=user /app/target/release/s3_proxy .
EXPOSE 8080
USER user
ENTRYPOINT ["/s3_proxy"]