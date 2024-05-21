# Initialise the build environment
FROM rust:alpine as builder
WORKDIR /usr/src/service-listener
COPY . .

# Do stuff...
RUN set -x && \
    # Install the required dependencies
    apk add --no-cache musl-dev perl make && \
    # Build the application with OpenSSL statically linked
    cargo add openssl --features vendored && \
    cargo install --path .

# Create the final image
FROM alpine:latest
COPY --from=builder /usr/local/cargo/bin/service-listener /usr/local/bin/service-listener
ENTRYPOINT ["service-listener"]